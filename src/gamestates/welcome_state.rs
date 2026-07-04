use ggez::event::*;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use super::giveup_state::GiveupState;
use super::victory_state::VictoryState;

use game::Game;
use ggez::graphics::{Canvas, DrawParam, Drawable};
use ggez::input::mouse::MouseButton;
use resources::Resources;

pub struct WelcomeState {
    pub resources: Resources,
    pub move_on: bool,
    pub game: Game,
    pub ready: bool,
}

impl WelcomeState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(WelcomeState {
            resources: Resources::new(ctx)?,
            move_on: false,
            game: Game::new(),
            ready: false,
        })
    }
}

impl EventHandler for WelcomeState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.ready {
            // skip first frame because it has a super high delta-time
            self.ready = true;
        } else {
            let dt = ctx.time.delta().as_secs_f32();
            self.game.state.run_update(dt, &mut self.resources);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(&ctx.gfx, None);
        self.game
            .state
            .run_render(ctx, &mut self.resources, &mut canvas)?;

        let text = self
            .resources
            .get_text(ctx, "Click anywhere to start a new game.")?;
        let dim = text.dimensions(&ctx.gfx);
        let pos = Point2::from([640.0 - dim.w / 2.0, 403.0 - dim.h / 2.0]);
        canvas.draw(text, DrawParam::new().dest(pos));

        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult<()> {
        if !self.game.state.busy() {
            self.move_on = true;
            ctx.request_quit();
        }
        Ok(())
    }
}

impl From<VictoryState> for WelcomeState {
    fn from(old: VictoryState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            move_on: false,
            game: Game::new(),
            ready: true,
        }
    }
}

impl From<GiveupState> for WelcomeState {
    fn from(old: GiveupState) -> WelcomeState {
        WelcomeState {
            resources: old.resources,
            move_on: false,
            game: Game::new(),
            ready: true,
        }
    }
}
