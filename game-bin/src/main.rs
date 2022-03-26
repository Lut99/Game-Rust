/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 12:11:47
 * Last edited:
 *   26 Mar 2022, 12:55:21
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the game executable.
**/

use std::fs::{self, File};
use std::path::PathBuf;

use log::{debug, error, info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TerminalMode, TermLogger, WriteLogger};

use game_cfg::{Action, Config};
use game_cfg::file::Settings;


/***** ENTRYPOINT *****/
fn main() {
    // Load the config
    let config = match Config::new() {
        Ok(config) => config,
        Err(err)   => { eprintln!("Could not load configuration: {}", err); std::process::exit(1); }
    };

    // Initialize the logger
    let install_log_path = PathBuf::from("./game-install.log");
    if let Err(err) = CombinedLogger::init(vec![
         TermLogger::new(config.log_level, Default::default(), TerminalMode::Mixed, ColorChoice::Auto),
         WriteLogger::new(LevelFilter::Debug, Default::default(), File::create(if let Action::Install{ .. } = &config.action { &install_log_path } else { &config.session_log_path }).unwrap_or_else(|err| panic!("Could not open log file '{}': {}", config.log_path.display(), err))),
    ]) {
        eprintln!("Could not load initialize loggers: {}", err);
        std::process::exit(1);
    }
    info!("Initializing Game-Rust {}", env!("CARGO_PKG_VERSION"));



    // Switch on the action!
    match config.action {
        Action::Install{ config_dir, log_dir } => {
            debug!("Executing subcommand: install");

            println!("Creating directories...");
            // Create the configuration directory if it does not yet exist
            if !config_dir.exists() {
                println!(" > Creating config directory '{}'...", config_dir.display());
                debug!("Creating config directory '{}'", config_dir.display());
                if let Err(err) = fs::create_dir_all(&config_dir) {
                    eprintln!("ERROR: Failed to create configuration directory: {}", err);
                    error!("Failed to create configuration directory: {}", err);
                    std::process::exit(1);
                }
            }
            // Create the log directory if it does not yet exist
            if !log_dir.exists() {
                println!(" > Creating log directory '{}'...", log_dir.display());
                debug!("Creating log directory '{}'", log_dir.display());
                if let Err(err) = fs::create_dir_all(&log_dir) {
                    eprintln!("ERROR: Failed to create log directory: {}", err);
                    error!("Failed to create log directory: {}", err);
                    std::process::exit(1);
                }
            }
            println!();

            println!("Generating settings...");
            println!(" > Selecting GPU...");
            debug!("Selecting GPU");
            // Use 0 for now
            let gpu: usize = 0;

            // Create a Settings file
            println!(" > Generating defaults...");
            let settings = Settings::default(config.log_path, gpu);

            // Write it to the location
            let settings_path = config_dir.join("settings.json");
            println!(" > Writing settings file '{}'...", settings_path.display());
            debug!("Exporting settings file to '{}'", settings_path.display());
            if let Err(err) = settings.write(settings_path) {
                eprintln!("ERROR: Could not write new settings file: {}", err);
                error!("Could not write new settings file: {}", err);
                std::process::exit(1);
            }
            println!();

            // Done!
            println!("Installation complete.");
            println!();
        },
        
        Action::List{} => {
            debug!("Executing subcommand: list");

            error!("'list' is not yet implemented.");
            std::process::exit(1);
        },
        
        Action::Run{ gpu } => {
            debug!("Executing subcommand: run");

            error!("'run' is not yet implemented.");
            std::process::exit(1);
        },
    }
}
