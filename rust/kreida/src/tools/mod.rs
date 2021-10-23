pub mod bar;
pub mod line;

use crate::Color;

pub use self::bar::PixelBar;
pub use self::line::PixelLine;

#[derive(Clone, Copy, Debug)]
pub struct ToolBox {
    pub fg: Color,
    pub bg: Color,
}

pub trait HasToolBox {
    fn toolbox(&self) -> &ToolBox;
    fn toolbox_mut(&mut self) -> &mut ToolBox;
}

pub trait WithToolBox {
    fn fg(self, color: Color) -> Self;
    fn bg(self, color: Color) -> Self;
}

impl<T> WithToolBox for T
where
    T: HasToolBox,
{
    fn fg(mut self, color: Color) -> Self {
        self.toolbox_mut().fg = color;
        self
    }

    fn bg(mut self, color: Color) -> Self {
        self.toolbox_mut().bg = color;
        self
    }
}

impl Default for ToolBox {
    fn default() -> Self {
        ToolBox {
            fg: Color::transparent(),
            bg: Color::black(),
        }
    }
}

impl HasToolBox for ToolBox {
    fn toolbox(&self) -> &ToolBox {
        self
    }

    fn toolbox_mut(&mut self) -> &mut ToolBox {
        self
    }
}

impl ToolBox {
    pub fn toolbox(self) -> Self {
        self
    }

    pub fn pixel_line(self) -> PixelLine {
        PixelLine::new(self)
    }

    pub fn pixel_bar(self) -> PixelBar {
        PixelBar::new(self)
    }
}
