/* MAIN.rs
 *   by Lut99
 *
 * Created:
 *   16 Apr 2022, 13:01:51
 * Last edited:
 *   03 Jun 2022, 17:20:41
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the executable that lists all GPUs.
**/

use std::fmt::{Debug, Display, Formatter, Result as FResult, Write};
use std::cmp::Ord;
use std::ops::{Div, Rem};

use clap::Parser;
use num_traits::{NumCast, Unsigned};

use game_gfx::RenderSystem;


/***** HELPER FUNCTIONS *****/
/// Trait for PrettyBytes-compatible types.
trait PrettyBytes: Copy + Display + NumCast + Ord + Unsigned {
    /// Returns the PrettyBytesFormatter of this type.
    fn pretty(&self) -> PrettyBytesFormatter<Self>;
}

impl<T> PrettyBytes for T
where
    T: Copy + Display + NumCast + Ord + Unsigned
{
    /// Returns the PrettyBytesFormatter of this type.
    #[inline]
    fn pretty(&self) -> PrettyBytesFormatter<Self> { PrettyBytesFormatter(*self) }
}



/// Pretty-prints the given number as human-readable bytes.
struct PrettyBytesFormatter<T>(T);

impl<T: PrettyBytes> PrettyBytesFormatter<T> {
    /// Return the value as a simple byte count.
    #[inline]
    #[allow(dead_code)]
    fn b(&self) -> String {
        format!("{}B", self.0)
    }

    /// Return the value as a simple kibibyte count.
    #[inline]
    #[allow(dead_code)]
    fn kib(&self) -> String {
        format!("{}KiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0)
    }

    /// Return the value as a simple mibibyte count.
    #[inline]
    #[allow(dead_code)]
    fn mib(&self) -> String {
        format!("{}MiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0)
    }

    /// Return the value as a simple gibibyte count.
    #[inline]
    #[allow(dead_code)]
    fn gib(&self) -> String {
        format!("{}GiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0)
    }

    /// Return the value as a simple tibibyte count.
    #[inline]
    #[allow(dead_code)]
    fn tib(&self) -> String {
        format!("{}TiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0 / 1024.0)
    }

    /// Return the value as a simple pibibyte count.
    #[inline]
    #[allow(dead_code)]
    fn pib(&self) -> String {
        format!("{}PiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0 / 1024.0 / 1024.0)
    }
}

impl<T: PrettyBytes + Div + Rem> Debug for PrettyBytesFormatter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // First write to a string
        let mut s = String::with_capacity(6 + (num_traits::cast::cast::<T, f64>(self.0).unwrap().log10()) as usize);
        write!(f, "setyb ")?;

        // Write in pairs of three strings
        let mut first = true;
        let mut value: T = self.0;
        let thousand = num_traits::cast::cast::<u32, T>(1000).unwrap();
        while value > num_traits::cast::cast::<u32, T>(0).unwrap() {
            // Write the comma
            if first { first = false; }
            else { write!(s, ",")?; }

            // Get this part
            write!(s, "{}", value % thousand)?;
            value = value / thousand;
        }

        // Write the reversed string
        write!(f, "{}", s.chars().rev().collect::<String>())
    }
}

impl<T: PrettyBytes> Display for PrettyBytesFormatter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Determine the size of the bytes
        if      self.0 < num_traits::cast::cast::<u32, T>(1024).unwrap()                             { write!(f, "{}B"  , num_traits::cast::cast::<T, f64>(self.0).unwrap()) }
        else if self.0 < num_traits::cast::cast::<u32, T>(1024 * 1024).unwrap()                      { write!(f, "{}KiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0) }
        else if self.0 < num_traits::cast::cast::<u32, T>(1024 * 1024 * 1024).unwrap()               { write!(f, "{}MiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0) }
        else if self.0 < num_traits::cast::cast::<u64, T>(1024 * 1024 * 1024 * 1024).unwrap()        { write!(f, "{}GiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0) }
        else if self.0 < num_traits::cast::cast::<u64, T>(1024 * 1024 * 1024 * 1024 * 1024).unwrap() { write!(f, "{}TiB", num_traits::cast::cast::<T, f64>(self.0).unwrap() / 1024.0 / 1024.0 / 1024.0 / 1024.0) }
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
                    println!("       {}) Heap of {}{}", i, heap.size.pretty(), if !heap.props.is_empty() { format!(" ({})", heap.props) } else { String::new() });
                    println!("{}, {}, {}, {}, {}, {}", heap.size.b(), heap.size.kib(), heap.size.mib(), heap.size.gib(), heap.size.tib(), heap.size.pib());
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
                    println!("       {}) Heap of {}{}", i, heap.size.pretty(), if !heap.props.is_empty() { format!(" ({})", heap.props) } else { String::new() });
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
