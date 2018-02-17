
use ggez::*;
use ggez::graphics::{Drawable, Point2, Vector2};

use cards::Color;
use resources::Resources;

const RADIUS: f32 = 30.0;
const RADIUS2: f32 = RADIUS * RADIUS;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ButtonState {
    Active,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Button {
    color: Color,
    pos: Point2,
    state: ButtonState,
}

impl Button {
    pub fn new(color: Color, pos: Point2) -> Button {
        Button {
            color,
            pos,
            state: ButtonState::Down,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn state(&self) -> ButtonState {
        self.state
    }

    pub fn set_state(&mut self, state: ButtonState) {
        self.state = state;
    }

    pub fn accept_click(&self, x: f32, y: f32) -> bool {
        if let ButtonState::Active = self.state {
            let dx = self.pos.x - x;
            let dy = self.pos.y - y;
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
