pub use crate::error::*;
pub use crate::sys::SpiceChar;
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct StringConversionError {
    msg: String,
}

impl Display for StringConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "String conversion failed: {}", self.msg)
    }
}

impl SpiceError for StringConversionError {}

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

/// helper to easier call functions that take mut pointers to raw c byte arrays with
/// Uses a callback to be on the safe side of the CString life time
pub fn with_string_ref<F, T>(s: &str, cb: F) -> SpiceResult<T>
where
    F: Fn(*mut SpiceChar) -> SpiceResult<T>,
{
    match CString::new(s) {
        Ok(str) => {
            let mut bytes = str.into_bytes_with_nul();
            cb(bytes.as_mut_ptr() as *mut SpiceChar)
        }
        Err(e) => Err(Box::new(StringConversionError { msg: e.to_string() })),
    }
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
    pub fn test_write_and_read_back() {
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

    #[test]
    #[serial]
    pub fn test_with_str() {
        assert!(with_string_ref("Haaalooooo", |_e| { Ok(()) }).is_ok());
        assert!(with_string_ref("Hiiihuuu\0with_err", |_e| { Ok(()) }).is_err());
    }
}
