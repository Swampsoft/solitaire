use std::cmp;
use std::collections::BinaryHeap;
use std::f32;

use ggez::graphics;
use ggez::graphics::{Canvas, DrawParam, Drawable, Sampler};
use ggez::{Context, GameResult};

use crate::resources::Resources;
use crate::types::*;
use crate::utils::all::All;

use super::Component;

enum DrawCommand {
    Card { z: f32, pos: Point2, suite: Suite },
}

#[derive(Default)]
pub struct RenderQueue {
    queue: BinaryHeap<DrawCommand>,
}

impl RenderQueue {
    pub fn render(
        &mut self,
        ctx: &mut Context,
        res: &Resources,
        canvas: &mut Canvas,
    ) -> GameResult<()> {
        while let Some(cmd) = self.queue.pop() {
            match cmd {
                DrawCommand::Card { pos, suite, .. } => {
                    self.render_card(pos, suite, ctx, res, canvas)?
                }
            }
        }
        Ok(())
    }

    pub fn background_render_system(
        &self,
        ctx: &mut Context,
        res: &mut Resources,
        canvas: &mut Canvas,
    ) -> GameResult<()> {
        //graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        //graphics::draw(ctx, &res.table_image, Point2::new(0.0, 0.0), 0.0)?;
        canvas.draw(&res.table_image, DrawParam::new());

        //graphics::set_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0))?;
        let txt = format!("Win Count: {}", res.wins());
        let text = res.get_text(ctx, &txt)?;
        let pos = Point2::new(0.0, 806.0 - text.dimensions(ctx).h);
        //graphics::draw(ctx, text,pos, 0.0)?;
        canvas.draw(
            text,
            DrawParam::new()
                .dest(pos)
                .color(graphics::Color::new(0.0, 0.0, 0.0, 1.0)),
        );

        Ok(())
    }

    pub fn button_render_system(
        &self,
        res: &Resources,
        canvas: &mut Canvas,
        pos: &Component<Point2>,
        btn: &Component<Button>,
    ) -> GameResult<()> {
        //graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        for (p, b) in pos
            .iter()
            .zip(btn.iter())
            .filter_map(|x| -> Option<(_, &Button)> { x.all() })
        {
            let img = &res.button_images[&(b.color, b.state)];
            //img.draw(ctx, p - Vector2::new(img.width() as f32, img.height() as f32) / 2.0, 0.0)?;
            let pos = p - Vector2::new(img.width() as f32, img.height() as f32) / 2.0;
            img.draw(canvas, DrawParam::new().dest(pos));
            //graphics::circle(ctx, graphics::DrawMode::Line(1.0), self.pos, RADIUS, 0.1)?;
        }
        Ok(())
    }

    pub fn stack_render_system(
        &mut self,
        pos: &Component<Point2>,
        stk: &Component<Stack>,
        zs: &Component<f32>,
    ) -> GameResult<()> {
        let compound_iterator = pos
            .iter()
            .zip(stk.iter())
            .zip(zs.iter())
            .filter_map(|x| -> Option<(_, _, &f32)> { x.all() });
        for (p, s, &z) in compound_iterator {
            let mut pos = *p;
            let dpos = s.get_stackshift();

            for (i, card) in s.iter().enumerate() {
                let z = z + 0.1 * i as f32;
                //graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
                self.queue.push(DrawCommand::Card {
                    z,
                    pos,
                    suite: *card,
                });
                pos += dpos;
            }
        }
        Ok(())
    }

