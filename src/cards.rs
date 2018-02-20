
use std::f32::consts::PI;

use ggez::*;
use ggez::graphics::{Drawable, Point2, Vector2};

use bbox::BoundingBox;
use resources::Resources;

pub const WIDTH: f32 = 123.0;
pub const HEIGHT: f32 = 233.0;


#[derive(Debug)]
pub struct Card {
    pos: Point2,
    bbox: BoundingBox,
    stack: Option<usize>,
    face_up: bool,
    suite: Suite,
}

impl Card {
    pub fn new(suite: Suite) -> Card {
        Card {
            pos: Point2::new(0.0, 0.0),
            bbox: BoundingBox::new(0.0, 0.0, WIDTH, HEIGHT),
            stack: None,
            face_up: false,
            suite
        }
    }

    pub fn set_faceup(&mut self, b: bool) {
        self.face_up = b;
    }

    pub fn is_faceup(&self) -> bool {
        self.face_up
    }

    pub fn suite(&self) -> &Suite {
        &self.suite
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

    pub fn get_bounds(&self) -> &BoundingBox {
        &self.bbox
    }

    pub fn is_hit(&self, x: f32, y: f32) -> bool{
        self.bbox.is_hit(x, y)
    }

    pub fn draw(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        if self.face_up {
            res.card_front.draw(ctx, self.pos, 0.0)?;
            self.suite.draw(ctx, self.pos, res)?;
        } else {
            res.card_back.draw(ctx, self.pos, 0.0)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Suite {
    Flower,
    Dragon(Color),
    Number(u8, Color),
}

impl Suite {
    fn draw(&self, ctx: &mut Context, pos: Point2, res: &Resources) -> GameResult<()> {
        let small_icon;
        let large_icon;
        let ih;
        match *self {
            Suite::Flower => {
                small_icon = &res.flower_icon;
                ih = small_icon.height() as f32 / 2.0 - 18.0;

                large_icon = &res.flower_image;

                graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
            },
            Suite::Dragon(ref c) => {
                small_icon = &res.dragon_icons[c];
                ih = small_icon.height() as f32 / 2.0 - 18.0;

                large_icon = &res.dragon_images[c];

                c.set_icon_color(ctx)?;
            }
            Suite::Number(i, ref c) => {
                c.set_font_color(ctx)?;
                let nr = &res.numbers[i as usize - 1];
                let nw = nr.width() as f32 / 2.0 - 20.0;
                let nh = nr.height() as f32 / 2.0 - 18.0;
                graphics::draw(ctx, nr, pos + Vector2::new(-nw, -nh), 0.0)?;
                graphics::draw(ctx, nr, pos + Vector2::new(WIDTH + nw, HEIGHT + nh), PI)?;

                small_icon = &res.suite_icons[c];
                ih = small_icon.height() as f32 / 2.0 - 37.0;

                large_icon = &res.suite_images[c][i as usize - 1];

                c.set_icon_color(ctx)?;
            },
        }

        let iw = small_icon.width() as f32 / 2.0 - 20.0;
        graphics::draw(ctx, small_icon, pos + Vector2::new(-iw, -ih), 0.0)?;
        graphics::draw(ctx, small_icon, pos + Vector2::new(WIDTH + iw, HEIGHT + ih), PI)?;

        let lw = (WIDTH - large_icon.width() as f32) / 2.0;
        let lh = (HEIGHT - large_icon.height() as f32) / 2.0;
        graphics::draw(ctx, large_icon, pos + Vector2::new(lw, lh), 0.0)?;

        //graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new(pos.x, pos.y, WIDTH, HEIGHT))?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    Red,
    Green,
    White
}

impl Color {
    fn set_font_color(&self, ctx: &mut Context) -> GameResult<()> {
        match *self {
            Color::Red => graphics::set_color(ctx, graphics::Color::new(0.7, 0.2, 0.1, 1.0)),
            Color::Green => graphics::set_color(ctx, graphics::Color::new(0.1, 0.4, 0.3, 1.0)),
            Color::White => graphics::set_color(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0)),
        }
    }
    fn set_icon_color(&self, ctx: &mut Context) -> GameResult<()> {
        match *self {
            Color::Red => graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0)),
            Color::Green => graphics::set_color(ctx, graphics::Color::new(0.1, 0.4, 0.3, 1.0)),
            Color::White => graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0)),
        }
    }
}