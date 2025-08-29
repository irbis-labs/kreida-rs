use std::cmp::min;
use std::f64::consts::PI;
use std::f64::consts::TAU;

use kreida::*;

use crate::effects::Effect;

const N: i32 = 100;
const WINGS: f64 = 21.0;

pub struct Wave {
    n: i32,
    nf: f64,
    wings: f64,
}

impl Wave {
    pub fn new() -> Self {
        let n = N;
        let nf = n as f64;
        let wings = WINGS;
        Wave { n, nf, wings }
    }
}

impl Effect for Wave {
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let bar = ToolBox::default().pixel_bar();

        let h_2 = (buffer.height() / 2) as i32;
        let w_2 = (buffer.width() / 2) as i32;

        buffer.fade(200);

        let size = min(h_2 * 2, w_2) as f64 * 1.5;
        let size_kx = size / self.nf;
        let size_ky = size / self.nf / 2.0;

        let ta = phi * TAU / 10.0;
        let pa_4 = PI * (ta / 4.0).sin();

        for rn in 0..self.n {
            let r = rn as f64 / self.nf;
            let rkx = rn as f64 * size_kx;
            let rky = rn as f64 * size_ky;

            let m = (rkx * TAU / 12.0) as i32;
            let tau_m = TAU / (m as f64);

            let rkb = (2.0 + (pa_4 / 10.0).sin()) / 3.0;

            for dn in 0..m {
                let d = dn as f64 * tau_m;
                let dkb = 2.0 * (ta * 0.25 - r * TAU * 0.5).sin() * (2.0 - r);
                let dkr = 1.5 * (ta * 3.0 - r * TAU * 3.0).sin();

                let a = d + dkb * (1.0 - r / 2.0);
                let dw = d * self.wings;
                let b = dw.sin();
                let k = rkb + dw.cos().abs() / self.nf;

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
