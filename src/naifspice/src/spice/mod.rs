use crate::error::{SpiceError, SpiceResult};
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};

/// Struct that implements the spice api, and guards it from multi-threaded access.
#[derive(Debug)]
pub struct Spice {
    _empty: (),
}

static INSTANCE_EXISTS: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct SpiceInitError {
    msg: String,
}

impl Display for SpiceInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error during SPICE Initialisation: {}", self.msg)
    }
}

impl SpiceError for SpiceInitError {
    fn spice_error_text(&self) -> String {
        self.msg.clone()
    }
}

impl Spice {
    pub fn create() -> SpiceResult<Self> {
        if let Ok(_) =
            INSTANCE_EXISTS.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        {
            let api = Self { _empty: () };
            api.setup_error_handling()?;
            Ok(api)
        } else {
            Err(Box::new(SpiceInitError {
                msg: "Tried to create multiple instances".to_string(),
            }))
        }
    }
}

impl Drop for Spice {
    fn drop(&mut self) {
        if let Ok(_) =
            INSTANCE_EXISTS.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
        {
            // done
        } else {
            panic!("Spice protection atomic (INSTANCE_EXISTS) cannot be set to false from true. This hints at a programming mistake on the api side, please create an issue. Thanks!");
        }
    }
}

mod error_handling;

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn spice_single_instance() {
        let a = Spice::create();
        assert!(a.is_ok());
        let b = Spice::create();
        assert!(b.is_err());
        drop(a);
        let c = Spice::create();
        assert!(c.is_ok());
    }
}
