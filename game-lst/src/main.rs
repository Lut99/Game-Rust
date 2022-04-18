/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Apr 2022, 13:01:51
 * Last edited:
 *   18 Apr 2022, 15:52:07
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
    let gpus = match RenderSystem::list(args.debug) {
        Ok(gpus) => gpus,
        Err(err) => {
            eprintln!("Could not list GPUs: {}", err);
            std::process::exit(1);
        },
    };

    // Print the results
    println!();
    println!("Supported GPUs:");
    if !gpus.0.is_empty() {
        for (index, name, kind) in gpus.0 {
            println!(" - Device {}: {} ({})", index, name, kind);
        }
    } else {
        println!("   <no devices>")
    }
    
    println!();
    println!("Unsupported GPUs:");
    if !gpus.1.is_empty() {
        for (index, name, kind) in gpus.1 {
            println!(" - Device {}: {} ({})", index, name, kind);
        }
    } else {
        println!("   <no devices>")
    }

    println!();
    println!("To use a GPU, edit settings.json and set 'gpu' to the index of the GPU you'd like to use.");

    // Done
    println!();
    println!();
}
