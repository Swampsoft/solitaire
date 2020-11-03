#[cfg(feature = "profiling")]
extern crate cpuprofiler;

extern crate env_logger;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate rand;
extern crate rodio;

mod gamestates;

mod ai;
//mod custom_audio;
mod cs;
mod game;
mod resources;
mod rules;
mod types;
mod utils;

use std::env;

use ggez::{conf, ContextBuilder};

use gamestates::GameWrapper;

const SHENZHEN_PATH: &str = ".local/share/Steam/SteamApps/common/SHENZHEN IO/Content/";

fn main() {
    env_logger::init();

    let c = conf::Conf {
        window_mode: conf::WindowMode::default().dimensions(1280.0, 806.0),
        window_setup: conf::WindowSetup::default().title("Solitaire Clone"),
        backend: conf::Backend::OpenGL { major: 3, minor: 2 },
        ..conf::Conf::default()
    };

    #[cfg(feature = "profiling")]
    use cpuprofiler::PROFILER;
    #[cfg(feature = "profiling")]
    PROFILER.lock().unwrap().start("solitaire.profile").unwrap();

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("solitaire_clone", "Swampsoft Games")
        .conf(c)
        .add_resource_path(resource_dir)
        .add_resource_path(env::home_dir().unwrap().join(SHENZHEN_PATH))
        .build()
        .unwrap();

    let mut state = GameWrapper::new(&mut ctx).unwrap();
    loop {
        if let GameWrapper::Quit = state {
            break;
        }
        state = state.run(&mut ctx, &mut event_loop).unwrap();
    }

    #[cfg(feature = "profiling")]
    PROFILER.lock().unwrap().stop().unwrap();
}
