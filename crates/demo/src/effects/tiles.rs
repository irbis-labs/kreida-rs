use kreida::Buffer;
use kreida::Color;
use kreida::ToolBox;
use kreida::WithToolBox;
use rand::Rng;

use crate::effects::Effect;

pub struct Tiles {
    hn: u32,
    vn: u32,
    gap: u32,
    v: f64,
    tile_phase: Vec<f64>,
}

impl Tiles {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let mx = 24;
        let k = rng.random_range(1..mx);
        let v = mx as f64 / k as f64;

        let hn = 3 * k;
        let vn = 2 * k;
        let gap = 2;

        let tile_phase = (0..(hn * vn)).map(|_| rng.random()).collect();

        Tiles {
            hn,
            vn,
            gap,
            v,
            tile_phase,
        }
    }
}

impl Effect for Tiles {
    fn update(&mut self, buffer: &mut Buffer<'_>, phi: f64) {
        let bar = ToolBox::default().pixel_bar();

        let h = buffer.height();
        let w = buffer.width();

        buffer.fade(200);
        let tile_h = h / self.vn;
        let tile_w = w / self.hn;
        let gap = self.gap;

        let gap2 = gap / 2;
        let dh = (h - self.vn * tile_h) / 2;
        let dw = (w - self.hn * tile_w) / 2;

        for i in 0..self.vn {
            for j in 0..self.hn {
                let x0 = j * tile_w + gap2 + dw;
                let y0 = i * tile_h + gap2 + dh;
                let x1 = (j + 1) * tile_w - gap2 + dw;
                let y1 = (i + 1) * tile_h - gap2 + dh;

                let n = i * self.hn + j;
                let phase = self.tile_phase[n as usize] as f64;
                let c = (phase + phi * 0.01 * self.v) % 1.0;
                let r = (c * 255.0) as u8;
                let g = (c * 128.0) as u8;
                let b = (c * 64.0) as u8;
                let color = Color::rgb(r, g, b);

                bar.fg(color)
                    .bar(buffer, (x0 as i32, y0 as i32, x1 as i32, y1 as i32));
            }
        }
    }
}
