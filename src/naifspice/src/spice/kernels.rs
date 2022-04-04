use crate::error::*;
use crate::Spice;

use crate::string_tools::*;
use crate::sys::furnsh_c;

impl Spice {
    pub fn furnsh(&self, file: &str) -> SpiceResult<()> {
        with_string_ref(file, |file| {
            unsafe {
                furnsh_c(file);
            }
            self.check_for_error()?;
            Ok(())
        })
    }
}
