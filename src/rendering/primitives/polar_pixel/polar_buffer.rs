use na::{Vector2, Vector4, Matrix2};
use glium;
use glium::{Display, Surface, Frame, DrawParameters, Blend};
use glium::index::PrimitiveType;
use super::{PolarPixel, PolarPixelVertex};
use gg::rendering;
use gg::rendering::shaders;
use gg::rendering::glium_buffer::GliumBuffer;
use gg::rendering::render_by_shaders::GliumStandardPrimitive;
use gg::games::view_details;
use gg::games::view_details::{ViewDetails, PolarViewDetails};
use gg::rendering::shaders::make_program_from_shaders;

pub struct PolarBuffer {
    vertices: Vec<PolarPixelVertex>,
    program: glium::Program,
    primitive_type: PrimitiveType
}

impl GliumBuffer<PolarPixel> for PolarBuffer {
    fn draw_at_target<Unif: glium::uniforms::Uniforms> (
        &mut self,
        target: &mut Frame,
        display: &Display,
        view_details: ViewDetails,
        _: &DrawParameters,
        _: &Unif,
    ) {
        if !self.vertices.is_empty() {
            let polar_view: PolarViewDetails = match view_details {
                ViewDetails::Polar(pol_view) => pol_view,
                _ => panic!("Must use PolarViewDetails with polar pixel rendering elements!")
            };
            
            let vertex_buffer = glium::VertexBuffer::new(display, &self.vertices).unwrap();

            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f64 / height as f64;

            let uniforms = uniform! {
                rotation_angle: polar_view.rotation_angle as f32,
                radial_shift: polar_view.radial_shift as f32,
                aspect_ratio: aspect_ratio as f32,
                tunnel_mode: polar_view.tunnel_mode,
                length_total: polar_view.length_total as f32,
                length_circle: polar_view.length_circle as f32
            };

            let mut draw_params =  glium::draw_parameters::DrawParameters::default();
            draw_params.blend = Blend::alpha_blending();
            
            target.draw(&vertex_buffer,
                        &glium::index::NoIndices(self.primitive_type),
                        &self.program,
                        &uniforms,
                        &draw_params).unwrap();
        }
    }

    fn flush_buffer(&mut self) {
        self.vertices = Vec::new();
    }

    fn get_vertices(&mut self) -> &mut Vec<PolarPixelVertex> {
        &mut self.vertices
    }
}

impl PolarBuffer {
    pub fn new(display: &Display) -> Self {
        PolarBuffer {
            vertices: Vec::new(),
            program: make_program_from_shaders(PolarPixel::get_shaders(), display),
            primitive_type: PolarPixel::get_primitive_type(),
        }
    }
}
