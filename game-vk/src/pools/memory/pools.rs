/* POOLS.rs
 *   by Lut99
 *
 * Created:
 *   25 Jun 2022, 18:04:08
 * Last edited:
 *   26 Jun 2022, 14:14:47
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The pools that we use to allocate new bits of memory.
 *   
 *   The algorith for the BlockPool is taken from the Rasterizer project
 *   (https://github.com/Lut99/Rasterizer).
**/

use std::fmt::{Debug, Formatter, Result as FResult};
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use ash::vk;

pub use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::{MemoryPropertyFlags, MemoryRequirements};
use crate::device::Device;
use crate::pools::memory::block::MemoryBlock;
use crate::pools::memory::spec::MemoryPool;


/***** HELPER MACROS *****/
/// Computes the aligned version of a pointer
macro_rules! align {
    ($ptr:expr,$align:expr) => {
        if $align != 0 {
            if ($align & ($align - 1)) != 0 { panic!("Given alignment '{}' is not a power of two", $align); }
            ($ptr + ($align - 1)) & ((!$align) + 1)
        } else {
            $ptr
        }
    };
}





/***** HELPER STRUCTS *****/
/// Represents a piece of a MemoryBlock that is used for something. It's implemented as a (doubly) linked list.
struct UsedBlock {
    /// The start of the block.
    offset : usize,
    /// The size of the block (in bytes).
    size   : usize,

    /// The next block in the list.
    next : Option<Rc<Self>>,
    /// The previous block in the list.
    prev : Option<Rc<Self>>,
}

impl UsedBlock {
    /// Convenience constructor for the UsedBlock.
    /// 
    /// # Arguments
    /// - `offset`: The start of the UsedBlock (as a byte offset).
    /// - `size`: The size of the UsedBlock (in bytes).
    /// - `next`: An optional previous block in the list.
    /// - `prev`: An optional next block in the list.
    /// 
    /// # Returns
    /// A new instance of the UsedBlock, already wrapped in a reference-counting pointer.
    #[inline]
    fn new(offset: usize, size: usize, next: Option<Rc<Self>>, prev: Option<Rc<Self>>) -> Rc<Self> {
        Rc::new(Self {
            offset,
            size,

            next,
            prev,
        })
    }



    /// Inserts a new block directly before this one, properly setting links and such.
    /// 
    /// # Arguments
    /// - `this`: The "self" to change.
    /// - `block`: The new UsedBlock to insert.
    /// 
    /// # Returns
    /// Nothing, but does set links internally in this and neighbouring blocks to insert the new block.
    fn insert_before(this: &mut Rc<Self>, mut block: Rc<UsedBlock>) {
        // If there is a next block, link the new one to that first
        if let Some(prev) = Rc::get_mut(this).expect("Could not get this as muteable reference").prev.as_mut() {
            Rc::get_mut(prev).expect("Could not get prev as muteable reference").next        = Some(block.clone());
            Rc::get_mut(&mut block).expect("Could not get block as muteable reference").prev = Some(prev.clone());
        }

        // Set it as the neighbour before
        Rc::get_mut(&mut block).expect("Could not get block as muteable reference").next = Some(this.clone());
        Rc::get_mut(this).expect("Could not get this as muteable reference").prev        = Some(block);
    }

    /// Inserts a new block directly after this one, properly setting links and such.
    /// 
    /// # Arguments
    /// - `this`: The "self" to change.
    /// - `block`: The new UsedBlock to insert.
    /// 
    /// # Returns
    /// Nothing, but does set links internally in this and neighbouring blocks to insert the new block.
    fn insert_after(this: &mut Rc<Self>, mut block: Rc<UsedBlock>) {
        // If there is a next block, link the new one to that first
        if let Some(next) = Rc::get_mut(this).expect("Could not get this as muteable reference").next.as_mut() {
            Rc::get_mut(next).expect("Could not get next as muteable reference").prev        = Some(block.clone());
            Rc::get_mut(&mut block).expect("Could not get block as muteable reference").next = Some(next.clone());
        }

        // Set it as the neighbour before
        Rc::get_mut(&mut block).expect("Could not get block as muteable reference").prev = Some(this.clone());
        Rc::get_mut(this).expect("Could not get this as muteable reference").next        = Some(block);
    }

