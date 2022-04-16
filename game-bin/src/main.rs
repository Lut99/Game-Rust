/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 12:11:47
 * Last edited:
 *   16 Apr 2022, 12:29:37
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the game executable.
**/

use std::fs::File;
use std::str::FromStr;

use log::{error, info, LevelFilter};
use semver::Version;
use simplelog::{ColorChoice, CombinedLogger, TerminalMode, TermLogger, WriteLogger};
use winit::event_loop::{ControlFlow, EventLoop};

use game_cfg::Config;
use game_ecs::Ecs;
use game_gfx::RenderSystem;
use game_gfx::spec::RenderTargetStage;


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

    // Initialize the event loop
    let event_loop = EventLoop::new();

    // Initialize the entity component system
    let mut ecs = Ecs::default();

    // Initialize the render system
    let mut render_system = match RenderSystem::new(&mut ecs, "Game-Rust", Version::from_str(env!("CARGO_PKG_VERSION")).unwrap_or_else(|err| panic!("Could not parse environment variable CARGO_PKG_VERSION ('{}') as Version: {}", env!("CARGO_PKG_VERSION"), err)), "Game-Rust-Engine", Version::new(0, 1, 0), config.gpu, config.verbosity >= LevelFilter::Debug) {
        Ok(system) => system,
        Err(err)   => { error!("Could not initialize render system: {}", err); std::process::exit(1); }
    };

    // Initialize a new Window RenderTarget with the Triangle pipeline
    let window_info = game_gfx::window::CreateInfo {
        title : format!("Game-Rust v{} (Triangle Pipeline)", env!("CARGO_PKG_VERSION")),

        width  : 800,
        height : 600,

        image_count : 3,

        pipeline_info : triangle::CreateInfo::default(),
    };
    if let Err(err) = render_system.register::<
        game_gfx::window::Window<triangle::Pipeline>,
        game_gfx::window::CreateInfo<triangle::CreateInfo>,
    >(
        &event_loop,
        0,
        RenderTargetStage::MainLoop,
        window_info,
    ) {
        error!("Could not initialize render subsystem: {}", err);
        std::process::exit(1);
    }



    // Enter the main loop
    info!("Initialization complete; entering game loop...");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match render_system.handle_events(&event, control_flow) {
            Ok(flow) => flow,
            Err(err) => {
                error!("{}", err);
                ControlFlow::Exit
            }
        };
    });
}
