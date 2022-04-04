pub mod sys {
    pub use naifspice_sys::*;
}

pub mod spice;
pub use spice::*;

pub mod error;