use rendering::{Renderable, PolarPrimitive, PolarPixel};

impl Renderable<PolarPrimitive> for PolarPixel {
    fn get_primitives(&mut self) -> Vec<PolarPrimitive> { vec![PolarPrimitive::PolarPix(self.clone())] }
}