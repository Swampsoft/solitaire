
#[cfg(feature = "profiling")]
extern crate cpuprofiler;

extern crate ggez;
extern crate rand;

mod gamestates;

mod bbox;
mod button;
mod cards;
mod cardstack;
mod resources;
mod rules;
mod table;

use std::env;

use ggez::conf;

use gamestates::GameWrapper;

const SHENZHEN_PATH: &str =".local/share/Steam/SteamApps/common/SHENZHEN IO/Content/";



fn main() {
    let c = conf::Conf {
        window_mode: conf::WindowMode::default()
            .dimensions(1280, 806)
            .borderless(true),
        window_setup: conf::WindowSetup::default().title("SHENZHEN IO Solitaire Clone"),
        backend: conf::Backend::OpenGL{major: 3, minor: 2},
    };

    #[cfg(feature = "profiling")]
    use cpuprofiler::PROFILER;
    #[cfg(feature = "profiling")]
    PROFILER.lock().unwrap().start("solitaire.profile").unwrap();

    let ctx = &mut ggez::Context::load_from_conf("solitaire", "Martin Billinger", c).unwrap();

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
