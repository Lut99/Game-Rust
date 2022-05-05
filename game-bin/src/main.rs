/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 12:11:47
 * Last edited:
 *   05 May 2022, 12:20:41
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
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use game_cfg::Config;
use game_ecs::Ecs;
use game_gfx::RenderSystem;


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

    // Initialize a new Window RenderTarget
    let window_info = game_gfx::targets::window::CreateInfo {
        event_loop: &event_loop,

        title : format!("Game-Rust v{}", env!("CARGO_PKG_VERSION")),

        width  : 800,
        height : 600,

        image_count : 3,
    };
    let window = match render_system.register_target::<
        game_gfx::targets::window::Window,
        game_gfx::targets::window::CreateInfo,
    >(
        window_info,
    ) {
        Ok(window) => window,
        Err(err)   => {
            error!("Could not initialize RenderSystem Window: {}", err);
            std::process::exit(1);
        }
    };

    // Initialize a new Triangle RenderPipeline
    let triangle = match render_system.register_pipeline::<
        game_gfx::pipelines::TrianglePipeline,
        (),
    >(
        window,
        (),
    ) {
        Ok(triangle) => triangle,
        Err(err)     => {
            error!("Could not initialize RenderSystem TrianglePipeline: {}", err);
            std::process::exit(1);
        }
    };



    // Enter the main loop
    info!("Initialization complete; entering game loop...");
    event_loop.run(move |event, _, control_flow| {
        // Switch on the event type
        match event {
            | Event::WindowEvent{ window_id: _window_id, event } => {
                // Match the event again
                match event {
                    | WindowEvent::CloseRequested => {
                        // For now, we close on _any_ window close, but this should obviously be marginally more clever
                        *control_flow = ControlFlow::Exit;
                    },

                    // Ignore the others
                    _ => {}
                }
            },

            | Event::MainEventsCleared => {
                // Request a redraw of all internal windows
                render_system.get_target_as::<game_gfx::targets::window::Window>(window).request_redraw();
            },

            | Event::RedrawRequested(window_id) => {
                // Check if this concerns our Window
                let window_obj = render_system.get_target_as_mut::<game_gfx::targets::window::Window>(window);
                if window_id == window_obj.id() {
                    // Render the necessary pipelines
                    if let Err(err) = render_system.render(window, triangle) {
                        error!("Rendering Triangle Pipeline failed: {}", err);
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
            },

            // We do nothing for all other events
            _ => {}
        }
    });
}
