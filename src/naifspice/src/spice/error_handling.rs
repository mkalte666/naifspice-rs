use crate::error::{SpiceError, SpiceResult};
use crate::Spice;
use std::fmt::{Display, Formatter};

use crate::string_tools::*;
use crate::sys::*;

#[derive(Debug)]
struct SpiceApiError {
    short: String,
    long: String,
    explain: String,
    trace: String,
}

impl Display for SpiceApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The SPICE api returned an error:\n\tSHORT:      {}\n\tEXPLAIN:    {}\n\tLONG:       {}\n\tTRACE:      {}\n", self.short, self.explain, self.long, self.trace)
    }
}

impl SpiceError for SpiceApiError {
    fn spice_error_text(&self) -> String {
        self.long.clone()
    }
}

impl Spice {
    /// checks if an error is present. if yes, it clears the flag and returns it as an Err()
    pub(super) fn check_for_error(&self) -> SpiceResult<()> {
        // the error path is quite full of copies, but errors are usually quite bad anyway.
        // Thus its probably no issue that this is a lot of code to copy a bit of text.
        if unsafe { failed_c() } == SPICETRUE as i32 {
            let mut short_bytes = null_vec(SPICE_ERROR_LMSGLN as usize);
            let mut long_bytes = null_vec(SPICE_ERROR_LMSGLN as usize);
            let mut explain_bytes = null_vec(SPICE_ERROR_LMSGLN as usize);
            let mut trace_bytes = null_vec(SPICE_ERROR_TRCLEN as usize);
            let short = unsafe {
                getmsg_c(
                    spice_str!("SHORT"),
                    short_bytes.len() as SpiceInt,
                    short_bytes.as_mut_ptr(),
                );
                vec_to_string(&short_bytes)
            };
            let long = unsafe {
                getmsg_c(
                    spice_str!("LONG"),
                    long_bytes.len() as SpiceInt,
                    long_bytes.as_mut_ptr(),
                );
                vec_to_string(&long_bytes)
            };
            let explain = unsafe {
                getmsg_c(
                    spice_str!("EXPLAIN"),
                    explain_bytes.len() as SpiceInt,
                    explain_bytes.as_mut_ptr(),
                );
                vec_to_string(&explain_bytes)
            };
            let trace = unsafe {
                qcktrc_c(trace_bytes.len() as SpiceInt, trace_bytes.as_mut_ptr());
                vec_to_string(&trace_bytes)
            };

            unsafe {
                reset_c();
            }

            Err(Box::new(SpiceApiError {
                short,
                explain,
                long,
                trace,
            }))
        } else {
            Ok(())
        }
    }

    /// Setup function for the error handling.
    pub(super) fn setup_error_handling(&self) -> SpiceResult<()> {
        unsafe {
            reset_c();
            erract_c(spice_str!("SET"), 0, spice_str!("RETURN"));
        }
        self.check_for_error()?;

        Ok(())
    }

    /// Disables the error printing of SPICE.
    /// Errors can still be viewed and all explainations read by lookig at SpiceError returned by all functions that might cause one.
    pub fn disable_error_texts(&self) -> SpiceResult<()> {
        unsafe {
            errdev_c(spice_str!("SET"), 0, spice_str!("NULL"));
        }
        self.check_for_error()?;
        Ok(())
    }

    /// Turns SPICE error output that was disabled with disable_spice_error_texts back on.
    pub fn enable_error_texts(&self) -> SpiceResult<()> {
        unsafe {
            errdev_c(spice_str!("SET"), 0, spice_str!("SCREEN"));
        }
        self.check_for_error()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    pub fn test_api_errors() {
        if let Ok(api) = Spice::create() {
            assert!(api.disable_spice_error_texts().is_ok());
            unsafe {
                furnsh_c(spice_str!("Not A file . png"));
            }
            let should_be_err = api.check_for_error();
            assert!(should_be_err.is_err());
            if let Some(err) = should_be_err.err() {
                eprintln!("Example Error follows \n{}", err);
            }
        } else {
            assert!(false, "Could not create spice api for testing.")
        }
    }
}
