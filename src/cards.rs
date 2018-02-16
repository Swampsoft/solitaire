
use ggez::*;
use ggez::graphics::Point2;

use bbox::BoundingBox;
use resources::Resources;

pub const WIDTH: f32 = 122.0;
pub const HEIGHT: f32 = 237.0;


#[derive(Debug)]
pub struct Card {
    pos: Point2,
    bbox: BoundingBox,
    stack: Option<usize>,
    face_up: bool,
}

impl Card {
    pub fn new() -> Card {
        Card {
            pos: Point2::new(0.0, 0.0),
            bbox: BoundingBox::new(0.0, 0.0, WIDTH, HEIGHT),
            stack: None,
            face_up: true,
        }
    }

    pub fn is_faceup(&self) -> bool {
        self.face_up
    }

    pub fn get_pos(&self) -> Point2 {
        self.pos
    }

    pub fn set_pos(&mut self, p: Point2) {
        self.pos = p;
        self.bbox = BoundingBox::new(p.x, p.x + WIDTH, p.y, p.y + HEIGHT)
    }

    pub fn move_pos(&mut self, dx: f32, dy: f32) {
        self.pos.x += dx;
        self.pos.y += dy;

        self.bbox.topleft.x += dx;
        self.bbox.topleft.y += dy;
        self.bbox.bottomright.x += dx;
        self.bbox.bottomright.y += dy;
    }

    pub fn get_bounds(&self) -> BoundingBox {
        self.bbox
    }

    pub fn is_hit(&self, x: f32, y: f32) -> bool{
        self.bbox.is_hit(x, y)
    }

    pub fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        use graphics::*;
        draw(ctx, &res.card_front, self.pos, 0.0)?;
        Ok(())
    }
}

