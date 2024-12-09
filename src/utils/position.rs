use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Position<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Position<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}