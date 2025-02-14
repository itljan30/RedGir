#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color{r, g, b, a,}
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub const RED: Color = Color{r: 255, g: 0, b: 0, a: 255};
    pub const GREEN: Color = Color{r: 0, g: 255, b: 0, a: 255};
    pub const BLUE: Color = Color{r: 0, g: 0, b: 255, a: 255};
    pub const YELLOW: Color = Color{r: 255, g: 255, b: 0, a: 255};
    pub const CYAN: Color = Color{r: 0, g: 255, b: 255, a: 255};
    pub const MAGENTA: Color = Color{r: 255, g: 0, b: 255, a: 255};
    pub const BLACK: Color = Color{r: 0, g: 0, b: 0, a: 255};
    pub const WHITE: Color = Color{r: 255, g: 255, b: 255, a: 255};
    pub const LIGHT_GRAY: Color = Color{r: 192, g: 192, b: 192, a: 255};
    pub const GRAY: Color = Color{r: 128, g: 128, b: 128, a: 255};
    pub const DARK_GRAY: Color = Color{r: 64, g: 64, b: 64, a: 255};
}
