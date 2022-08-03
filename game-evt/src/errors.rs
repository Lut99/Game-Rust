//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    18 Jul 2022, 18:30:11
//  Last edited:
//    03 Aug 2022, 18:16:51
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the errors for the EventSystem.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};


/***** LIBRARY *****/
/// Errors that relate to the EventSystem as a whole.
#[derive(Debug)]
pub enum EventError {
    Temp,
}

impl Display for EventError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use EventError::*;
        match self {
            Temp => write!(f, "Temp"),
        }
    }
}

impl Error for EventError {}
