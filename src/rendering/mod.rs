pub mod primitives;
pub mod renderables;
pub mod glium_renderer;

pub use ::rendering::primitives::{PolarPixel, PolarPrimitive};
pub use ::rendering::glium_renderer::{GliumRenderer};
pub use ::rendering::renderables::{PolarRenderable};

use gg::rendering::renderables::{Renderable, Line, LineShape, Arrow, Circle, BoxBorder, Annulus, AnnularSegment};
use gg::rendering::DisplaySettings;
use gg::games::view_details;
use glium::glutin::EventsLoop;

#[derive(Copy, Clone, Debug, Default)]
pub struct WindowSpec {
    pub aspect_ratio: f64
}