    /// Removes this block from the chain, fixing links around it.
    /// 
    /// # Arguments
    /// - `this`: The "self" to remove.
    /// 
    /// # Returns
    /// Nothing, but does set links internally in this and neighbouring blocks to insert the new block.
    fn remove(this: &mut Rc<Self>) {
        // If there is a previous block, fix that
        if let Some(mut prev) = Rc::get_mut(this).expect("Could not get this as muteable reference").prev.take() {
            Rc::get_mut(&mut prev).expect("Could not get prev as muteable reference").next = this.next.clone();
        }
        // If there is a next block, fix that
        if let Some(mut next) = Rc::get_mut(this).expect("Could not get this as muteable reference").next.take() {
            Rc::get_mut(&mut next).expect("Could not get next as muteable reference").prev = this.prev.clone();
        }
    }
}

impl Debug for UsedBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // We always print the entire chain, no matter where you start
        if let Some(prev) = &self.prev { write!(f, "{:?}, ", prev); }
        write!(f, "UsedBlock{{offset={}, size={}}}", self.offset, self.size);
        if let Some(next) = &self.next { write!(f, ", {:?}", next); }
        Ok(())
    }
}





/***** LIBRARY *****/
/// A LinearPool uses a very fast memory allocation algorithm, but wastes space because freed blocks cannot be re-used until the pool is reset. Additionally, this type of pool only supports one type of memory.
pub struct LinearPool {
    /// The Device where the LinearPool lives.
    device : Rc<Device>,
    /// The single memory block used in the linear pool.
    block  : Option<MemoryBlock>,

    /// The pointer that determines up to where we already gave to memory blocks.
    pointer  : usize,
    /// The size (in bytes) of the LinearPool.
    capacity : usize,
}

impl LinearPool {
    /// Constructor for the LinearPool.
    /// 
    /// Note that memory will be allocated lazily.
    /// 
    /// # Arguments
    /// - `capacity`: The size (in bytes) of the pool.
    /// 
    /// # Returns
    /// A new LinearPool instance, already wrapped in an Arc and a RwLock.
    #[inline]
    pub fn new(device: Rc<Device>, capacity: usize) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            device,
            block : None,

            pointer : 0,
            capacity,
        }))
    }



    /// Frees the internal memory block.
    /// 
    /// This is useful if you want to repurpose the LinearPool for a different kind of memory.
    /// 
    /// # Results
    /// Nothing, but does free the internal block so it will be allocated again on the next allocate() call.
    #[inline]
    pub fn release(&mut self) {
        self.block = None;
    }



    /// Returns the used size in the LinearPool.
    #[inline]
    pub fn size(&self) -> usize { self.pointer }

    /// Returns the total size of the LinearPool.
    #[inline]
    pub fn capacity(&self) -> usize { self.capacity }
}

