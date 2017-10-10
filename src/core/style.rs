#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_bool(v: bool) -> Self {
        match v {
            true => Color::black(),
            false => Color::white(),
        }
    }

    pub fn transparent() -> Self {
        Color { r: 0x00, g: 0x00, b: 0x00, a: 0x00, }
    }

    pub fn black() -> Self {
        Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF, }
    }

    pub fn white() -> Self {
        Color { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF, }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 0xFF }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn red(self, r: u8) -> Self {
        Color { r, .. self }
    }

    pub fn green(self, g: u8) -> Self {
        Color { g, .. self }
    }

    pub fn blue(self, b: u8) -> Self {
        Color { b, .. self }
    }

    pub fn alpha(self, a: u8) -> Self {
        Color { a, .. self }
    }

    pub fn as_bool(&self) -> bool {
        self.r == 0
    }
}
