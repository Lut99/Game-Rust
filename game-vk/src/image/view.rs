/* VIEW.rs
 *   by Lut99
 *
 * Created:
 *   05 Apr 2022, 17:41:18
 * Last edited:
 *   05 Apr 2022, 18:12:11
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains code related to image views.
**/

// use ash::vk;

// // pub use crate::errors::ImageError;
// pub use crate::errors::ImageViewError as Error;


// /***** AUXILLARY STRUCTS *****/
// /// Helper struct for the CreateInfo that defines the component swizzle for an image.
// #[derive(Debug, Clone)]
// pub struct ComponentSwizzle {
//     /// The mapping of the red channel
//     red   : vk::ComponentSwizzle,
//     /// The mapping of the green channel
//     green : vk::ComponentSwizzle,
//     /// The mapping of the blue channel
//     blue  : vk::ComponentSwizzle,
//     /// The mapping of the alpha channel
//     alpha : vk::ComponentSwizzle,
// }

// impl Default for ComponentSwizzle {
//     fn default() -> Self {
//         Self {
//             red   : vk::ComponentSwizzle::IDENTITY,
//             green : vk::ComponentSwizzle::IDENTITY,
//             blue  : vk::ComponentSwizzle::IDENTITY,
//             alpha : vk::ComponentSwizzle::IDENTITY,
//         }
//     }
// }



// /// CreateInfo for the View.
// #[derive(Debug, Default, Clone)]
// pub struct CreateInfo {
//     /// Defines the type of the image view
//     kind   : vk::ImageViewType,
//     /// Defines the format of the image
//     format : vk::Format,
//     /// Defines the channel mapping for the image
//     swizzle : ComponentSwizzle,

//     /// Defines the aspect for this image (how it will be used)
//     aspect : vk::ImageAspectFlags,
//     /// Defines the number of image MIP levels
//     base_mip_level : u32,
// }





// /***** LIBRARY *****/
// /// The ImageView class, which wraps around an Image or a VkImage to define how it should be accessed.
// pub struct View<'a> {
//     /// The parent image for this view, who's lifetime we are tied  to
//     image : &'a vk::Image,

//     /// The image view object itself.
//     view : vk::ImageView,
// }

// impl<'a> View<'a> {
//     /// Constructor for the View.
//     /// 
//     /// Creates a new ImageView with the given properties from the given Image.
//     /// 
//     /// # Examples
//     /// 
//     /// ```
//     /// // TBD
//     /// ```
//     /// 
//     /// # Errors
//     /// 
//     /// This function errors if the Vulkan backend errors.
//     pub fn new() -> Result<Self, Error> {
//         Err(Error::NotImplemented)
//     }

//     /// Constructor for the View.
//     /// 
//     /// Creates a new ImageView with the given properties from the given Vulkan Image.
//     /// 
//     /// # Examples
//     /// 
//     /// ```
//     /// // TBD
//     /// ```
//     /// 
//     /// # Errors
//     /// 
//     /// This function errors if the Vulkan backend errors.
//     pub fn from_vk(image: vk::Image, create_info: CreateInfo) -> Result<Self, Error> {
//         // Define the mapping for the image channels
        

//         Ok(())
//     }
// }
