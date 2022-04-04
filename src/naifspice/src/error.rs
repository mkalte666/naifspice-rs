use std::fmt::{Debug, Display};

pub trait SpiceError: Debug + Display {}

pub type SpiceResult<T> = Result<T, Box<dyn SpiceError>>;
