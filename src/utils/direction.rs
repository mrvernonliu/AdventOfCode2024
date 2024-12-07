#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Direction {
    x: i32,
    y: i32
}

impl Direction {
    pub(crate) fn apply_movement(&self, x: i32, y: i32) -> (i32, i32) {
        ((x + self.x), (y + self.y))
    }
    pub(crate) const NORTH: Self = Self { x: 0, y: 1 };
    pub(crate) const SOUTH: Self = Self { x: 0, y: -1 };
    pub(crate) const WEST: Self = Self { x: -1, y: 0 };
    pub(crate) const EAST: Self = Self { x: 1, y: 0 };

    pub(crate) const NORTH_EAST: Self = Self { x: 1, y: 1 };
    pub(crate) const NORTH_WEST: Self = Self { x: -1, y: 1 };
    pub(crate) const SOUTH_EAST: Self = Self { x: 1, y: -1 };
    pub(crate) const SOUTH_WEST: Self = Self { x: -1, y: -1 };

    pub(crate) const ALL_DIRECTIONS: [Direction; 8] = [
        Direction::NORTH,
        Direction::SOUTH,
        Direction::WEST,
        Direction::EAST,
        Direction::NORTH_EAST,
        Direction::NORTH_WEST,
        Direction::SOUTH_EAST,
        Direction::SOUTH_WEST,
    ];
}