impl MemoryPool for LinearPool {
    /// Returns a newly allocated area of (at least) the requested size.
    /// 
    /// # Arguments
    /// - `reqs`: The memory requirements of the new memory block.
    /// - `props`: Any desired memory properties for this memory block.
    /// 
    /// # Returns
    /// A tuple with the VkDeviceMemory where the new block of memory is allocated on `.0`, and the index in this memory block on `.1`.
    /// 
    /// # Errors
    /// This function errors if the MemoryPool failed to allocate new memory.
    fn allocate(&mut self, reqs: &MemoryRequirements, props: MemoryPropertyFlags) -> Result<(vk::DeviceMemory, usize), Error> {
        // Check whether we have a block of memory already
        let memory: vk::DeviceMemory = match self.block.as_ref() {
            Some(block) => {
                // Make sure the requirements & properties are satisfied
                if !reqs.types.check(block.mem_type()) { panic!("LinearPool is allocated for device memory type {}, but new allocation only supports {}", block.mem_type(), reqs.types); }
                if !block.mem_props().check(props) { panic!("LinearPool is allocated for device memory type {} which supports the properties {}, but new allocation requires {}", block.mem_type(), block.mem_props(), props); }
                block.vk()
            },

            None => {
                // Allocate a new block
                let block = MemoryBlock::allocate(self.device.clone(), &reqs, props)?;
                let memory = block.vk();
                self.block = Some(block);
                memory
            },
        };

        // Compute the alignment requirements based on the current pointer
        let pointer = if reqs.align != 0 {
            if (reqs.align & (reqs.align - 1)) != 0 { panic!("Given alignment '{}' is not a power of two", reqs.align); }
            (self.pointer + (reqs.align - 1)) & ((!reqs.align) + 1)
        } else {
            self.pointer
        };

        // Check if that leaves us with enough space
        if reqs.size > self.capacity - pointer { return Err(Error::OutOfMemoryError{ req_size: reqs.size }); }

        // Advance the internal pointer and return the allocated one
        self.pointer = pointer + reqs.size;
        Ok((memory, pointer))
    }

    /// Frees an allocated bit of memory.
    /// 
    /// Note that not all types of pools may actually do anything with this. A LinearPool, for example, might deallocate but will never re-use that memory until reset anyway.
    /// 
    /// # Arguments
    /// - `pointer`: The pointer to the block that was allocated.
    /// 
    /// # Panics
    /// This function may panic if the given pointer was never allocated with this pool.
    #[inline]
    fn free(&mut self, _pointer: usize) {}

    /// Resets the memory pool back to its initial, empty state.
    #[inline]
    fn reset(&mut self) { self.pointer = 0; }



    /// Returns the device of the pool.
    #[inline]
    fn device(&self) -> &Rc<Device> { &self.device }

    /// Returns the used space in the pool.
    #[inline]
    fn size(&self) -> usize { self.pointer }

    /// Returns the total space in the pool.
    #[inline]
    fn capacity(&self) -> usize { self.capacity }
}



/// A BlockPool uses a more complicated and slow allocation algorithm, but saves space because it does reuse freed blocks. This specific type of pool only supports one type of memory.
pub struct BlockPool {
    /// The Device where the BlockPool lives.
    device : Rc<Device>,
    /// The single memory block used in this pool.
    block  : MemoryBlock,

    /// Pointer to the start of the linked list.
    first : Option<Rc<UsedBlock>>,
    /// Pointer to the end of the linked list.
    last  : Option<Rc<UsedBlock>>,
    /// The used space in the BlockPool.
    size  : usize,
}

impl BlockPool {
    /// Constructor for the BlockPool.
    /// 
    /// # Arguments
    /// - `block`: The already allocated MemoryBlock. If you have yet to allocate one, check `MemoryBlock::allocate()`.
    /// 
    /// # Returns
    /// A new BlockPool instance, already wrapped in an Arc and a RwLock.
    #[inline]
    pub fn new(device: Rc<Device>, block: MemoryBlock) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            device,
            block,

            first : None,
            last  : None,
            size : 0,
        }))
    }
}

