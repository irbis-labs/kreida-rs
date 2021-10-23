use kreida::*;
use kreida::fun::*;

use crate::effects::Effect;

#[derive(Default)]
pub struct Spirograph {
    last_phi: f64,
}

impl Effect for Spirograph {
    fn set_time(&mut self, phi: f64) {
        self.last_phi = phi;
    }

    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let bar = ToolBox::default().pixel_bar();

        let w_2: i32 = buffer.width() as i32 / 2;
        let h_2: i32 = buffer.height() as i32 / 2;
        let r = min(h_2, w_2) as f64;

        use std::cmp::min;

        buffer.fade(255);

        let f = |tt: f64| -> (Point<i32>, Color) {
            let tr1 = step(tt / 24.0, 9.0) / 9.0;
            let tr2 = step(tt / 12.0, 3.0) / 3.0;

            let r0 = 1.0;

            let r1 = r0 * step(map_range((0.0, 10.0), (3.0 / 23.0, 22.0 / 23.0), (tr1 * 3.0) % 10.0).sqrt(), 1.0 / 19.0);
            let r2 = r1 * step(map_range((0.0, 10.0), (2.0 / 17.0, 16.0 / 17.0), (tr2 * 7.0) % 10.0).sqrt(), 1.0 / 14.0);

            let tc = tt * 0.005;

            let sk = 5.0;
            let ts = tt / sk;
            let t1 = 5.0 * (tt + 0.9 * sk * ts.cos());
            let t2 = -(t1 * r0 / r1);

            let x = (r0 - r1) * (t1).cos() + (r2) * t2.cos();
            let y = (r0 - r1) * (t1).sin() + (r2) * t2.sin();
            let a = Point::new(w_2 + (r * x) as i32, h_2 + (r * y) as i32);

            let color = Color::rgb(
                ((0.5 + 0.5 * (11.0 * tc).sin() * 0.9) * 255.0) as u8,
                ((0.5 + 0.5 * (17.0 * tc).sin() * 0.9) * 255.0) as u8,
                ((0.5 + 0.5 * (23.0 * tc).sin() * 0.9) * 255.0) as u8,
            );

            (a, color)
        };

        let base = self.last_phi;
        let elapsed = phi - base;
        // ConsoleService::debug(&format!("Elapsed {}", elapsed));
        self.last_phi = phi;

        const N: f64 = 5000.0;

        let num = N * elapsed;
        let step = elapsed / num;

        for n in 0..num as i32 {
            let tt = base + n as f64 * step;
            let (a, color) = f(tt);
            bar.fg(color).bar(buffer, (a.x, a.y, a.x + 2, a.y + 2));
        }
    }
}