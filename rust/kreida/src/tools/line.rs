use crate::Buffer;
use crate::HasToolBox;
use crate::Point;
use crate::ToolBox;

#[derive(Clone, Copy, Debug)]
pub struct PixelLine {
    pub toolbox: ToolBox,
}

impl HasToolBox for PixelLine {
    fn toolbox(&self) -> &ToolBox {
        &self.toolbox
    }

    fn toolbox_mut(&mut self) -> &mut ToolBox {
        &mut self.toolbox
    }
}

impl PixelLine {
    pub fn new(toolbox: ToolBox) -> Self {
        PixelLine { toolbox }
    }

    /// Bresenham's line
    pub fn line(
        self,
        buf: &mut Buffer<'_>,
        a: impl Into<Point<i32>>,
        b: impl Into<Point<i32>>,
    ) -> Self {
        let (a, b) = (a.into(), b.into());

        let color = self.toolbox.fg;

        if a.x == b.x || a.y == b.y {
            buf.point(a.x, a.y, color);
            return self;
        }

        let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };

        let (transpose, a, b) = if (b.y - a.y).abs() > (b.x - a.x).abs() {
            (true, a.transpose(), b.transpose())
        } else {
            (false, a, b)
        };

        let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };

        debug_assert!(b.x >= a.x);

        let dx = b.x - a.x;
        let dy = b.y - a.y;

        //debug_assert!(dy.abs() < dx.abs());

        let (sy, sd) = if dy > 0 { (1, 2 * dy) } else { (-1, -2 * dy) };

        let mut d = sd - dx;
        let mut y = a.y;

        for x in a.x..b.x {
            let (tx, ty) = if transpose { (y, x) } else { (x, y) };
            buf.point(tx, ty, color);
            if d > 0 {
                y += sy;
                d -= 2 * dx;
            }
            d += sd;
        }

        self
    }
}
