use kreida::*;

use crate::effects::Effect;

#[derive(Default)]
pub struct Sinusoid1 {}

impl Effect for Sinusoid1 {
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let bar = ToolBox::default().pixel_bar();

        let h_2 = buffer.height() / 2;
        let h_3 = buffer.height() / 5;
        let bg = Color::transparent();
        let fg = Color::rgb(255, 191, 0);

        buffer.clear(bg);

        const N: u32 = 0;
        const M: u32 = 30;

        for i in (N..M).rev() {
            let ii = i as f64 / M as f64;
            let phi = phi * 2.0 - (ii * 2.0);
            let fg = fg.alpha(255 - (ii * 255.0) as u8);

            for ix in (10..buffer.width() - 10).rev().step_by(10) {
                let x = (ix as f64) * 2.0 / (buffer.width() as f64) - 1.0;
                let y = h_3 as f64 * ((x * 12.0 - phi).cos() * (x * 4.0 - phi * 0.9).cos());
                let iy = h_2 as i32 - y.round() as i32;
                let ix = (ix + i * 2) as i32;
                let d = (M - i) as i32 / 1;
                let (x1, y1, x2, y2) = (ix, iy, ix + d, iy + d);
                let rect: Rect<_> = (x1, y1, x2, y2).into();
                bar.fg(fg).bar(buffer, rect);
                bar.fg(fg.alpha(fg.a / 4)).bar(buffer, rect.shrink(2));
            }
        }
    }
}