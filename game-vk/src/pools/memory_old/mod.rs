/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:09:25
 * Last edited:
 *   04 Jun 2022, 15:30:00
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the Buffers, Images and MemoryPool module.
**/

/// Contains the buffer definitions
pub mod buffers;
/// Defines the allocators used
pub mod allocators;
/// Contains the pool itself
pub mod pool;


// Bring some stuff into the module scope
pub use buffers::Buffer;
pub use pool::{Error, MemoryPool as Pool};
