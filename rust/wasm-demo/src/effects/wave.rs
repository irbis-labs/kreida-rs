use kreida::*;

use crate::effects::Effect;

#[derive(Default)]
pub struct Wave {}

impl Effect for Wave {
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let bar = ToolBox::default().pixel_bar();

        let h_2 = (buffer.height() / 2) as i32;
        let w_2 = (buffer.width() / 2) as i32;

        buffer.fade(200);

        const N: i32 = 80;
        use std::f64::consts::PI;
        const PI2: f64 = PI * 2.0;
        use std::cmp::min;
        let size = min(h_2 * 2, w_2) as f64 * 1.5;
        let size_kx = size / N as f64;
        let size_ky = size / N as f64 / 2.0;

        let ta = phi * PI2 / 10.0;
        let pa_4 = PI * (ta / 4.0).sin();

        for rn in 0..N {
            let r = rn as f64 / N as f64;
            let rkx = rn as f64 * size_kx;
            let rky = rn as f64 * size_ky;

            let m = (rkx * PI2 / 24.0) as i32;
            let pi2_m = PI2 / (m as f64);

            let rkb = (2.0 + (pa_4 / 10.0).sin()) / 3.0;

            for dn in 0..m {
                let d = dn as f64 * pi2_m;
                let dkb = 2.0 * (ta * 0.25 - r * PI2 * 0.5).sin() * (2.0 - r);
                let dkr = 1.5 * (ta * 3.0 - r * PI2 * 3.0).sin();

                let wings = 7.0;

                let a = d + dkb * (1.0 - r / 2.0);
                let b = (d * wings).sin();
                let k = rkb + (d * wings).cos().abs() / N as f64;

                let c = a * 2.0 + r * 4.0 - ta * 2.0;
                let color = Color::rgb(
                    ((0.5 + 0.5 * (0.03 * ta + c).sin()) * 255.0) as u8,
                    ((0.5 + 0.5 * (0.05 * ta + c).sin()) * 255.0) as u8,
                    ((0.5 + 0.5 * (0.07 * ta + c).sin()) * 255.0) as u8,
                );

                let x = a.cos();
                let y = a.sin();

                let ty = -(r * b + dkr) * size * 0.015;

                let x = w_2 + (rkx * k * x).round() as i32;
                let y = h_2 + (ty + rky * k * y).round() as i32;
                bar.fg(color).bar(buffer, (x, y, x + 2, y + 2));
            }
        }
    }
}