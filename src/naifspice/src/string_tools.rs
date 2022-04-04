pub use crate::error::*;
pub use crate::sys::SpiceChar;
use std::ffi::CStr;

///
/// Creates a `*mut SpiceChar` out of a string literal that can be used with cspice functions.
#[macro_export]
macro_rules! spice_str {
    ($lit:expr) => {
        (concat!($lit, "\0").as_ptr() as *mut SpiceChar)
    };
}

/// Create a vector with `len` elements of SpiceChar that are all null
pub fn null_vec(len: usize) -> Vec<SpiceChar> {
    let mut vec = Vec::new();
    vec.resize(len, '\0' as SpiceChar);
    vec
}

/// Create a `String` out of a vector of SpiceChars. Will return empty string if the vector is empty or not created with `null_vec`
pub fn vec_to_string(vec: &Vec<SpiceChar>) -> String {
    if vec.len() > 1 && vec.ends_with(&['\0' as SpiceChar]) {
        unsafe {
            let c_str = CStr::from_ptr(vec.as_ptr());
            if let Ok(slice) = c_str.to_str() {
                let string = slice.to_owned();
                return string;
            }
        }
    }

    return "".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys::*;
    use serial_test::serial;

    #[test]
    #[serial]
    pub fn test_literals_with_spice() {
        unsafe {
            reset_c();
            erract_c(spice_str!("SET"), 6, spice_str!("RETURN"));
            assert_eq!(failed_c(), SPICEFALSE as i32);
            erract_c(spice_str!("BORK"), 6, spice_str!("bark"));
            assert_eq!(failed_c(), SPICETRUE as i32);
        }
    }

    #[test]
    #[serial]
    pub fn write_and_read_back() {
        unsafe {
            reset_c();
            erract_c(spice_str!("SET"), 6, spice_str!("RETURN"));
            assert_eq!(failed_c(), SPICEFALSE as i32);
            let mut dst = null_vec(1024);
            erract_c(spice_str!("GET"), dst.len() as SpiceInt, dst.as_mut_ptr());
            let str = vec_to_string(&dst);
            assert_eq!(str, "RETURN");
        }
    }
}
