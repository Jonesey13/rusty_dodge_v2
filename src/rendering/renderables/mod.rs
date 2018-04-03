pub mod polar_pixel;
pub mod text;
use ::rendering::primitives::PolarPrimitive;
use gg::rendering::Renderable;

pub type PolarRenderable = Renderable<PolarPrimitive>;