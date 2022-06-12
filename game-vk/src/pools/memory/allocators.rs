/* ALLOCATORS.rs
 *   by Lut99
 *
 * Created:
 *   04 Jun 2022, 15:29:44
 * Last edited:
 *   12 Jun 2022, 13:20:18
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the allocators used in the MemoryPool.
**/

use game_utl::traits::AsAny;

pub(crate) use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::MemoryAllocatorKind;


/***** AUXILLARY STRUCTS *****/
/// A single block of free memory within the free list.
struct FreeBlock {
    
}





/***** LIBRARY TRAIT *****/
/// The MemoryAllocator trait, which is the globalized interface to each memory allocator.
pub(crate) trait MemoryAllocator: AsAny {
    /// Allocates a new piece of memory in the area managed by the allocator.
    /// 
    /// Doesn't really allocate it, but does reserve space for it internally and returns where this area may be created.
    /// 
    /// # Arguments
    /// - `align`: The bytes on which to align for the linear allocator. Must be a multiple of two.
    /// - `size`: The size of the area to allocate.
    /// 
    /// # Returns
    /// The "pointer" (index) in the area that this allocator manages that has been reserved for the new block.
    /// 
    /// # Errors
    /// This function may error if the block could not be allocated. In general, this would be because of not enough (continious) memory available.
    fn allocate(&mut self, align: usize, size: usize) -> Result<usize, Error>;

    /// Returns the type of this MemoryAllocator.
    fn kind(&self) -> MemoryAllocatorKind;
    /// Returns the space used in the area managed by this MemoryAllocator.
    fn size(&self) -> usize;
    /// Returns the total capacity of the area managed by this MemoryAllocator.
    fn capacity(&self) -> usize;
}





/***** ALLOCATORS *****/
/// A simple allocator optimised for allocating many blocks and then throwing them away again.
pub(crate) struct LinearAllocator {
    /// Some ID used to distinguish multiple LinearAllocators.
    id : u64,

    /// The current pointer in the area managed by this allocator.
    pointer  : usize,
    /// The total size of the area managed by this allocator.
    capacity : usize,
}

impl LinearAllocator {
    /// Constructor for the LinearAllocator.
    /// 
    /// # Arguments
    /// - `id`: The identifier of this allocator, so it may be distinguished and reset as a whole.
    /// - `size`: The size of the area which this allocator manages.
    #[inline]
    pub(crate) fn new(id: u64, size: usize) -> Self {
        Self {
            id,

            pointer  : 0,
            capacity : size,
        }
    }



    /// Resets the LinearAllocator to be completely empty again.
    #[inline]
    pub(crate) fn reset(&mut self) { self.pointer = 0; }



    /// Returns the ID of the LinearAllocator.
    #[inline]
    pub(crate) fn id(&self) -> u64 { self.id }
}

impl MemoryAllocator for LinearAllocator {
    /// Allocates a new piece of memory in the area managed by the allocator.
    /// 
    /// Doesn't really allocate it, but does reserve space for it internally and returns where this area may be created.
    /// 
    /// # Arguments
    /// - `align`: The bytes on which to align for the linear allocator. Must be a multiple of two.
    /// - `size`: The size of the area to allocate.
    /// 
    /// # Returns
    /// The "pointer" (index) in the area that this allocator manages that has been reserved for the new block.
    /// 
    /// # Errors
    /// This function may error if the block could not be allocated. In general, this would be because of not enough (continious) memory available.
    fn allocate(&mut self, align: usize, size: usize) -> Result<usize, Error> {
        // Align the internal pointer first
        let pointer = if align != 0 {
            if (align & (align - 1)) != 0 { panic!("Given alignment '{}' is not a power of two", align); }
            (self.pointer + (align - 1)) & ((!align) + 1)
        } else {
            self.pointer
        };

        // Check if the space left behind the pointer is enough
        if self.capacity - pointer > size { return Err(Error::OutOfMemoryError{ req_size: size }); }

        // Get the pointer, then increment it
        let result = pointer;
        self.pointer = pointer + size;

        // Done
        Ok(result)
    }



    /// Returns the type of this MemoryAllocator.
    #[inline]
    fn kind(&self) -> MemoryAllocatorKind { MemoryAllocatorKind::Linear(self.id) }

    /// Returns the space used in the area managed by this MemoryAllocator.
    #[inline]
    fn size(&self) -> usize { self.pointer }

    /// Returns the total capacity of the area managed by this MemoryAllocator.
    #[inline]
    fn capacity(&self) -> usize { self.capacity }
}



/// A more complex allocator that tries to find free space in previously freed blocks.
pub(crate) struct DenseAllocator {
    /// A list of all free blocks within the DenseAllocator.
    free_list : FreeBlock,
}

impl DenseAllocator {
    /// Constructor for the DenseAllocator.
    /// 
    /// # Arguments
    /// - 
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            free_list : FreeBlock{},
        }
    }
}

impl MemoryAllocator for DenseAllocator {
    /// Allocates a new piece of memory in the area managed by the allocator.
    /// 
    /// Doesn't really allocate it, but does reserve space for it internally and returns where this area may be created.
    /// 
    /// # Arguments
    /// - `align`: The bytes on which to align for the linear allocator. Must be a multiple of two.
    /// - `size`: The size of the area to allocate.
    /// 
    /// # Returns
    /// The "pointer" (index) in the area that this allocator manages that has been reserved for the new block.
    /// 
    /// # Errors
    /// This function may error if the block could not be allocated. In general, this would be because of not enough (continious) memory available.
    fn allocate(&mut self, align: usize, size: usize) -> Result<usize, Error> {
        todo!();
    }



    /// Returns the type of this MemoryAllocator.
    #[inline]
    fn kind(&self) -> MemoryAllocatorKind { MemoryAllocatorKind::Dense }

    /// Returns the space used in the area managed by this MemoryAllocator.
    #[inline]
    fn size(&self) -> usize { todo!(); }

    /// Returns the total capacity of the area managed by this MemoryAllocator.
    #[inline]
    fn capacity(&self) -> usize { todo!(); }
}
