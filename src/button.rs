
use ggez::*;
use ggez::graphics::{Drawable, Point2, Vector2};

use cards::Color;
use resources::Resources;

const RADIUS: f32 = 30.0;
const RADIUS2: f32 = RADIUS * RADIUS;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum State {
    Active,
    Up,
    Down,
}

pub struct Button {
    color: Color,
    pos: Point2,
    state: State,
}

impl Button {
    pub fn new(color: Color, pos: Point2) -> Button {
        Button {
            color,
            pos,
            state: State::Up,
        }
    }

    pub fn accept_click(&self, p: Point2) -> bool {
        if let State::Active = self.state {
            let dx = self.pos.x - p.x;
            let dy = self.pos.y - p.y;
            dx * dx + dy * dy <= RADIUS2
        } else {
            false
        }
    }

    pub fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        let img = &res.button_images[&(self.color, self.state)];
        img.draw(ctx, self.pos - Vector2::new(img.width() as f32, img.height() as f32) / 2.0, 0.0)?;
        //graphics::circle(ctx, graphics::DrawMode::Line(1.0), self.pos, RADIUS, 0.1)?;
        Ok(())
    }
}
