use std::slice;
use std::ptr;

use super::Color;
use super::space::*;

pub struct Buffer<'a> {
    buf: &'a mut [Color],
    width: usize,
    height: usize,
}

impl<'a> Buffer<'a> {
    pub fn from_raw(ptr: *mut Color, len: usize, width: usize) -> Self {
        if width != 0 && len % width != 0 {
            unreachable!()
        }
        Buffer {
            buf: unsafe { slice::from_raw_parts_mut(ptr, len) },
            width,
            height: if width == 0 { 0 } else { len / width },
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            ptr::copy(&color, self.buf.as_mut_ptr(), self.buf.len());
        }
    }

    pub fn fade(&mut self, k: u8) {
        for pixel in self.buf.iter_mut() {
            pixel.a = (pixel.a as u16 * k as u16 / 256) as u8;
        }
    }

    pub fn point(&mut self, x: isize, y: isize, color: Color) {
        if clip_value(x, 0, self.width as isize).is_fixed() ||
            clip_value(y, 0, self.height as isize).is_fixed()
        {
            return;
        }
        self.unchecked_point(x as usize, y as usize, color);
    }

    #[inline]
    pub fn unchecked_point(&mut self, x: usize, y: usize, color: Color) {
        self.buf[y * self.width + x] = color;
    }

    pub fn bar<R>(&mut self, rect: R, color: Color)
        where R: Into<Rect<isize>>
    {
        let (rect, _is_clipped) = rect.into()
            .clip(((0, 0), (self.width as isize, self.height as isize)))
            .into_inner_checked();

        if rect.is_collapsed() {
            return;
        }

        let (left, top) = (rect.a.x as usize, rect.a.y as usize);
        let (right, bottom) = (rect.b.x as usize, rect.b.y as usize);
        let width = right - left;

        let mut start = top * self.width + left;
        for _ in top .. bottom {
            for v in &mut self.buf[start .. start + width] {
                *v = color;
            }
            start += self.width;
        }
    }

    /// Bresenham's line
    pub fn line<PA, PB>(&mut self, a: PA, b: PB, color: Color)
        where
            PA: Into<Point<isize>>,
            PB: Into<Point<isize>>,
    {
        let (a, b) = (a.into(), b.into());

        if a.x == b.x || a.y == b.y {
            self.bar((a, b), color);
        }

        let (a, b) =
            if a.x > b.x { (b, a) } else { (a, b) };

        let (transpose, a, b) =
            if (b.y - a.y).abs() > (b.x - a.x).abs() {
                (true, a.transpose(), b.transpose())
            } else {
                (false, a, b)
            };

        let (a, b) =
            if a.x > b.x { (b, a) } else { (a, b) };

        debug_assert!(b.x >= a.x);

        let dx = b.x - a.x;
        let dy = b.y - a.y;

        //debug_assert!(dy.abs() < dx.abs());

        let (sy, sd) = if dy > 0 {
            (1, 2 * dy)
        } else {
            (-1, - 2 * dy)
        };

        let mut d: isize = sd - dx;
        let mut y: isize = a.y;

        for x in a.x .. b.x {
            let (tx, ty) = if transpose { (y, x) } else { (x, y) };
            self.point(tx, ty, color);
            if d > 0 {
                y += sy;
                d -= 2 * dx;
            }
            d += sd;
        }
    }

    pub fn rect(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: Color) {
        let x1 = clip_value(x1, 0, self.width as isize).into_inner();
        let x2 = clip_value(x2, 0, self.width as isize).into_inner();
        if x1 == x2 {
            return;
        }
        let y1 = clip_value(y1, 0, self.height as isize).into_inner();
        let y2 = clip_value(y2, 0, self.height as isize).into_inner();
        if y1 == y2 {
            return;
        }

        let (left, right) = sort(x1 as usize, x2 as usize).into_inner();
        let (top, bottom) = sort(y1 as usize, y2 as usize).into_inner();
        let (right_1, bottom_1) = (right - 1, bottom - 1);

        for x in left .. right {
            self.unchecked_point(x, top, color);
        }
        if bottom_1 != top {
            for x in left .. right {
                self.unchecked_point(x, bottom_1, color);
            }
        }
        for y in top .. bottom {
            self.unchecked_point(left, y, color);
        }
        if right_1 != left {
            for y in top .. bottom {
                self.unchecked_point(right_1, y, color);
            }
        }
    }
}
