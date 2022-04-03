/* LIB.rs
 *   by Lut99
 *
 * Created:
 *   27 Mar 2022, 11:44:36
 * Last edited:
 *   03 Apr 2022, 15:13:32
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Contains the cross-crate utilities and functions for the Game.
**/

// /// Module that contains the errors for this crate
// pub mod errors;
/// Module that contains the common traits.
pub mod traits;
// /// Module that contains the common functions.
// pub mod utils;


/***** MACROS *****/
/// Translates a Rust String(-like) to a CString.
#[macro_export]
macro_rules! to_cstring {
    ($s:expr) => {
        std::ffi::CString::new($s.as_bytes()).unwrap_or_else(|_| panic!("Given string '{}' contains NULL-byte; cannot convert to CString", $s))
    };
}

/// Translates a Rust str(-like) to a CStr.
#[macro_export]
macro_rules! to_cstr {
    ($s:expr) => {
        std::ffi::CStr::from_bytes_with_nul(concat!($s, "\0").as_bytes()).unwrap_or_else(|_| panic!("Given str '{}' contains NULL-byte; cannot convert to CStr", $s))
    };
}

/// Translates a Rust CString to a ptr
#[macro_export]
macro_rules! to_p_cstring {
    ($cs:expr) => {
        $cs.as_ptr()
    };
    ($cs:expr,$ptr_type:tt) => {
        $cs.as_ptr() as $ptr_type
    };
}

/// Translates a Rust String to a ptr
#[macro_export]
macro_rules! to_p_string {
    ($s:expr) => {
        to_cstring!($s).as_ptr()
    };
    ($s:expr,$ptr_type:tt) => {
        to_cstring!($s).as_ptr() as $ptr_type
    };
}

/// Translates a Rust CString to a &str
#[macro_export]
macro_rules! from_cstring {
    ($cs:expr) => {
        $cs.to_str().unwrap_or_else(|err| panic!("Could not convert CString to str: {}", err))
    };
}

/// Translates a Rust ptr to a &str
#[macro_export]
macro_rules! from_p_cstring {
    ($ps:expr) => {
        CStr::from_ptr($ps).to_str().unwrap_or_else(|err| panic!("Could not convert raw ptr to str: {}", err))
    };
}
