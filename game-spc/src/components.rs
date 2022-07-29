/* COMPONENTS.rs
 *   by Lut99
 *
 * Created:
 *   29 Jul 2022, 12:46:59
 * Last edited:
 *   29 Jul 2022, 12:48:13
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains ECS components that are used in more than one crate.
**/

use std::rc::Rc;

use game_ecs::Component;
use game_vk::auxillary::enums::ImageFormat;
use game_vk::auxillary::structs::Extent2D;
use game_vk::image;


/***** LIBRARY *****/
/// Defines a Target, which is an entity which may be rendered to.
pub struct Target {
    /// The ImageViews of this RenderTarget.
    pub views  : Vec<Rc<image::View>>,
    /// The format for these views.
    pub format : ImageFormat,
    /// The size (extent) of these views.
    pub extent : Extent2D<u32>,
}

impl Component for Target {}
