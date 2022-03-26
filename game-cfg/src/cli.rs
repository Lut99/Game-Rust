/* CLI.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 10:55:40
 * Last edited:
 *   26 Mar 2022, 12:54:51
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the part of the config that parses the command-line
 *   interface.
**/

use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;


/***** CONSTANTS *****/
/// The default location of the log files
const DEFAULT_LOG_DIR_PATH: &str = "./logs/";

// Constants that we lazily load
lazy_static! {
    /// The default location of the config files
    static ref DEFAULT_CONFIG_DIR_PATH: String = format!("{}", dirs_2::document_dir().expect("Could not get standard user documents directory").join("Game/config/").display());
    
    /// The default location of the settings file
    static ref DEFAULT_SETTINGS_PATH: String = format!("{}settings.json", *DEFAULT_CONFIG_DIR_PATH);
}





/***** ARGUMENT STRUCTS *****/
/// Defines the command-line part of the Config struct.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    /// The location of the settings file
    #[clap(short, long, default_value = &DEFAULT_SETTINGS_PATH, help = "The location of the settings file.")]
    pub(crate) settings_path : PathBuf,
    /// The location of the logging file
    #[clap(long, help = "The location of the logging file for the client. Note that any occurence of '%TODAY%' will be replaced with the current date, and any occurence of '%NOW%' will be replaced by the current time.")]
    pub(crate) log_path      : Option<PathBuf>,
    /// The verbosity of the logging (the CLI-part, at least)
    #[clap(short, long, help = "The verbosity of the terminal logging.")]
    pub(crate) log_level     : Option<LevelFilter>,

    /// Defined the subcommand that is run
    #[clap(subcommand)]
    pub(crate) subcommand : ArgumentSubcommand,
}



/// Talks about the subcommands for the config.
#[derive(Debug, Parser)]
pub(crate) enum ArgumentSubcommand {
    /// 'Installs' the executable by creating the required files and folders.
    #[clap(name = "install", about = "Installs the required files and folders on this machine.")]
    Install {
        /// The location of the game's config files
        #[clap(short, long, default_value = &DEFAULT_CONFIG_DIR_PATH, help = "The path of the folder where all configuration files will be installed. Note that if you choose anything other than the default location, you will have to start the game with the '--settings-path' option.")]
        config_dir : PathBuf,
        
        /// The location of the game's log files
        #[clap(short, long, default_value = &DEFAULT_LOG_DIR_PATH, help = "The path of the folder where all log files will be located.")]
        log_dir : PathBuf,  
    },

    /// A subcommand that returns a list of GPUs to select.
    #[clap(name = "list", about = "Lists all GPUs that are found by the internal grapics backend.")]
    List {},

    /// No subcommand is used; just run the game
    #[clap(name = "run", about = "Runs the game normally.")]
    Run {
        /// Overrides the automatic search for a GPU.
        #[clap(long, help = "Overrides the setting's selected GPU with the GPU at the given index. To see which GPUs are available under which indices, refer to the 'list' subcommand.")]
        gpu : Option<usize>,
    },
}
