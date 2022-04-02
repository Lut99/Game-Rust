/* TRIANGLE.rs
 *   by Lut99
 *
 * Created:
 *   02 Apr 2022, 13:20:45
 * Last edited:
 *   02 Apr 2022, 13:25:47
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Simple, test subsystem that simply renders a single triangle to one
 *   window.
**/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use log::debug;

use crate::subsystems::{RenderSubsystem, RenderSubsystemBuilder};


/***** TRIANGLE AUXILLARY TYPES *****/
/// The CreateInfo struct for the TriangleSystem.
pub struct CreateInfo {
    
}



/// The CreateError enum for the TriangleSystem.
#[derive(Debug)]
pub enum CreateError {
    /// Placeholder error
    Temp,
}

impl Display for CreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            CreateError::Temp => write!(f, "<<<TEMP>>>"),
        }
    }
}

impl Error for CreateError {}





/***** TRIANGLE SYSTEM *****/
/// Implements the simplest subsystem we have for testing purposes.
pub struct System {
    
}

impl RenderSubsystem for System {
    
}

impl RenderSubsystemBuilder for System {
    type CreateInfo = CreateInfo;
    type CreateError = CreateError;


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
    fn new(create_info: Self::CreateInfo) -> Result<Self, Self::CreateError> {
        debug!("Initialized TriangleSubsystem");
        Ok(Self {})
    }
}
