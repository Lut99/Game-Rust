//  MAIN.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 12:11:47
//  Last edited:
//    07 Aug 2022, 18:35:48
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the game executable.
// 

use std::fs::File;

use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TerminalMode, TermLogger, WriteLogger};

use game_cfg::Config;
use rust_ecs::Ecs;
use rust_win::spec::WindowInfo;
use game_evt::EventSystem;
use game_gfx::RenderSystem;
use game_gfx::spec::{AppInfo, VulkanInfo};


/***** ENTRYPOINT *****/
fn main() {
    // Load the config
    let config = match Config::new() {
        Ok(config) => config,
        Err(err)   => { eprintln!("Could not load configuration: {}", err); std::process::exit(1); }
    };

    // Initialize the logger
    if let Err(err) = CombinedLogger::init(vec![
         TermLogger::new(config.verbosity, Default::default(), TerminalMode::Mixed, ColorChoice::Auto),
         WriteLogger::new(LevelFilter::Debug, Default::default(), File::create(&config.files.log).unwrap_or_else(|err| panic!("Could not open log file '{}': {}", config.files.log.display(), err))),
    ]) {
        eprintln!("Could not load initialize loggers: {}", err);
        std::process::exit(1);
    }



    info!("Initializing Game-Rust {}", env!("CARGO_PKG_VERSION"));

    // Initialize the entity component system
    let ecs = Ecs::new(2048);
    // Initialize the event system
    let event_system = EventSystem::new(ecs.clone());

    // Initialize the render system
    let render_system = match RenderSystem::new(
        ecs.clone(),
        event_system.event_loop(),
        AppInfo::new(
            "Game-Rust",
            env!("CARGO_PKG_VERSION"),
            EventSystem::name(),
            EventSystem::version(),
        ),
        WindowInfo::new(
            "Game-Rust",
            config.window_mode,
        ),
        VulkanInfo {
            gpu   : config.gpu,
            debug : config.verbosity >= LevelFilter::Debug,
        },
    ) {
        Ok(system) => system,
        Err(err)   => { error!("Could not initialize render system: {}", err); std::process::exit(1); }
    };



    // Enter the main loop
    info!("Initialization complete; entering game loop...");
    event_system.game_loop(render_system);
}
