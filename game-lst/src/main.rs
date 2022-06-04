/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Apr 2022, 13:01:51
 * Last edited:
 *   04 Jun 2022, 12:23:32
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the executable that lists all GPUs.
**/

use clap::Parser;
use num_format::{Locale, ToFormattedString};

use game_gfx::RenderSystem;


/***** ARGUMENTS *****/
/// Defines the arguments for the list tool
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Whether or not to seRch for GPUs with extra debug capabilities
    #[clap(short, long, help = "If given, requires that supported GPUs also support extra debug capabilities.")]
    pub debug : bool,
    /// Whether to display additional memory information or not.
    #[clap(short, long, help = "If given, shows detailled Vulkan memory statistics about each GPU.")]
    pub memory: bool,
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
        for info in gpus.0 {
            println!(" - Device {}: {} ({})", info.index, info.name, info.kind);
            if args.memory {
                println!("    - Supported memory heaps:");
                for (i, heap) in info.mem_props.heaps.into_iter().enumerate() {
                    println!("       {}) Heap of {} bytes{}", i, heap.size.to_formatted_string(&Locale::en), if !heap.props.is_empty() { format!(" ({})", heap.props) } else { String::new() });
                }
                println!("    - Supported memory types:");
                for (i, mem_type) in info.mem_props.types.into_iter().enumerate() {
                    println!("       {}) Type matching heap {}{}", i, mem_type.heap_index, if !mem_type.props.is_empty() { format!(": {}", mem_type.props) } else { String::new() });
                }
            }
        }
    } else {
        println!("   <no devices>")
    }
    
    println!();
    println!("Unsupported GPUs:");
    if !gpus.1.is_empty() {
        for info in gpus.1 {
            println!(" - Device {}: {} ({})", info.index, info.name, info.kind);
            if args.memory {
                println!("    - Supported memory heaps:");
                for (i, heap) in info.mem_props.heaps.into_iter().enumerate() {
                    println!("       {}) Heap of {} bytes{}", i, heap.size.to_formatted_string(&Locale::en), if !heap.props.is_empty() { format!(" ({})", heap.props) } else { String::new() });
                }
                println!("    - Supported memory types:");
                for (i, mem_type) in info.mem_props.types.into_iter().enumerate() {
                    println!("       {}) Type matching heap {}{}", i, mem_type.heap_index, if !mem_type.props.is_empty() { format!(": {}", mem_type.props) } else { String::new() });
                }
            }
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
