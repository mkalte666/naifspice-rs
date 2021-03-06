//! Cells are not thread safe, because error handling is not
//! Thus, to avoid UB, we need some way to safe cells access, as these all can err
//! Basic access is fine, but everything using api functions is not
//!
//! The easiest way would be to just implement the api functions using the wrapped cell type,
//! but that would be clunky to use
//! ```rust
//!  let mut cell = SpiceCellDouble::create(100);
//!  api.appendd(&mut cell, 123.0);
//! ```
//!
//! Alternatively, one could let the `Spice` api create the cells, and then let them carry around a
//! reference to the spice api. That will create awkward lifetimes however.
//! ```rust
//!  let mut cell = api.create_cell_double(100);
//!  cell.append(123.0);
//! ```
//! The second option becomes especially problematic if `Spice` is wrapped inside an arc<mutex>.
//! Then the cells could not outlive the lock.
//!
//! Then there is the third alternative, that is: implement basic cell functions manually (append etc)
//! over the original c struct, using safe rust. This essentially ends up just as using vecs, however
//! there are 6 extra elements used by the original fortran api.

pub use naifspice_sys::SpiceCell as SysCell;

pub struct SpiceCellDouble
{
}