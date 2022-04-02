/* SPEC.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 12:49:14
 * Last edited:
 *   02 Apr 2022, 13:19:07
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains common interfaces across the different submodules, including
 *   the RenderSubSystem trait.
**/

use std::error::Error;


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
    fn new(create_info: Self::CreateInfo) -> Result<Self, Self::CreateError> where Self: Sized;
}
