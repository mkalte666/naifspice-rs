#[cfg(feature = "export_sys")]
pub mod sys {
    pub use naifspice_sys::*;
}

#[cfg(not(feature = "export_sys"))]
mod sys {
    pub use naifspice_sys::*;
}

#[cfg(feature = "export_sys")]
#[macro_use]
pub mod string_tools;

#[cfg(not(feature = "export_sys"))]
#[macro_use]
mod string_tools;

#[cfg(test)]
#[macro_use]
mod test_tools;

pub mod error;
pub mod spice;
pub mod cell;

pub use spice::*;
pub use cell::*;
