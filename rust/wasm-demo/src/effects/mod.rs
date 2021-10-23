mod wave;
mod lines;
mod spirograph;
mod sinusoid1;
mod sinusoid2;

pub use self::wave::Wave;
pub use self::lines::Lines;
pub use self::spirograph::Spirograph;
pub use self::sinusoid1::Sinusoid1;
pub use self::sinusoid2::Sinusoid2;

use kreida::Buffer;

pub trait Effect {
    fn set_time(&mut self, _phi: f64) {}
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64);
}
