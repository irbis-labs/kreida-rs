use std::slice;

use num::Integer;

use super::space::*;
use super::Color;

pub struct Buffer<'a> {
    buf: &'a mut [Color],
    width: u32,
    height: u32,
}

impl<'a> Buffer<'a> {
    pub fn new(buf: &'a mut [Color], width: u32) -> Self {
        assert!(width > 0);
        assert!(width <= i32::max_value() as u32);
        let len = buf.len() as u32;
        let (height, rem) = len.div_rem(&width);
        assert!(rem == 0);
        assert!(height <= i32::max_value() as u32);
        Buffer { buf, width, height }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn buf_mut(&mut self) -> &mut [Color] {
        &mut self.buf
    }

    pub fn as_bytes(&self) -> &[u8] {
        let len = self.buf.len() * std::mem::size_of::<Color>();
        unsafe { slice::from_raw_parts(self.buf.as_ptr() as *const u8, len) }
    }

    pub fn clear(&mut self, color: Color) {
        self.buf.fill(color);
        // let count = self.buf.len();
        // unsafe { std::ptr::copy(&color, self.buf.as_mut_ptr(), count) };
    }

    pub fn fade(&mut self, k: u8) {
        for pixel in self.buf.iter_mut() {
            pixel.a = (pixel.a as u16 * k as u16 / 256) as u8;
        }
    }

    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        if clip_value(x, 0, self.width as i32).is_fixed()
            || clip_value(y, 0, self.height as i32).is_fixed()
        {
            return;
        }
        self.unchecked_point(x as u32, y as u32, color);
    }

    #[inline]
    pub fn unchecked_point(&mut self, x: u32, y: u32, color: Color) {
        let offset = y * self.width + x;
        self.buf[offset as usize] = color;
    }

    pub fn rect(&mut self, rect: impl Into<Rect<i32>>, color: Color) {
        let (rect, _not_clipped) = rect
            .into()
            .clip((0, 0, self.width as i32, self.height as i32))
            .into_inner_checked();

        if rect.is_collapsed() {
            return;
        }

        let (left, top, right, bottom) = (
            rect.a.x as u32,
            rect.a.y as u32,
            rect.b.x as u32,
            rect.b.y as u32,
        );
        let (right_1, bottom_1) = (right - 1, bottom - 1);

        for x in left..right {
            self.unchecked_point(x, top, color);
        }
        if bottom_1 != top {
            for x in left..right {
                self.unchecked_point(x, bottom_1, color);
            }
        }
        for y in top..bottom {
            self.unchecked_point(left, y, color);
        }
        if right_1 != left {
            for y in top..bottom {
                self.unchecked_point(right_1, y, color);
            }
        }
    }
}
