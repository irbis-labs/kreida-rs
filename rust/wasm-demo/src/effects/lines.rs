use kreida::*;

use crate::effects::Effect;

#[derive(Default)]
pub struct Lines {}

impl Effect for Lines {
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let w_2: i32 = buffer.width() as i32 / 2;
        let h_2: i32 = buffer.height() as i32 / 2;

        use std::f64::consts::PI;
        use std::cmp::min;

        buffer.fade(245);

        let toolbox = ToolBox::default();
        let mut pl = toolbox.pixel_line();

        let r = min(h_2, w_2) as f64;
        const N: i32 = 32;
        for i in 0..N {
            let tt = phi * PI * 0.5 * 5.0 + PI * i as f64 / (N * 4) as f64;

            let k1 = (1.555 + (tt * 0.03733).sin() * 0.71) * 100.0;
            let k2 = (1.555 + (tt * 0.03377).sin() * 0.69) * 100.0;
            let kk = 1.5 + (tt * 0.003).cos();

            let a = Point::new(w_2 + (r * (tt + (k1 - kk)).cos()) as i32, h_2 + (r * (tt + (k2 - kk)).sin()) as i32);
            let b = Point::new(w_2 + (r * (tt + (k1 + kk)).cos()) as i32, h_2 + (r * (tt + (k2 + kk)).sin()) as i32);

            let c1 = PI / 2.0 * (i % N) as f64 / N as f64;
            let color = Color::rgb(
                ((0.5 + 0.5 * (0.003 * tt + c1).sin()) * 255.0) as u8,
                ((0.5 + 0.5 * (0.005 * tt + c1).sin()) * 255.0) as u8,
                ((0.5 + 0.5 * (0.007 * tt + c1).sin()) * 255.0) as u8,
            );

            // buffer.line(a, b, color);
            pl = pl.fg(color).line(buffer, a, b);
        }
    }
}