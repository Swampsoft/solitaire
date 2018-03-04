use std::cmp;
use std::collections::BinaryHeap;
use std::f32;

use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{Drawable, Point2, Vector2};

use all::All;
use resources::Resources;

use super::Component;
use super::GameState;
use super::types::*;


enum DrawCommand {
    Card{z: f32, pos: Point2, suite: Suite}
}

#[derive(Default)]
pub struct RenderQueue {
    queue: BinaryHeap<DrawCommand>,
}


impl RenderQueue {
    pub fn new() -> RenderQueue {
        RenderQueue {
            queue: BinaryHeap::new(),
        }
    }

    pub fn render(&mut self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        while let Some(cmd) = self.queue.pop() {
            match cmd {
                DrawCommand::Card{z, pos, suite} => self.render_card(pos, suite, ctx, res)?,
            }
        }
        Ok(())
    }

    pub fn background_render_system(&self, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        graphics::draw(ctx, &res.table_image, Point2::new(0.0, 0.0), 0.0)
    }

    pub fn button_render_system(&self, ctx: &mut Context, res: &Resources,
                                pos: &Component<Point2>, btn: &Component<Button>) -> GameResult<()> {
        for (p, b) in pos.iter().zip(btn.iter()).filter_map(|x| -> Option<(_, &Button)> {x.all()}) {
            let img = &res.button_images[&(b.color, b.state)];
            img.draw(ctx, p - Vector2::new(img.width() as f32, img.height() as f32) / 2.0, 0.0)?;
            //graphics::circle(ctx, graphics::DrawMode::Line(1.0), self.pos, RADIUS, 0.1)?;
        }
        Ok(())
    }

    pub fn stack_render_system(&mut self, ctx: &mut Context, res: &Resources,
                               pos: &Component<Point2>, stk: &Component<Stack>, zs: &Component<f32>) -> GameResult<()> {
        let compound_iterator = pos.iter()
            .zip(stk.iter())
            .zip(zs.iter())
            .filter_map(|x| -> (Option<(_, _, &f32)>) {x.all()});
        for (p, s, &z) in compound_iterator {
            let mut pos = *p;
            let dpos = s.get_stackshift();

            for (i, card) in s.iter().enumerate() {
                let z = z + 0.1 * i as f32;
                graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
                //res.card_front.draw(ctx, pos, 0.0)?;
                //self.render_card(pos, *card, ctx, res)?;
                self.queue.push(DrawCommand::Card{z, pos: pos, suite: *card});
                pos += dpos;
            }
        }
        Ok(())
    }


    fn render_card(&self, pos: Point2, suite: Suite, ctx: &mut Context, res: &Resources) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        match suite {
            Suite::FaceDown => {
                res.card_back.draw(ctx, pos, 0.0)?;
                return Ok(())
            },
            Suite::Flower => {
                res.card_front.draw(ctx, pos, 0.0)?;

                let small_icon = &res.flower_icon;
                let iw = small_icon.width() as f32 / 2.0 - 20.0;
                let ih = small_icon.height() as f32 / 2.0 - 18.0;
                graphics::draw(ctx, small_icon, pos + Vector2::new(-iw, -ih), 0.0)?;
                graphics::draw(ctx, small_icon, pos + Vector2::new(CARD_WIDTH + iw, CARD_HEIGHT + ih), f32::consts::PI)?;

                let large_icon = &res.flower_image;
                let lw = (CARD_WIDTH - large_icon.width() as f32) / 2.0;
                let lh = (CARD_HEIGHT - large_icon.height() as f32) / 2.0;
                graphics::draw(ctx, large_icon, pos + Vector2::new(lw, lh), 0.0)?;
            },
            Suite::Dragon(ref c) => {
                res.card_front.draw(ctx, pos, 0.0)?;

                c.set_icon_color(ctx)?;

                let small_icon = &res.dragon_icons[c];
                let iw = small_icon.width() as f32 / 2.0 - 20.0;
                let ih = small_icon.height() as f32 / 2.0 - 18.0;
                graphics::draw(ctx, small_icon, pos + Vector2::new(-iw, -ih), 0.0)?;
                graphics::draw(ctx, small_icon, pos + Vector2::new(CARD_WIDTH + iw, CARD_HEIGHT + ih), f32::consts::PI)?;

                let large_icon = &res.dragon_images[c];
                let lw = (CARD_WIDTH - large_icon.width() as f32) / 2.0;
                let lh = (CARD_HEIGHT - large_icon.height() as f32) / 2.0;
                graphics::draw(ctx, large_icon, pos + Vector2::new(lw, lh), 0.0)?;
            }
            Suite::Number(i, ref c) => {
                res.card_front.draw(ctx, pos, 0.0)?;

                c.set_icon_color(ctx)?;

                let small_icon = &res.suite_icons[c];
                let iw = small_icon.width() as f32 / 2.0 - 20.0;
                let ih = small_icon.height() as f32 / 2.0 - 37.0;
                graphics::draw(ctx, small_icon, pos + Vector2::new(-iw, -ih), 0.0)?;
                graphics::draw(ctx, small_icon, pos + Vector2::new(CARD_WIDTH + iw, CARD_HEIGHT + ih), f32::consts::PI)?;

                let large_icon = &res.suite_images[c][i as usize - 1];
                let lw = (CARD_WIDTH - large_icon.width() as f32) / 2.0;
                let lh = (CARD_HEIGHT - large_icon.height() as f32) / 2.0;
                graphics::draw(ctx, large_icon, pos + Vector2::new(lw, lh), 0.0)?;

                c.set_font_color(ctx)?;
                let nr = &res.numbers[i as usize - 1];
                let nw = nr.width() as f32 / 2.0 - 20.0;
                let nh = nr.height() as f32 / 2.0 - 18.0;
                graphics::draw(ctx, nr, pos + Vector2::new(-nw, -nh), 0.0)?;
                graphics::draw(ctx, nr, pos + Vector2::new(CARD_WIDTH + nw, CARD_HEIGHT + nh), f32::consts::PI)?;
            },
        }
        //graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new(pos.x, pos.y, WIDTH, HEIGHT))?;
        Ok(())
    }
}

impl DrawCommand {
    fn get_z(&self) -> f32 {
        match *self {
            DrawCommand::Card{z, ..} => z
        }
    }
}

impl cmp::Eq for DrawCommand {}

impl cmp::Ord for DrawCommand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl cmp::PartialOrd for DrawCommand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        other.get_z().partial_cmp(&self.get_z())
    }
}

impl cmp::PartialEq for DrawCommand {
    fn eq(&self, other: &Self) -> bool {
        self.get_z() == other.get_z()
    }
}
