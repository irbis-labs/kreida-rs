mod lines;
mod sinusoid1;
mod sinusoid2;
mod spirograph;
mod tiles;
mod wave;

use kreida::Buffer;

pub use self::lines::Lines;
pub use self::sinusoid1::Sinusoid1;
pub use self::sinusoid2::Sinusoid2;
pub use self::spirograph::Spirograph;
pub use self::tiles::Tiles;
pub use self::wave::Wave;

pub trait Effect {
    fn set_time(&mut self, _phi: f64) {}
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64);
}
