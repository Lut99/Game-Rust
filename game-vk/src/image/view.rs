/* VIEW.rs
 *   by Lut99
 *
 * Created:
 *   05 Apr 2022, 17:41:18
 * Last edited:
 *   17 Apr 2022, 18:09:02
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains code related to image views.
**/

use std::ptr;

use ash::vk;

// pub use crate::errors::ImageError;
pub use crate::errors::ImageViewError as Error;
use crate::gpu::Gpu;


// /***** AUXILLARY ENUMS *****/
// /// The type of the ImageView
// #[derive(Clone, Copy, Debug)]
// pub enum ImageViewKind {
//     /// A simple, one-dimensional image (i.e., a line of pixels)
//     OneD,
//     /// A simple, one-dimensional image but as an array (i.e., for stereophonic 3D)
//     OneDArray,

//     /// A simple, two-dimensional image (i.e., a grid of pixels)
//     TwoD,
//     /// A simple, two-dimensional image but as an array (i.e., for stereophonic 3D)
//     TwoDArray,

//     /// A simple, three-dimensional image
//     ThreeD,

//     /// A cubic (3D?) image
//     Cube,
//     /// A cubic (3D?) image but an array (i.e., for stereophonic 3D)
//     CubeArray,
// }

// impl Default for ImageViewKind {
//     #[inline]
//     fn default() -> Self {
//         ImageViewKind::TwoD
//     }
// }

// impl From<vk::ImageViewType> for ImageViewKind {
//     fn from(value: vk::ImageViewType) -> Self {
//         match value {
//             vk::ImageViewType::TYPE_1D       => ImageViewKind::OneD,
//             vk::ImageViewType::TYPE_1D_ARRAY => ImageViewKind::OneDArray,
//             vk::ImageViewType::TYPE_2D       => ImageViewKind::TwoD,
//             vk::ImageViewType::TYPE_2D_ARRAY => ImageViewKind::TwoDArray,
//             vk::ImageViewType::TYPE_3D       => ImageViewKind::ThreeD,
//             vk::ImageViewType::CUBE          => ImageViewKind::Cube,
//             vk::ImageViewType::CUBE_ARRAY    => ImageViewKind::CubeArray,
//         }
//     }
// }





/***** AUXILLARY STRUCTS *****/
/// Helper struct for the CreateInfo that defines the component swizzle for an image.
#[derive(Debug, Clone)]
pub struct ComponentSwizzle {
    /// The mapping of the red channel
    pub red   : vk::ComponentSwizzle,
    /// The mapping of the green channel
    pub green : vk::ComponentSwizzle,
    /// The mapping of the blue channel
    pub blue  : vk::ComponentSwizzle,
    /// The mapping of the alpha channel
    pub alpha : vk::ComponentSwizzle,
}

impl Default for ComponentSwizzle {
    fn default() -> Self {
        Self {
            red   : vk::ComponentSwizzle::IDENTITY,
            green : vk::ComponentSwizzle::IDENTITY,
            blue  : vk::ComponentSwizzle::IDENTITY,
            alpha : vk::ComponentSwizzle::IDENTITY,
        }
    }
}

impl From<vk::ComponentMapping> for ComponentSwizzle {
    #[inline]
    fn from(value: vk::ComponentMapping) -> Self {
        Self {
            red   : value.r,
            green : value.g,
            blue  : value.b,
            alpha : value.a,
        }
    }
}

impl From<ComponentSwizzle> for vk::ComponentMapping {
    #[inline]
    fn from(value: ComponentSwizzle) -> Self {
        Self {
            r : value.red,
            g : value.green,
            b : value.blue,
            a : value.alpha,
        }
    }
}



/// CreateInfo for the View.
#[derive(Debug, Default, Clone)]
pub struct CreateInfo {
    /// Defines the type of the image view
    pub kind    : vk::ImageViewType,
    /// Defines the format of the image
    pub format  : vk::Format,
    /// Defines the channel mapping for the image
    pub swizzle : ComponentSwizzle,

    /// Defines the aspect for this image (how it will be used)
    pub aspect     : vk::ImageAspectFlags,
    /// Defines the base MIP level
    pub base_level : u32,
    /// Defines the number of image MIP levels
    pub mip_levels : u32,
}





/***** LIBRARY *****/
/// The ImageView class, which wraps around an Image or a VkImage to define how it should be accessed.
pub struct View<'a> {
    /// The parent device for the parent image, who's lifetime we are tied  to
    gpu   : &'a Gpu,
    /// The parent image for this view
    image : vk::Image,
    /// The image view object itself.
    view  : vk::ImageView,
}

impl<'a> View<'a> {
    /// Constructor for the View.
    /// 
    /// Creates a new ImageView with the given properties from the given Image.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function errors if the Vulkan backend errors.
    pub fn new() -> Result<Self, Error> {
        Err(Error::NotImplemented)
    }

    /// Constructor for the View, from a VkImage instead of a Rusty one.
    /// 
    /// # Arguments
    /// - `gpu`: The GPU to allocate the view on.
    /// - `image`: The VkImage to base this image on.
    /// - `create_info`: The CreateInfo for this image view.
    /// 
    /// # Returns
    /// The new View instance on success, or else an Error.
    pub fn from_vk(gpu: &'a Gpu, image: vk::Image, create_info: CreateInfo) -> Result<Self, Error> {
        // Define the create info
        let image_info = vk::ImageViewCreateInfo {
            // Do the default stuff
            s_type : vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next : ptr::null(),
            flags  : vk::ImageViewCreateFlags::empty(),
            
            // Define the type of the image
            view_type  : create_info.kind,
            // Define the format of the image
            format     : create_info.format,
            // Define the component swizzler
            components : create_info.swizzle.into(),

            // Populate the subresource range
            subresource_range : vk::ImageSubresourceRange {
                aspect_mask      : create_info.aspect,
                base_mip_level   : create_info.base_level,
                level_count      : create_info.mip_levels,
                base_array_layer : 0,
                layer_count      : 1,
            },

            // Finally, set the image
            image,
        };

        // Use that to create the view
        let view = unsafe {
            match gpu.create_image_view(&image_info, None) {
                Ok(view) => view,
                Err(err) => { return Err(Error::ViewCreateError{ err }); }
            }
        };

        // Return the new instance
        Ok(Self {
            gpu,
            image,
            view,
        })
    }



    /// Returns a reference to the parent GPU
    #[inline]
    pub fn gpu(&self) -> &'a Gpu { self.gpu }

    /// Returns a reference to the parent image
    #[inline]
    pub fn image(&self) -> &vk::Image { &self.image }

    /// Returns a reference to the internal view
    #[inline]
    pub fn view(&self) -> &vk::ImageView { &self.view }
}

impl<'a> Drop for View<'a> {
    fn drop(&mut self) {
        unsafe { self.gpu.destroy_image_view(self.view, None); };
    }
}
