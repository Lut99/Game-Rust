/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   26 Mar 2022, 13:01:17
 * Last edited:
 *   02 Apr 2022, 14:44:47
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains interfaces and other structs for the GFX crate.
**/

use std::error::Error;


use game_evt::EventLoop;
use game_vk::instance::Instance;


/***** RENDERSUBSYSTEM TRAIT *****/
/// Defines the interface to the render subsystems.
pub trait RenderSubsystem: 'static {}



/// Defines the interface to the constructor of render subsystems.
pub trait RenderSubsystemBuilder: RenderSubsystem {
    /// The create info type for this RenderSubsystemBuilder.
    type CreateInfo;
    /// The Error type for this RenderSubsystemBuilder.
    type CreateError: Error;


    /// Constructor for the RenderSubsystem.
    /// 
    /// This function initializes a new subsystem using the given CreateInfo to tune it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // TBD
    /// ```
    /// 
    /// # Errors
    /// 
    /// When this function errors determines on the actual implementation of the constructor. However, it is defined that if it does, it returns the given CreateError.
    fn new(event_loop: &EventLoop, instance: &Instance, create_info: Self::CreateInfo) -> Result<Self, Self::CreateError> where Self: Sized;
}
