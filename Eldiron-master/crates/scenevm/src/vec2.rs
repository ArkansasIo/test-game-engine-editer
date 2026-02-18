//! 2D vector math
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self { x: self.x / len, y: self.y / len }
        } else {
            Self::zero()
        }
    }
    pub fn perp(self) -> Self {
        Self { x: -self.y, y: self.x }
    }
}
