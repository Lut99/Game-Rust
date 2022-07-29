/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 12:11:47
 * Last edited:
 *   29 Jul 2022, 13:44:15
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

use game_cfg::Config;
use game_ecs::Ecs;
use game_evt::EventSystem;
use game_gfx::RenderSystem;
use game_win::WindowSystem;


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
    game_spc::register_components(&ecs);

    // Initialize the event system
    let event_system = EventSystem::new(ecs.clone());

    // Initialize the window system
    let window_system = WindowSystem::new(ecs.clone());

    // Initialize the render system
    let render_system = match RenderSystem::new(
        ecs.clone(),
        event_system.clone(),
        "Game-Rust", Version::from_str(env!("CARGO_PKG_VERSION")).unwrap_or_else(|err| panic!("Could not parse environment variable CARGO_PKG_VERSION ('{}') as Version: {}", env!("CARGO_PKG_VERSION"), err)),
        "Game-Rust-Engine", Version::new(0, 1, 0),
        config.gpu,
        window_system.clone(),
        config.window_mode,
        2,
        config.verbosity >= LevelFilter::Debug
    ) {
        Ok(system) => system,
        Err(err)   => { error!("Could not initialize render system: {}", err); std::process::exit(1); }
    };



    // Enter the main loop
    info!("Initialization complete; entering game loop...");
    {
        let register = move || {
            // Register system components
            render_system.register();
        };

        // Run the game loop
        EventSystem::game_loop(event_system);
    }
    // event_loop.run(move |event, _, control_flow| {
    //     // Switch on the event type
    //     match event {
    //         | Event::WindowEvent{ window_id: _window_id, event } => {
    //             // Match the event again
    //             match event {
    //                 | WindowEvent::CloseRequested => {
    //                     // For now, we close on _any_ window close, but this should obviously be marginally more clever
    //                     *control_flow = ControlFlow::Exit;
    //                 },

    //                 // Ignore the others
    //                 _ => {}
    //             }
    //         },

    //         | Event::MainEventsCleared => {
    //             // Request a redraw of all internal windows
    //             let ecs: Ref<Ecs> = ecs.borrow();
    //             let windows = ecs.list_component::<game_win::Window>();
    //             for window in windows.iter() {
    //                 window.window.request_redraw();
    //             }
    //         },

    //         | Event::RedrawRequested(_) => {
    //             // Redraw the pipeline
    //             if let Err(err) = render_system.render(RenderPipelineId::Triangle) {
    //                 error!("Rendering Triangle Pipeline failed: {}", err);
    //                 if let Err(err) = render_system.wait_for_idle() { error!("Could not wait for device to be idle: {}", err); }
    //                 *control_flow = ControlFlow::Exit;
    //                 return;
    //             }
    //         },

    //         // We do nothing for all other events
    //         _ => {}
    //     }

    //     // If we're about to exit, wait until the device is idle
    //     if *control_flow == ControlFlow::Exit {
    //         if let Err(err) = render_system.wait_for_idle() { error!("Could not wait for device to be idle: {}", err); }
    //     }
    // });
}
