#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

include! {"generated.rs"}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::*;

    #[test]
    fn spice_some_sanity() {
        unsafe {
            reset_c();
            let set_str = CString::new("SET").unwrap();
            let return_str = CString::new("RETURN").unwrap();
            erract_c(
                set_str.as_ptr() as *mut SpiceChar,
                6,
                return_str.as_ptr() as *mut SpiceChar,
            );
            assert_eq!(failed_c(), SPICEFALSE as i32);

            let not_a_file_str = CString::new("not a file . png").unwrap();
            furnsh_c(not_a_file_str.as_ptr() as *mut SpiceChar);
            assert_eq!(failed_c(), SPICETRUE as i32);
        }
    }
}
