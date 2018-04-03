pub mod polar_pixel;
pub use self::polar_pixel::PolarPixel;
use gg::rendering::PlainText;

#[derive(Clone)]
pub enum PolarPrimitive {
    PolarPix(PolarPixel),
    Text(PlainText),
}