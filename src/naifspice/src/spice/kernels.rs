use crate::error::*;
use crate::Spice;

use crate::string_tools::*;
use crate::sys::{furnsh_c, unload_c};

impl Spice {
    /// Load one or more SPICE kernels into a program.
    ///
    /// wraps furnsh_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/furnsh_c.html>
    pub fn furnsh(&self, file: &str) -> SpiceResult<()> {
        with_string_ref(file, |file| {
            unsafe {
                furnsh_c(file);
            }
            self.check_for_error()?;
            Ok(())
        })
    }

    /// Unload a SPICE kernel.
    ///
    /// wraps unload_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/unload_c.html>
    pub fn unload(&self, file: &str) -> SpiceResult<()> {
        with_string_ref(file, |file| {
            unsafe {
                unload_c(file);
            }
            self.check_for_error()?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_furnsh_unload() {
        run_in_temp_dir!({
            test_file!("naif0011.tls");

            let api = Spice::create().unwrap();
            api.disable_error_texts().unwrap();
            assert!(
                api.furnsh("naif0011.tls").is_ok(),
                "loading valid kernel should succeed"
            );
            assert!(
                api.furnsh("invalid file my peoples").is_err(),
                "loading that doesnt exist should fail"
            );
            assert!(
                api.unload("more invalid").is_ok(),
                "unloading something not loaded should not fail"
            );
            assert!(
                api.unload("naif0011.tls").is_ok(),
                "unloading something loaded should succeed"
            );
        });
    }
}
