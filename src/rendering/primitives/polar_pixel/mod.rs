use gg::rendering::render_by_shaders::GliumStandardPrimitive;
use gg::rendering::shaders::Shaders;
mod polar_buffer;
pub use self::polar_buffer::PolarBuffer;

#[derive(Copy, Clone)]
pub struct PolarPixel {
    pub radial: [f64; 2],
    pub angle: [f64; 2],
    pub color: [f64; 4]
}

impl GliumStandardPrimitive for PolarPixel {
    type Vertex = PolarPixelVertex;

    fn get_shaders() -> Shaders {
        Shaders::VertexGeometryFragment(
            include_str!("polar.vs"),
            include_str!("polar.ges"),
            include_str!("polar.fs"))
    }

    fn get_vertex(self) -> Vec<Self::Vertex> { vec![self.clone().into()] }
}

implement_vertex!(PolarPixelVertex, radial, angle, color);

#[derive(Copy, Clone, Debug)]
pub struct PolarPixelVertex {
    pub radial: [f64; 2],
    pub angle: [f64; 2],
    pub color: [f64; 4]
}

impl From<PolarPixel> for PolarPixelVertex {
    fn from(pol: PolarPixel) -> Self {
        PolarPixelVertex {
            radial: pol.radial,
            angle: pol.angle,
            color: pol.color
        }
    }
}
