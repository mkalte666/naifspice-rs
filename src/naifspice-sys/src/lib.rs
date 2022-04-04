#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include!{"generated.rs"}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::*;

    #[test]
    fn spice_some_sanity() {
        unsafe {
            erract_c(CString::new("SET").unwrap().as_ptr() as *mut SpiceChar, 6, CString::new("RETURN").unwrap().as_ptr() as *mut SpiceChar);
            reset_c();
            assert_eq!(failed_c(),SPICEFALSE as i32);

        }
    }
}
