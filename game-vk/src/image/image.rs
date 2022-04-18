/* IMAGE.rs
 *   by Lut99
 *
 * Created:
 *   18 Apr 2022, 14:34:47
 * Last edited:
 *   18 Apr 2022, 15:38:48
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Defines a wrapper around Vulkan's Image buffer.
**/

use std::sync::Arc;

use ash::vk;

pub use crate::errors::ImageError as Error;


/***** LIBRARY *****/
/// Represents an image, which is a kind of buffer that we may render to.
pub struct Image {
    /// The VkImage we wrap around.
    image : vk::Image,
}

impl Image {
    /// Constructor for the Image, which takes an already existing VkImage and wraps around it.
    pub(crate) fn from_vk(image: vk::Image) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(Self {
            image,
        }))
    }



    /// Returns the internal VkImage.
    #[inline]
    pub fn vk(&self) -> vk::Image { self.image }
}
