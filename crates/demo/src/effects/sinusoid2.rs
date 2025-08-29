use kreida::*;

use crate::effects::Effect;

#[derive(Default)]
pub struct Sinusoid2 {}

impl Effect for Sinusoid2 {
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let bar = ToolBox::default().pixel_bar();

        let h_2 = (buffer.height() / 2) as i32;
        let h_5 = (buffer.height() / 5) as i32;
        let bg = Color::transparent();
        let fg1 = Color::rgb(255, 31, 0);
        let fg2 = Color::rgb(63, 191, 0);

        buffer.fade(240);

        for ix in (10..buffer.width() - 10).step_by(10) {
            let x = (ix as f64) * 2.0 / (buffer.width() as f64) - 1.0;
            let y = h_5 as f64 * ((x * 12.0 + phi * 2.5).cos() + (x * 4.0 - phi * 1.7).cos());
            let iy = h_2 - y.round() as i32;
            let ix = ix as i32;
            let rect = Rect::new((ix - 3, h_2), (ix + 4, iy));
            if y < 0.0 {
                bar.fg(fg1).bar(buffer, rect);
                // bar.fg(bg).bar(buffer, rect.shrink(2));
            } else {
                bar.fg(fg2).bar(buffer, rect);
                bar.fg(bg).bar(buffer, rect.shrink(2));
            }
        }
    }
}
