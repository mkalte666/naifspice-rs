use crate::error::*;
use crate::Spice;
use std::fmt::{Display, Formatter};

use crate::string_tools::*;
use crate::sys::*;

pub type SpiceEt = f64;

#[cfg(feature = "chrono")]
use chrono::NaiveDateTime;

#[derive(Debug)]
struct TimeConversionError {
    msg: String,
}

impl Display for TimeConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time conversion failed: {}", self.msg)
    }
}

impl SpiceError for TimeConversionError {}

impl Spice {
    /// Convert a string representing an epoch to a double precision
    /// value representing the number of TDB seconds past the J2000
    /// epoch corresponding to the input epoch.
    ///
    /// wraps str2et_c
    /// https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/str2et_c.html
    pub fn str2et(&self, s: &str) -> SpiceResult<SpiceEt> {
        with_string_ref(s, |s| {
            let mut f: f64 = 0.0;
            unsafe {
                str2et_c(s, &mut f);
            }
            self.check_for_error()?;
            Ok(f)
        })
    }

    /// Convert a `NaiveDateTime` to a spice et (TDB seconds past the J2000 epoch)
    ///
    /// This is a helper function enabled by the chrono feature and otherwise behaves like str2et
    #[cfg(feature = "chrono")]
    pub fn ndt_to_et(&self, utc: &NaiveDateTime) -> SpiceResult<SpiceEt> {
        // the formatting that spice uses is a little different.
        // this format string works
        let time_str = utc.format("%F %T.%t").to_string();
        self.str2et(&time_str)
    }

    /// Convert an input epoch represented in TDB seconds past the TDB
    /// epoch of J2000 to a character string formatted to the
    /// specifications of a user's format picture.
    ///
    /// wraps timout_c
    /// https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timout_c.html
    pub fn timout(&self, et: SpiceEt, pictur: &str) -> SpiceResult<String> {
        with_string_ref(pictur, |pictur| {
            let mut bytes = null_vec(128);
            unsafe {
                timout_c(et, pictur, bytes.len() as SpiceInt, bytes.as_mut_ptr());
            }
            self.check_for_error()?;
            let str = vec_to_string(&bytes);
            Ok(str)
        })
    }

    /// Convert an input time from ephemeris seconds past J2000
    /// to Calendar, Day-of-Year, or Julian Date format, UTC.
    ///
    /// Check the original documentation for info on the format string
    ///
    /// wraps et2utc_c
    /// https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/et2utc_c.html
    pub fn et2utc(&self, et: SpiceEt, format: &str, prec: i32) -> SpiceResult<String> {
        with_string_ref(format, |format| {
            let mut bytes = null_vec(128);
            unsafe {
                et2utc_c(
                    et,
                    format,
                    prec,
                    bytes.len() as SpiceInt,
                    bytes.as_mut_ptr(),
                );
            }
            self.check_for_error()?;
            let str = vec_to_string(&bytes);
            Ok(str)
        })
    }

    /// Convert a spice et (TDB seconds past the J2000 epoch) to a `NaiveDateTime`
    ///
    /// This is a helper function enabled by the chrono feature and behaves like et2utc
    #[cfg(feature = "chrono")]
    pub fn et_to_ndt(&self, et: SpiceEt) -> SpiceResult<NaiveDateTime> {
        let str_time = self.et2utc(et, "ISOC", 14)?;
        match NaiveDateTime::parse_from_str(&str_time, "%FT%T.%t") {
            Ok(ndt) => Ok(ndt),
            Err(e) => Err(Box::new(TimeConversionError { msg: e.to_string() })),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_et_str() {}

    #[test]
    #[serial]
    fn test_chrono_time() {}
}
