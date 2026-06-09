pub mod buffer;
pub mod color;
// pub mod font;

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    #[inline]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
