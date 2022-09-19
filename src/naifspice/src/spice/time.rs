use crate::error::*;
use crate::Spice;
use std::fmt::{Display, Formatter};

use crate::string_tools::*;
use crate::sys::*;

pub type SpiceEt = f64;
pub type SpiceSclk = f64;

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
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/str2et_c.html>
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
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/timout_c.html>
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
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/et2utc_c.html>
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
        let str_time = self.et2utc(et, "ISOC", 9)?;
        match NaiveDateTime::parse_from_str(&str_time, "%FT%T.%f") {
            Ok(ndt) => Ok(ndt),
            Err(e) => Err(Box::new(TimeConversionError {
                msg: format!("{} ({})", e.to_string(), str_time),
            })),
        }
    }

    /// Convert a spacecraft clock string to ephemeris seconds past
    ///
    /// wraps scs2e_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scs2e_c.html>
    pub fn scs2e(&self, scid: SpiceInt, sclkch: &str) -> SpiceResult<SpiceEt> {
        with_string_ref(sclkch, |s| {
            let mut res: SpiceEt = 0.0;
            unsafe {
                scs2e_c(scid, s, &mut res);
            }

            self.check_for_error()?;
            Ok(res)
        })
    }

    ///  Convert an epoch specified as ephemeris seconds past J2000 (ET) to a
    ///   character string representation of a spacecraft clock value (SCLK).
    ///
    /// wraps sce2s_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2s_c.html>
    pub fn sce2s(&self, sc: SpiceInt, et: SpiceEt) -> SpiceResult<String> {
        let mut bytes = null_vec(128);
        unsafe {
            sce2s_c(sc, et, bytes.len() as SpiceInt, bytes.as_mut_ptr());
        }

        self.check_for_error()?;
        Ok(vec_to_string(&bytes))
    }

    /// Convert encoded spacecraft clock (`ticks') to ephemeris
    /// seconds past J2000 (ET).
    ///
    /// wraps sct2e_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sct2e_c.html>
    pub fn sct2e(&self, sc: SpiceInt, sclkpd: SpiceSclk) -> SpiceResult<SpiceEt> {
        let mut res: SpiceEt = 0.0;
        unsafe {
            sct2e_c(sc, sclkpd, &mut res);
        }
        self.check_for_error()?;

        Ok(res)
    }

    /// Convert ephemeris seconds past J2000 (ET) to continuous encoded
    /// spacecraft clock (`ticks').  Non-integral tick values may be
    /// returned.
    ///
    /// wraps sce2c_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/sce2c_c.html>
    pub fn sce2c(&self, sc: SpiceInt, et: SpiceEt) -> SpiceResult<SpiceSclk> {
        let mut res: SpiceSclk = 0.0;
        unsafe {
            sce2c_c(sc, et, &mut res);
        }

        self.check_for_error()?;
        Ok(res)
    }

    /// Encode a character representation of spacecraft clock time into a
    /// double precision number.
    ///
    /// wraps scencd_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scencd_c.html>
    pub fn scencd(&self, sc: SpiceInt, s: &str) -> SpiceResult<SpiceSclk> {
        with_string_ref(s, |s| {
            let mut res: SpiceSclk = 0.0;
            unsafe {
                scencd_c(sc, s, &mut res);
            }

            self.check_for_error()?;
            Ok(res)
        })
    }

    /// Convert a double precision encoding of spacecraft clock time into
    /// a character representation.
    ///
    /// wraps scdecd_c
    /// <https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/scdecd_c.html>
    pub fn scdecd(&self, sc: SpiceInt, sclkdp: SpiceSclk) -> SpiceResult<String> {
        let mut bytes = null_vec(128);
        unsafe {
            scdecd_c(sc, sclkdp, bytes.len() as SpiceInt, bytes.as_mut_ptr());
        }
        self.check_for_error()?;
        Ok(vec_to_string(&bytes))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;

    #[cfg(feature = "chrono")]
    use chrono::NaiveDate;
    use chrono::{Datelike, Timelike};

    #[test]
    #[serial]
    fn test_et_str() {
        run_in_temp_dir!({
            test_file!("naif0011.tls");

            let api = Spice::create().unwrap();
            assert!(
                api.furnsh("naif0011.tls").is_ok(),
                "could not load leap second kernel"
            );

            // JD 2451545.0 is about 64.18 seconds before noon utc. not gonna check subseconds here.
            match api.str2et("2000-01-01 12:00:00.00") {
                Ok(et) => assert_eq!(
                    et.round() as i32,
                    64,
                    "Time conversion is off! J2000 is not where it should be."
                ),
                Err(e) => assert!(false, "String to time conversion failed: {}", e),
            }

            // 2451545.0 + 1 minute = midday + 1 minute 2000-01-01. the above mentioned 64 seconds are more like 64.18 though
            match api.et2utc(124.18, "ISOC", 2) {
                Ok(utcstr) => assert_eq!(utcstr, "2000-01-01T12:01:00.00"),
                Err(e) => assert!(false, "Time to string conversion failed: {}", e),
            }

            match api.timout(0.0, "MON DD,YYYY HR:MN:SC.#### (TDB) ::TDB") {
                Ok(timestr) => assert_eq!(
                    timestr, "JAN 01,2000 12:00:00.0000 (TDB)",
                    "et to timestring via timeout_c is off"
                ),
                Err(e) => assert!(false, "Time to string via timout_c failed: {}", e),
            }
        });
    }

    #[test]
    #[serial]
    #[cfg(feature = "chrono")]
    fn test_chrono_time() {
        run_in_temp_dir!({
            test_file!("naif0011.tls");

            let api = Spice::create().unwrap();
            assert!(
                api.furnsh("naif0011.tls").is_ok(),
                "could not load leap second kernel"
            );

            let dt = NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0);
            match api.ndt_to_et(&dt) {
                Ok(et) => {
                    assert_eq!(et.round() as i32, 64, "NaiveDateTime to et failed");
                }
                Err(e) => {
                    assert!(false, "NaiveDateTime to et failed: {}", e.to_string())
                }
            }

            let et2 = 124.19; // roughtly one minute after midday 2000-01-01
            match api.et_to_ndt(et2) {
                Ok(dt2) => {
                    assert_eq!(dt2.year(), 2000, "et to NaiveDateTime year is off");
                    assert_eq!(dt2.month(), 1, "et to NaiveDateTime month is off");
                    assert_eq!(dt2.day(), 1, "et to NaiveDateTime day is off");
                    assert_eq!(dt2.hour(), 12, "et to NaiveDateTime hour is off");
                    assert_eq!(dt2.minute(), 1, "et to NaiveDateTime minute is off");
                    assert_eq!(dt2.second(), 0, "et to NaiveDateTime second is off");
                }
                Err(e) => {
                    assert!(false, "et to NaiveDateTime failed: {}", e);
                }
            }
        });
    }
}
