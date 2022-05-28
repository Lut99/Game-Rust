/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   28 May 2022, 17:09:25
 * Last edited:
 *   28 May 2022, 17:10:28
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the Buffers, Images and MemoryPool module.
**/

/// Contains the buffer definitions
pub mod buffers;
/// Contains the pool itself
pub mod pool;


// Bring some stuff into the module scope
pub use buffers::Buffer;
pub use pool::{Error, MemoryPool as Pool};
