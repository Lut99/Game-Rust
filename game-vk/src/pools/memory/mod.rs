/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 16:16:04
 * Last edited:
 *   10 Jul 2022, 15:32:47
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


// Define a prelude to import
pub mod prelude {
    pub use super::spec::{Buffer, HostBuffer, LocalBuffer, MemoryPool, TransferBuffer};
}

// Bring some stuff into the module scope
pub use buffers::{StagingBuffer, VertexBuffer};
pub use spec::{Buffer, HostBuffer, LocalBuffer, MappedMemory, MemoryPool, TransferBuffer};
pub use pools::{Error, BlockPool, LinearPool, MetaPool};
