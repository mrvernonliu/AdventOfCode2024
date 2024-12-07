#[derive(Debug)]
pub struct Position {
    pub(crate) x: usize,
    pub(crate) y: usize
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}