/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 16:16:04
 * Last edited:
 *   03 Jul 2022, 11:14:10
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Entrypoint to the Buffers, Images and MemoryPool module.
**/

/// Contains common definitions for the MemoryPool specifically.
pub mod spec;
/// Contains a memory block wrapper.
pub mod block;
/// Contains the pools with which we allocate the buffer.
pub mod pools;
/// Contains the buffer definitions
pub mod buffers;


// Bring some stuff into the module scope
pub use buffers::Buffer;
pub use spec::MemoryPool;
pub use pools::{Error, BlockPool, LinearPool, MetaPool};