    fn render_card(
        &self,
        pos: Point2,
        suite: Suite,
        ctx: &mut Context,
        res: &Resources,
        canvas: &mut Canvas,
    ) -> GameResult<()> {
        //graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        match suite {
            Suite::FaceDown => {
                res.card_back.draw(canvas, DrawParam::new().dest(pos));
                return Ok(());
            }
            Suite::Flower => {
                res.card_front.draw(canvas, DrawParam::new().dest(pos));

                canvas.set_sampler(Sampler::nearest_clamp());
                let small_icon = &res.flower_icon;
                let iw = small_icon.width() as f32 / 2.0 - 20.0;
                let ih = small_icon.height() as f32 / 2.0 - 18.0;
                canvas.draw(
                    small_icon,
                    DrawParam::new().dest(pos + Vector2::new(-iw, -ih)),
                );
                canvas.draw(
                    small_icon,
                    DrawParam::new()
                        .dest(pos + Vector2::new(CARD_WIDTH + iw, CARD_HEIGHT + ih))
                        .rotation(f32::consts::PI),
                );
                canvas.set_default_sampler();

                let large_icon = &res.flower_image;
                let lw = (CARD_WIDTH - large_icon.width() as f32) / 2.0;
                let lh = (CARD_HEIGHT - large_icon.height() as f32) / 2.0;
                canvas.draw(
                    large_icon,
                    DrawParam::new().dest(pos + Vector2::new(lw, lh)),
                );
            }
            Suite::Dragon(ref c) => {
                res.card_front.draw(canvas, DrawParam::new().dest(pos));

                canvas.set_sampler(Sampler::nearest_clamp());
                let small_icon = &res.dragon_icons[c];
                let iw = small_icon.width() as f32 / 2.0 - 20.0;
                let ih = small_icon.height() as f32 / 2.0 - 18.0;
                canvas.draw(
                    small_icon,
                    DrawParam::new()
                        .dest(pos + Vector2::new(-iw, -ih))
                        .color(c.to_icon_color()),
                );
                canvas.draw(
                    small_icon,
                    DrawParam::new()
                        .dest(pos + Vector2::new(CARD_WIDTH + iw, CARD_HEIGHT + ih))
                        .rotation(f32::consts::PI)
                        .color(c.to_icon_color()),
                );
                canvas.set_default_sampler();

                let large_icon = &res.dragon_images[c];
                let lw = (CARD_WIDTH - large_icon.width() as f32) / 2.0;
                let lh = (CARD_HEIGHT - large_icon.height() as f32) / 2.0;
                canvas.draw(
                    large_icon,
                    DrawParam::new()
                        .dest(pos + Vector2::new(lw, lh))
                        .color(c.to_icon_color()),
                );
            }
            Suite::Number(i, ref c) => {
                res.card_front.draw(canvas, DrawParam::new().dest(pos));

                canvas.set_sampler(Sampler::nearest_clamp());
                let small_icon = &res.suite_icons[c];
                let iw = small_icon.width() as f32 / 2.0 - 20.0;
                let ih = small_icon.height() as f32 / 2.0 - 37.0;
                canvas.draw(
                    small_icon,
                    DrawParam::new()
                        .dest(pos + Vector2::new(-iw, -ih))
                        .color(c.to_icon_color()),
                );
                canvas.draw(
                    small_icon,
                    DrawParam::new()
                        .dest(pos + Vector2::new(CARD_WIDTH + iw, CARD_HEIGHT + ih))
                        .rotation(f32::consts::PI)
                        .color(c.to_icon_color()),
                );
                canvas.set_default_sampler();

                let large_icon = &res.suite_images[c][i as usize - 1];
                let lw = (CARD_WIDTH - large_icon.width() as f32) / 2.0;
                let lh = (CARD_HEIGHT - large_icon.height() as f32) / 2.0;
                canvas.draw(
                    large_icon,
                    DrawParam::new()
                        .dest(pos + Vector2::new(lw, lh))
                        .color(c.to_icon_color()),
                );

                let nr = &res.numbers[i as usize - 1];
                let dim = nr.dimensions(ctx);
                let nw = dim.w / 2.0 - 20.0;
                let nh = dim.h / 2.0 - 18.0;
                canvas.draw(
                    nr,
                    DrawParam::new()
                        .dest(pos + Vector2::new(-nw, -nh))
                        .color(c.to_font_color()),
                );
                canvas.draw(
                    nr,
                    DrawParam::new()
                        .dest(pos + Vector2::new(CARD_WIDTH + nw, CARD_HEIGHT + nh))
                        .rotation(f32::consts::PI)
                        .color(c.to_font_color()),
                );
            }
        }
        //graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), graphics::Rect::new(pos.x, pos.y, WIDTH, HEIGHT))?;
        Ok(())
    }
}

impl DrawCommand {
    fn get_z(&self) -> f32 {
        match *self {
            DrawCommand::Card { z, .. } => z,
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
