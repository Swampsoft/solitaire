extern crate ggez;
extern crate rand;

mod bbox;
mod button;
mod cards;
mod cardstack;
mod mainstate;
mod resources;
mod rules;

use std::env;

use ggez::conf;

use mainstate::MainState;

const SHENZHEN_PATH: &str =".local/share/Steam/SteamApps/common/SHENZHEN IO/";



fn main() {
    let c = conf::Conf {
        window_mode: conf::WindowMode::default()
            .dimensions(1280, 806)
            .borderless(true),
        window_setup: conf::WindowSetup::default().title("SHENZHEN IO Solitaire Clone"),
        backend: conf::Backend::OpenGL{major: 3, minor: 2},
    };

    let ctx = &mut ggez::Context::load_from_conf("solitaire", "Martin Billinger", c).unwrap();

    ctx.filesystem.mount(&env::home_dir().unwrap().join(SHENZHEN_PATH), true);

    let state = &mut MainState::new(ctx).unwrap();

    ggez::event::run(ctx, state).unwrap();
}
