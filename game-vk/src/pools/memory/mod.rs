/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 16:16:04
 * Last edited:
 *   25 Jun 2022, 16:17:02
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the Buffers, Images and MemoryPool module.
**/

/// Contains the buffer definitions
pub mod buffers;
/// Contains the pool trait that is the frontend for multiple types of memory pools
pub mod pool;
/// Contains the cheap but inflexible linear pool
pub mod linear_pool;
/// Contains the expensive but flexible block pool
pub mod block_pool;


// // Bring some stuff into the module scope
// pub use buffers::Buffer;
// pub use pool::{Error, MemoryPool as Pool};
