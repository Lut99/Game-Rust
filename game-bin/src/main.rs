//  MAIN.rs
//    by Lut99
// 
//  Created:
//    26 Mar 2022, 12:11:47
//  Last edited:
//    06 Aug 2022, 16:29:35
//  Auto updated?
//    Yes
// 
//  Description:
//!   Entrypoint to the game executable.
// 

use std::fs::File;
use std::str::FromStr;

use log::{error, info, LevelFilter};
use semver::Version;
use simplelog::{ColorChoice, CombinedLogger, TerminalMode, TermLogger, WriteLogger};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use game_cfg::Config;
use rust_ecs::Ecs;
use game_gfx::RenderSystem;
use game_gfx::spec::{RenderPipelineId, RenderTargetId};


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
    let mut ecs = Ecs::new(2048);

    // Initialize the render system
    let mut render_system = match RenderSystem::new(
        ecs.clone(),
        "Game-Rust", Version::from_str(env!("CARGO_PKG_VERSION")).unwrap_or_else(|err| panic!("Could not parse environment variable CARGO_PKG_VERSION ('{}') as Version: {}", env!("CARGO_PKG_VERSION"), err)),
        "Game-Rust-Engine", Version::new(0, 1, 0),
        &event_loop,
        config.gpu,
        config.window_mode,
        2,
        config.verbosity >= LevelFilter::Debug
    ) {
        Ok(system) => system,
        Err(err)   => { error!("Could not initialize render system: {}", err); std::process::exit(1); }
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
                render_system.get_target_as::<game_gfx::targets::window::Window>(RenderTargetId::TriangleWindow).request_redraw();
            },

            | Event::RedrawRequested(window_id) => {
                // Check if this concerns our Window
                let window_obj = render_system.get_target_as_mut::<game_gfx::targets::window::Window>(RenderTargetId::TriangleWindow);
                if window_id == window_obj.id() {
                    // Render the necessary pipelines
                    if let Err(err) = render_system.render(RenderPipelineId::Triangle, RenderTargetId::TriangleWindow) {
                        error!("Rendering Triangle Pipeline failed: {}", err);
                        if let Err(err) = render_system.wait_for_idle() { error!("Could not wait for device to be idle: {}", err); }
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
            },

            // We do nothing for all other events
            _ => {}
        }

        // If we're about to exit, wait until the device is idle
        if *control_flow == ControlFlow::Exit {
            if let Err(err) = render_system.wait_for_idle() { error!("Could not wait for device to be idle: {}", err); }
        }
    });
}
