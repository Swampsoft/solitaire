
use std::f32;

use ggez::graphics::Point2;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub topleft: Point2,
    pub bottomright: Point2
}

impl BoundingBox {
    pub fn new(l: f32, r: f32, t: f32, b: f32) -> BoundingBox {
        BoundingBox {
            topleft: Point2::new(l, t),
            bottomright: Point2::new(r, b)
        }
    }

    pub fn new_empty() -> BoundingBox {
        BoundingBox::new(f32::INFINITY, -f32::INFINITY, f32::INFINITY, -f32::INFINITY)
    }

    pub fn merge(&mut self, other: &BoundingBox) {
        self.topleft.x = f32::min(self.topleft.x, other.topleft.x);
        self.topleft.y = f32::min(self.topleft.y, other.topleft.y);
        self.bottomright.x = f32::max(self.bottomright.x, other.bottomright.x);
        self.bottomright.y = f32::max(self.bottomright.y, other.bottomright.y);
    }

    pub fn is_hit(&self, x: f32, y: f32) -> bool {
        x >= self.topleft.x && y >= self.topleft.y && x <= self.bottomright.x && y <= self.bottomright.y
    }

    pub fn is_touching(&self, other: &BoundingBox) -> bool {
        let ax = self.topleft.x + self.bottomright.x;
        let ay = self.topleft.y + self.bottomright.y;
        let bx = other.topleft.x + other.bottomright.x;
        let by = other.topleft.y + other.bottomright.y;

        let aw = self.bottomright.x - self.topleft.x;
        let ah = self.bottomright.y - self.topleft.y;
        let bw = other.bottomright.x - other.topleft.x;
        let bh = other.bottomright.y - other.topleft.y;

        f32::abs(ax - bx) < (aw + bw) && f32::abs(ay - by) < (ah + bh)
    }
}