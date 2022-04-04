use std::fmt::{Display, Debug};

pub trait SpiceError : Debug + Display {
    fn spice_error_text(&self) -> String;
}

pub type SpiceResult<T> = Result<T,Box<dyn SpiceError>>;