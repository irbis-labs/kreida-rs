use crate::Buffer;
use crate::HasToolBox;
use crate::Rect;
use crate::ToolBox;

#[derive(Clone, Copy, Debug)]
pub struct PixelBar {
    pub toolbox: ToolBox,
}

impl HasToolBox for PixelBar {
    fn toolbox(&self) -> &ToolBox {
        &self.toolbox
    }

    fn toolbox_mut(&mut self) -> &mut ToolBox {
        &mut self.toolbox
    }
}

impl PixelBar {
    pub fn new(toolbox: ToolBox) -> Self {
        PixelBar { toolbox }
    }

    pub fn bar(self, buf: &mut Buffer<'_>, rect: impl Into<Rect<i32>>) -> Self {
        let (rect, _is_clipped) = rect
            .into()
            .clip((0, 0, buf.width() as i32, buf.height() as i32))
            .into_inner_checked();

        if rect.is_collapsed() {
            return self;
        }

        let (left, top, right, bottom) = (rect.a.x, rect.a.y, rect.b.x, rect.b.y);
        let width = (right - left) as usize;

        let mut start = (top * buf.width() as i32 + left) as usize;
        for _ in top..bottom {
            for v in &mut buf.buf_mut()[start..start + width] {
                *v = self.toolbox.fg;
            }
            start += buf.width() as usize;
        }

        self
    }
}
