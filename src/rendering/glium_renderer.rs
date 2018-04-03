use gg::rendering::Renderer;
use gg::rendering::primitives::text::{TextBuffer, PlainText};
use ::rendering::primitives::polar_pixel::{PolarBuffer};
use ::rendering::{PolarPrimitive, PolarRenderable};
use gg::rendering::WindowSpec;
use gg::rendering::DisplaySettings;
use gg::rendering::glium_buffer::{GliumBuffer};
use glium;
use glium::{Display, Surface, DrawParameters, Depth, DepthTest};
use glium::texture;
use glium::glutin::EventsLoop;
use na;
use na::Matrix4;
use num::One;
use gg::games::view_details;
use gg::utils::transforms_2d;
use gg::debug::*;

pub struct GliumRenderer<'a> {
    display: Box<Display>,
    events_loop: Box<EventsLoop>,
    draw_params: DrawParameters<'a>,
    polar_buffer: PolarBuffer,
    text_processor: TextBuffer<'a, PlainText>,
    view_details: view_details::ViewDetails,
    display_settings: DisplaySettings,
    texture_array: texture::srgb_texture2d_array::SrgbTexture2dArray
}

impl<'a> GliumRenderer<'a> {
    pub fn new(settings: DisplaySettings) -> GliumRenderer<'a> {
        let (display, events_loop) = Self::build_display_and_events_loop(settings);

        let draw_params = DrawParameters {
            depth: Depth {
                test: DepthTest::IfLessOrEqual,
                write: true,..Default::default()
            },
            ..Default::default()
        };

        GliumRenderer {
            display: Box::new(display.clone()),
            events_loop: Box::new(events_loop),
            draw_params: draw_params,
            polar_buffer: PolarBuffer::new(&display),
            text_processor: TextBuffer::new(&display, settings),
            view_details: view_details::ViewDetails::TwoDim(view_details::ViewDetails2D::default()),
            display_settings: settings,
            texture_array: texture::srgb_texture2d_array::SrgbTexture2dArray::empty(&display, 1024, 1024, 1).unwrap()
        }
    }

    pub fn reset(&mut self, settings: DisplaySettings) {
        let window = Self::build_window(settings, &self.events_loop);
        
        let context = Self::build_context(settings);

        self.display.rebuild(window, context, &self.events_loop).unwrap();
    }

    fn reset_buffers(&mut self) {
        let display = &self.display;
        self.polar_buffer = PolarBuffer::new(display);
        self.text_processor = TextBuffer::new(display, self.display_settings);
    }

    fn build_display_and_events_loop(settings: DisplaySettings) -> (Display, EventsLoop) {
        let events_loop = glium::glutin::EventsLoop::new();
        
        let window = Self::build_window(settings, &events_loop);
        
        let context = Self::build_context(settings);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        (display, events_loop)
    }

    fn build_window(settings: DisplaySettings, events_loop: &glium::glutin::EventsLoop) -> glium::glutin::WindowBuilder {
        let mut window = glium::glutin::WindowBuilder::new()
            .with_dimensions(settings.res.0, settings.res.1);

        if settings.fullscreen { 
            window = window.with_fullscreen(Some(events_loop.get_primary_monitor())); 
        }

        window
    }

    fn build_context(settings: DisplaySettings) -> glium::glutin::ContextBuilder<'a> {
        glium::glutin::ContextBuilder::new().with_multisampling(settings.multisample_level)
    }

    fn flush_buffers(&mut self) {
        self.polar_buffer.flush_buffer();
        self.text_processor.flush_buffer();
    }
    
    pub fn create_worldview_mat(view_details: view_details::ViewDetails, aspect_ratio: f64) ->  [[f32;4]; 4] {
        let view_mat = match
            view_details {
                view_details::ViewDetails::TwoDim(ref view) =>
                transforms_2d::build_worldview_mat(
                    view.camera_pos,
                    view.viewport_height,
                    view.viewport_length,
                    aspect_ratio,
                    view.up_vector,
                    view.use_aspect_ratio),
                view_details::ViewDetails::ThreeDim(_) => panic!("3D mode not supported!"),
                _ => Matrix4::one()
            };
        let single_mat: Matrix4<f32> = na::convert(view_mat);
        *single_mat.as_ref()
    }
}

impl<'a> Renderer for GliumRenderer<'a> {
    type Primitive = PolarPrimitive;

    fn load_renderables(&mut self, renderables: Vec<Box<PolarRenderable>>) {
        debug_clock_start("Render::glium_load");
        for mut renderable in renderables {
            for primitive in renderable.get_primitives() {
                match primitive {
                        PolarPrimitive::Text(text) => self.text_processor.load_renderable(text),
                        PolarPrimitive::PolarPix(polar) => self.polar_buffer.load_renderable(polar),
                }
            }
        }
        debug_clock_stop("Render::glium_load");
    }

    fn render(&mut self) {
        debug_clock_start("Render::glium_render");
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.clear_depth(1.0);

        let (width, height) = target.get_dimensions();
        let aspect_ratio = width as f64 / height as f64;
        
        {
            let uniforms = uniform! {
                screen_width: width,
                screen_height: height,
                aspect_ratio: aspect_ratio as f32,
                world_view: GliumRenderer::create_worldview_mat(self.view_details, aspect_ratio),
                tex: &self.texture_array
            };
            
            self.polar_buffer.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            self.text_processor.draw_at_target(&mut target, &self.display, self.view_details, &self.draw_params, &uniforms);
            
            target.finish().unwrap();
        }
        
        self.flush_buffers();
        debug_clock_stop("Render::glium_render");
    }

    fn set_worldview(&mut self, view_details: view_details::ViewDetails) {
        self.view_details = view_details;
    }

    fn get_events_loop(&mut self) -> Option<&mut EventsLoop> {
        Some(&mut self.events_loop)
    }

    fn get_window_spec(&self) -> WindowSpec {
        let (width, height) = self.display.gl_window().get_inner_size().unwrap();

        WindowSpec {
            aspect_ratio: width as f64 / height as f64
        }
    }
}
