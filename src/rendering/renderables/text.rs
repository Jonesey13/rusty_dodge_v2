use gg::rendering::{Renderable, PlainText};
use ::rendering::PolarPrimitive;

impl Renderable<PolarPrimitive> for PlainText {
    fn get_primitives(&mut self) -> Vec<PolarPrimitive> { vec![PolarPrimitive::Text(self.clone())] }
}