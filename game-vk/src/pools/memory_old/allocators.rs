/* ALLOCATORS.rs
 *   by Lut99
 *
 * Created:
 *   04 Jun 2022, 15:29:44
 * Last edited:
 *   25 Jun 2022, 16:15:55
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines the allocators used in the MemoryPool.
**/

use std::rc::Rc;

use game_utl::traits::AsAny;

pub(crate) use crate::pools::errors::MemoryPoolError as Error;
use crate::auxillary::MemoryAllocatorKind;


/***** AUXILLARY STRUCTS *****/
/// A single block of used memory within the used list.
struct UsedBlock {
    /// The offset of this used block.
    pointer : usize,
    /// The size of this used block.
    size    : usize,

    /// The pointer to the next used block.
    next : Option<Rc<Self>>,
    /// The pointer to the previous used block.
    prev : Option<Rc<Self>>,
}

impl UsedBlock {
    /// Constructor for the UsedBlock.
    /// 
    /// # Arguments
    /// - `pointer`: The start 'address' of the free block of memory. Relative to whatever block of memory the allocator is in charge of.
    /// - `size`: The size of the free block.
    /// - `next`: Pointer to the next UsedBlock. If omitted, implies the end of this UsedBlock aligns with the end of the allocator memory (i.e., last block).
    /// - `prev`: Pointer to the previous UsedBlock. If omitted, implies the start of this UsedBlock aligns with the start of the allocator memory (i.e., first block).
    #[inline]
    fn new(pointer: usize, size: usize, next: Option<Rc<Self>>, prev: Option<Rc<Self>>) -> Rc<Self> {
        Rc::new(Self {
            pointer,
            size,

            next,
            prev,
        })
    }
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
    /// Resets the Allocator to its empty state.
    /// 
    /// This _might_ be a very cheap operation for some allocators (i.e., `LinearAllocator`), and a less cheap operation for others (i.e., `DenseAllocator`).
    fn reset(&mut self);

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

    /// Resets the Allocator to its empty state.
    /// 
    /// This _might_ be a very cheap operation for some allocators (i.e., `LinearAllocator`), and a less cheap operation for others (i.e., `DenseAllocator`).
    #[inline]
    fn reset(&mut self) { self.pointer = 0; }



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
    /// A list of all used blocks within the DenseAllocator.
    used_list : Option<Rc<UsedBlock>>,

    /// A counter that keeps track of the used space in the allocator. Deducible from the free list, but here as optimization.
    size     : usize,
    /// A counter that keeps track of the total space in the allocator. Deducible from the free list, but here as optimization.
    capacity : usize,
}

impl DenseAllocator {
    /// Constructor for the DenseAllocator.
    /// 
    /// # Arguments
    /// - `size`: The size of the memory managed by the allocator.
    #[inline]
    pub(crate) fn new(size: usize) -> Self {
        Self {
            used_list : Some(UsedBlock::new(0, size, None, None)),

            size     : 0,
            capacity : size,
        }
    }



    /// Frees a given block in the DenseAllocator.
    /// 
    /// # Arguments
    /// - `pointer`: The pointer in the block to remove.
    /// 
    /// # Errors
    /// This function may error if the pointer does not point to the start of a block.
    pub(crate) fn free(&mut self, pointer: usize) -> Result<(), Error> {
        // Try to find the UsedBlock with this pointer
        let mut this: Option<&mut Rc<UsedBlock>> = self.used_list.as_mut();
        while this.is_some() {
            // Extract the block from the ptr
            let block: &mut Rc<UsedBlock> = this.unwrap();

            // Do the comparison
            if block.pointer == pointer {
                // Cool, remove it
                if let Some(prev) = block.prev.as_mut() {
                    prev.next = block.next;
                } else {
                    self.used_list = block.next;
                }
                if let Some(next) = block.next.as_mut() {
                    next.prev = block.prev;
                }

                // Return
                return Ok(());
            }

            // Otherwise, try the next one
            this = block.next.as_mut();
        }

        // Didn't find the memory block
        Err(Error::UnknownPointer{ ptr: pointer })
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
        // Optimisation: we can sometimes be sure no free block has space left
        if size > (self.capacity - self.size) { return Err(Error::OutOfMemoryError{ req_size: size }); }

        // Try find if we can insert enough memory before any of the used blocks
        // Note: we require that the list of memory blocks is sorted
        let mut this: Option<&mut Rc<UsedBlock>> = self.used_list.as_mut();
        while this.is_some() {
            // Extract the block from the ptr
            let block: &mut Rc<UsedBlock> = this.unwrap();

            // Compute the aligned end of the previous block
            let pointer = block.prev.map(|b| b.pointer + b.size).unwrap_or(0);
            let align_pointer = if align != 0 {
                if (align & (align - 1)) != 0 { panic!("Given alignment '{}' is not a power of two", align); }
                (pointer + (align - 1)) & ((!align) + 1)
            } else {
                pointer
            };

            // Compute the required and available space before this block
            let req_size: usize = (align_pointer - pointer) + size;
            let avl_size: usize = block.pointer - block.prev.map(|b| b.pointer + b.size).unwrap_or(0);
            if avl_size >= req_size {
                // Create a new used block and insert it before this one
                let new_block = UsedBlock::new(align_pointer, size, Some(block.clone()), block.prev.map(|b| b.clone()));
                block.prev = Some(new_block);

                // Update the size & return
                self.size += req_size;
                return Ok(align_pointer);
            }

            // Otherwise, try the next block
            this = block.next.as_mut();
        }
    
        // If we haven't found space before any of the blocks, then check if we have space after the last one
        if let Some(block) = self.used_list.as_mut() {
            // Compute the aligned end of the previous block
            let pointer = block.prev.map(|b| b.pointer + b.size).unwrap_or(0);
            let align_pointer = if align != 0 {
                if (align & (align - 1)) != 0 { panic!("Given alignment '{}' is not a power of two", align); }
                (pointer + (align - 1)) & ((!align) + 1)
            } else {
                pointer
            };

            // If that leaves enough space until the end, use it
            let req_size: usize = (align_pointer - pointer) + size;
            let avl_size: usize = self.capacity - align_pointer;
            if avl_size >= req_size {
                // Create a new used block and insert it after this one
                let new_block = UsedBlock::new(align_pointer, size, None, Some(block.clone()));
                block.next = Some(new_block);

                // Update the size & return
                self.size += req_size;
                return Ok(align_pointer);
            }
        }

        // No suitable free block found
        Err(Error::OutOfMemoryError{ req_size: size })
    }

    /// Resets the Allocator to its empty state.
    /// 
    /// This _might_ be a very cheap operation for some allocators (i.e., `LinearAllocator`), and a less cheap operation for others (i.e., `DenseAllocator`).
    fn reset(&mut self) {
        self.used_list = None;
        self.size      = 0;
    }



    /// Returns the type of this MemoryAllocator.
    #[inline]
    fn kind(&self) -> MemoryAllocatorKind { MemoryAllocatorKind::Dense }

    /// Returns the space used in the area managed by this MemoryAllocator.
    #[inline]
    fn size(&self) -> usize { self.size }

    /// Returns the total capacity of the area managed by this MemoryAllocator.
    #[inline]
    fn capacity(&self) -> usize { self.capacity }
}
