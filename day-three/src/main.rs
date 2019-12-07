#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn up() -> Self {
        Self::new(0, 1)
    }

    pub const fn down() -> Self {
        Self::new(0, -1)
    }

    pub const fn right() -> Self {
        Self::new(1, 0)
    }

    pub const fn left() -> Self {
        Self::new(-1, 0)
    }

    pub const fn manhattan(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<i32> for Vector {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl std::ops::Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Self::Output {
        vector * self
    }
}

pub struct Seg {
    start: Vector,
    direction: Vector,
    length: i32,
}

impl Seg {
    pub fn is_vertical(&self) -> bool {
        self.direction.x == 0
    }

    pub fn is_horizontal(&self) -> bool {
        self.direction.y == 0
    }

    pub fn intersects(&self, other: &Self) -> Option<i32> {
        // Check if we are parallel
        if self.is_vertical() == other.is_vertical() {
            None
        } else {
            // x0 + dx0 * t = x1 + dx1 * u
            // y0 + dy0 * t = y1 + dy1 * u
            //
            // x0 - x1 = dx1 * u - dx0 * t
            // x0 - x1 = dx1 * u - dx0 * t
        }
    }
}

fn main() {
    println!("Hello, world!");
}
