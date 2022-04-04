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

pub mod spice;

pub mod error;

pub use spice::*;
