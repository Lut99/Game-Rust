/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Apr 2022, 13:01:51
 * Last edited:
 *   29 May 2022, 18:39:36
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the executable that lists all GPUs.
**/

use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::cmp::Ord;

use clap::Parser;
use num_traits::{Num, NumCast, Unsigned};

use game_gfx::RenderSystem;


/***** HELPER FUNCTIONS *****/
/// Pretty-prints the given number as human-readable bytes.
struct PrettyBytes<T>(T);

impl<T: NumCast + Ord + Unsigned> Debug for PrettyBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Write in pairs of three strings
        let mut first = true;
        let mut value: T = self.0;
        while value > num_traits::cast::cast::<u32, T>(0).unwrap() {
            // Write the comma
            if first { first = false; }
            else { write!(f, ",")?; }

            // Get this part
            write!(f, "{}", )
        }

        // Done
        Ok(())
    }
}

impl<T: NumCast + Ord + Unsigned> Display for PrettyBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Determine the size of the bytes
        if      self.0 < num_traits::cast::cast::<u32, T>(1024).unwrap()                             { write!(f, "{}B"  , num_traits::cast::cast::<T, f64>(self.0).unwrap()) }
        else if self.0 < num_traits::cast::cast::<u32, T>(1024 * 1024).unwrap()                      { write!(f, "{}KiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0) }
        else if self.0 < num_traits::cast::cast::<u32, T>(1024 * 1024 * 1024).unwrap()               { write!(f, "{}MiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0) }
        else if self.0 < num_traits::cast::cast::<u32, T>(1024 * 1024 * 1024 * 1024).unwrap()        { write!(f, "{}GiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0) }
        else if self.0 < num_traits::cast::cast::<u32, T>(1024 * 1024 * 1024 * 1024 * 1024).unwrap() { write!(f, "{}TiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0 / 1024.0) }
        else                                                                                         { write!(f, "{}PiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0 / 1024.0 / 1024.0) }
    }
}





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
                    println!("       {}) Heap of {} bytes{}", i, heap.size, if !heap.props.is_empty() { format!(" ({})", heap.props) } else { String::new() });
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
                    println!("       {}) Heap of {} bytes{}", i, heap.size, if !heap.props.is_empty() { format!(" ({})", heap.props) } else { String::new() });
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