impl MemoryPool for BlockPool {
    /// Returns a newly allocated area of (at least) the requested size.
    /// 
    /// # Arguments
    /// - `reqs`: The memory requirements of the new memory block.
    /// - `props`: Any desired memory properties for this memory block.
    /// 
    /// # Returns
    /// A tuple with the VkDeviceMemory where the new block of memory is allocated on `.0`, and the index in this memory block on `.1`.
    /// 
    /// # Errors
    /// This function errors if the MemoryPool failed to allocate new memory.
    fn allocate(&mut self, reqs: &MemoryRequirements, props: MemoryPropertyFlags) -> Result<(vk::DeviceMemory, usize), Error> {
        // Make sure the requirements & properties are satisfied
        if !reqs.types.check(self.block.mem_type()) { panic!("BlockPool is allocated for device memory type {}, but new allocation only supports {}", self.block.mem_type(), reqs.types); }
        if !self.block.mem_props().check(props) { panic!("BlockPool is allocated for device memory type {} which supports the properties {}, but new allocation requires {}", self.block.mem_type(), self.block.mem_props(), props); }

        // Optimization: we can stop early if there is no more space
        if reqs.size > self.size { return Err(Error::OutOfMemoryError{ req_size: reqs.size }); }

        // Now, check if we have simply the space to add it after the last block.
        {
            // Compute the aligned pointer based on the last block
            let block_end: usize = self.last.as_ref().map(|b| b.offset + b.size).unwrap_or(0);
            let pointer = align!(block_end, reqs.align);

            // Check the size
            if pointer + reqs.size < self.block.mem_size() {
                // Allocate a new block and return it
                let new = UsedBlock::new(pointer, reqs.size, None, self.last.as_ref().map(|b| b.clone()));
                if let Some(last) = self.last.as_mut() {
                    UsedBlock::insert_after(last, new.clone());
                }
                self.last = Some(new);
                self.size += reqs.size;
                return Ok((self.block.vk(), pointer));
            }
        }

        // If there was no space after the last block, iterate to find the first free space
        let mut this: Option<&mut Rc<UsedBlock>> = self.first.as_mut();
        while this.is_some() {
            // Get the block
            let block: &mut Rc<UsedBlock> = this.unwrap();

            // Check if there is space to insert the block before this one
            let block_end: usize = block.prev.as_ref().map(|b| b.offset + b.size).unwrap_or(0);
            let pointer = align!(block_end, reqs.align);
            if reqs.size <= block.offset - pointer {
                // There is; add a new block before this one
                let new = UsedBlock::new(pointer, reqs.size, None, self.last.as_ref().map(|b| b.clone()));
                UsedBlock::insert_before(block, new.clone());
                if new.prev.is_none() {
                    self.first = Some(new);
                }
                self.size += reqs.size;
                return Ok((self.block.vk(), pointer));
            }

            // Otherwise, go to the next block
            this = Rc::get_mut(block).expect("Could not get block as muteable reference").next.as_mut();
        }

        // If we've reached the end of the chain and allocated nothing, then no memory available
        Err(Error::OutOfMemoryError{ req_size: reqs.size })
    }

    /// Frees an allocated bit of memory.
    /// 
    /// Note that not all types of pools may actually do anything with this. A LinearPool, for example, might deallocate but will never re-use that memory until reset anyway.
    /// 
    /// # Arguments
    /// - `pointer`: The pointer to the block that was allocated.
    /// 
    /// # Panics
    /// This function may panic if the given pointer was never allocated with this pool.
    #[inline]
    fn free(&mut self, pointer: usize) {
        // Search for the block with the given pointer
        let mut this: Option<&mut Rc<UsedBlock>> = self.first.as_mut();
        while this.is_some() {
            // Get the block
            let block: &mut Rc<UsedBlock> = this.unwrap();

            // Check the pointer
            if block.offset == pointer {
                // Remove it
                UsedBlock::remove(block);
                self.size -= block.size;
                return;
            }

            // Otherwise, go to the next block
            this = Rc::get_mut(block).expect("Could not get block as muteable reference").next.as_mut();
        }

        // Didn't find the block!
        panic!("Given pointer '{:#X}' was not allocated with this pool", pointer);
    }

    /// Resets the memory pool back to its initial, empty state.
    #[inline]
    fn reset(&mut self) {
        // Clear the list
        self.first = None;
        self.last  = None;

        // Reset the size
        self.size = 0;
    }



    /// Returns the device of the pool.
    #[inline]
    fn device(&self) -> &Rc<Device> { &self.device }

    /// Returns the used space in the pool.
    #[inline]
    fn size(&self) -> usize { self.size }

    /// Returns the total space in the pool.
    #[inline]
    fn capacity(&self) -> usize { self.block.mem_size() }
}
