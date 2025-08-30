use std::cmp::min;
use std::f64::consts::TAU;

use kreida::fun::*;
use kreida::*;

use crate::effects::Effect;

const N: f64 = 10000.0;
const MOD: u32 = 8;
const CYCLE_TIME: f64 = 5.0;

#[derive(Default)]
pub struct Spirograph {
    start_phi: f64,
    last_phi: f64,
}

impl Effect for Spirograph {
    fn set_time(&mut self, phi: f64) {
        self.start_phi = phi;
        self.last_phi = 0.0;
    }

    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let bar = ToolBox::default().pixel_bar();

        let w_2: i32 = buffer.width() as i32 / 2;
        let h_2: i32 = buffer.height() as i32 / 2;
        let rk = min(h_2, w_2) as f64;

        let f = |ct: f64, _stage: u32, st: f64| -> (Point<i32>, Color) {
            let tr1 = step(TAU * st * 24.0, 9.0) / 9.0;
            let tr2 = step(TAU * st * 12.0, 3.0) / 3.0;

            let r0 = 1.0;

            let r1 = r0
                * step(
                    map_range((0.0, 10.0), (3.0 / 23.0, 22.0 / 23.0), (tr1 * 3.0) % 10.0).sqrt(),
                    1.0 / 19.0,
                );
            let r2 = r1
                * step(
                    map_range((0.0, 10.0), (2.0 / 17.0, 16.0 / 17.0), (tr2 * 7.0) % 10.0).sqrt(),
                    1.0 / 14.0,
                );

            // let sk = 5.0;
            // let ts = tt / sk;
            // let t1 = 5.0 * (tt + 0.9 * sk * ts.cos());
            // let t1 = 5.0 * tt;
            let t1 = ct * TAU * 99.0;
            let t2 = -(t1 * r0 / r1);

            let x = (r0 - r1) * (t1).cos() + (r2) * t2.cos();
            let y = (r0 - r1) * (t1).sin() + (r2) * t2.sin();
            let a = Point::new(w_2 + (rk * x) as i32, h_2 + (rk * y) as i32);

            let tc = st * 0.005;

            let color = Color::rgb(
                ((0.5 + 0.5 * (11.0 * tc).sin() * 0.9) * 255.0) as u8,
                ((0.5 + 0.5 * (17.0 * tc).sin() * 0.9) * 255.0) as u8,
                ((0.5 + 0.5 * (23.0 * tc).sin() * 0.9) * 255.0) as u8,
            );

            (a, color)
        };

        debug_assert!(phi >= self.start_phi);
        let phi = phi - self.start_phi;
        let base = self.last_phi;
        let elapsed = phi - base;
        // ConsoleService::debug(&format!("Elapsed {}", elapsed));
        self.last_phi = phi;

        let num = N * elapsed;
        let step = elapsed / num;

        for n in 0..num as i32 {
            let at = base + n as f64 * step;
            let pt = at / CYCLE_TIME;
            let ct = pt % 1.0;

            let cn = pt as u32;
            let stage = cn % MOD;
            let st = cn as f64 * CYCLE_TIME;

            if stage + 2 == MOD {
                break;
            } else if stage + 1 == MOD {
                buffer.fade(255);
                break;
            } else {
                let (a, color) = f(ct, stage, st);
                bar.fg(color).bar(buffer, (a.x, a.y, a.x + 2, a.y + 2));
            }
        }
    }
}
