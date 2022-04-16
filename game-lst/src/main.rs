/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Apr 2022, 13:01:51
 * Last edited:
 *   16 Apr 2022, 13:11:06
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the executable that lists all GPUs.
**/

use clap::Parser;

use game_gfx::RenderSystem;


/***** ARGUMENTS *****/
/// Defines the arguments for the list tool
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Whether or not to search for GPUs with extra debug capabilities
    #[clap(short, long, help = "If given, requires that supported GPUs also support extra debug capabilities.")]
    pub debug : bool,
}





/***** ENTRYPOINT *****/
fn main() {
    // Parse the CLI
    let args = Arguments::parse();

    // We don't setup a logger due to it not really being necessary for a tool this small

    // Simply call the function
    if let Err(err) = RenderSystem::list(args.debug) {
        eprintln!("Could not list GPUs: {}", err);
        std::process::exit(1);
    }

    // Do a helpful print
    println!("To use a GPU, edit settings.json and set 'gpu' to the index of the GPU you'd like to use.");

    // Done
    println!();
}
