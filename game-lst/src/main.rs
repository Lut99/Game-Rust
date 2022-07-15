/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Apr 2022, 13:01:51
 * Last edited:
 *   15 Jul 2022, 18:40:11
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the executable that lists all GPUs.
**/

use clap::{Parser, Subcommand};
use num_format::{Locale, ToFormattedString};

use game_gfx::RenderSystem;


/***** ARGUMENTS *****/
/// Defines the arguments for the list tool
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(subcommand)]
    action : Action,
}



/// Defines the actions for this executable.
#[derive(Subcommand)]
enum Action {
    /// Shows a list of all GPUs found by the Vulkan backend
    #[clap(name = "gpus", about = "Shows a list of all GPUs found by the Vulkan backend.")]
    Devices {
        /// Whether or not to search for GPUs with extra debug capabilities
        #[clap(short, long, help = "If given, requires that supported GPUs also support extra debug capabilities.")]
        debug  : bool,
        /// Whether to display additional memory information or not.
        #[clap(short, long, help = "If given, shows detailled Vulkan memory statistics about each GPU.")]
        memory : bool,
    },

    /// Shows a list of all monitors and their video modes found by the winit backend
    #[clap(name = "monitors", about = "Shows a list of all monitors found by the winit backend")]
    Monitors {
        /// Whether or not to display video modes for each monitor
        #[clap(short, long, help = "If given, shows the supported video modes for each monitor (relevant for eclusive fullscreen)")]
        video_modes : bool,
    },
}





/***** ENTRYPOINT *****/
fn main() {
    // Parse the CLI
    let args = Arguments::parse();

    // We don't setup a logger due to it not really being necessary for a tool this small

    // Switch on the action
    match args.action {
        Action::Devices{ debug, memory } => {
            // Simply call the function
            let gpus = match RenderSystem::list_gpus(debug) {
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
                    if memory {
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
                    if memory {
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
        },

        Action::Monitors{ video_modes } => {
            // Simply call the function
            let monitors = match RenderSystem::list_monitors() {
                Ok(monitors) => monitors,
                Err(err)     => {
                    eprintln!("Could not list monitors: {}", err);
                    std::process::exit(1);
                }
            };

            // Print 'em
            println!();
            println!("Found monitors:");
            if !monitors.is_empty() {
                for info in monitors {
                    println!(" - Monitor {}: {}", info.index, info.name);
                    if video_modes {
                        println!("    - Supported video modes:");
                        for (i, mode) in info.video_modes.into_iter().enumerate() {
                            println!("       {}) {}", i, mode);
                        }
                    }
                }
            } else {
                println!("   <no monitors>");
            }
        
            println!();
            println!("To use a monitor, edit settings.json and set 'monitor' in 'window_mode' to the index of the monitor you'd like to use.");
        
            // Done
            println!();
            println!();
        },
    };
}
