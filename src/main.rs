
#[cfg(feature = "profiling")] extern crate cpuprofiler;

extern crate env_logger;
extern crate ggez;
#[macro_use] extern crate log;
extern crate rand;
extern crate sdl2;

mod gamestates;

mod ai;
mod cs;
mod game;
mod resources;
mod rules;
mod types;
mod utils;

use std::env;

use ggez::conf;

use gamestates::GameWrapper;

const SHENZHEN_PATH: &str =".local/share/Steam/SteamApps/common/SHENZHEN IO/Content/";


fn main() {
    env_logger::init();

    let c = conf::Conf {
        window_mode: conf::WindowMode::default()
            .dimensions(1280, 806),
        window_setup: conf::WindowSetup::default().title("Solitaire Clone"),
        backend: conf::Backend::OpenGL{major: 3, minor: 2},
    };

    #[cfg(feature = "profiling")]
    use cpuprofiler::PROFILER;
    #[cfg(feature = "profiling")]
    PROFILER.lock().unwrap().start("solitaire.profile").unwrap();

    let ctx = &mut ggez::Context::load_from_conf("solitaire_clone", "Swampsoft Games", c).unwrap();

    ctx.filesystem.mount(&env::home_dir().unwrap().join(SHENZHEN_PATH), true);

    let mut state = GameWrapper::new(ctx).unwrap();
    loop {
        if let GameWrapper::Quit = state {
            break
        }
        state = state.run(ctx).unwrap();
    }

    #[cfg(feature = "profiling")]
    PROFILER.lock().unwrap().stop().unwrap();
}
