//! Cells are not thread safe, because error handling is not
//! Thus, to avoid UB, we need some way to safe cells access, as these all can err
//! Basic access is fine, but everything using api functions is not

pub use crate::sys::SpiceCell as SysCell;
use crate::sys::*;
use std::ffi::c_void;

static SPICE_CELL_CTRLSZ: usize = naifspice_sys::SPICE_CELL_CTRLSZ as usize;

trait SpiceCell {
    type Item;
    fn create() -> Self;
    fn create_capacity(cap: usize) -> Self;
    fn raw_vec(&self) -> &Vec<Self::Item>;
    fn raw_vec_mut(&mut self) -> &mut Vec<Self::Item>;

    fn from_slice(s: &[Self::Item]) -> Self;
    fn from_vec(s: &Vec<Self::Item>) -> Self;

    fn into_vec(self) -> Vec<Self::Item>;
    fn push(&mut self, val: Self::Item) {
        self.raw_vec_mut().push(val)
    }
    fn pop(&mut self) -> Option<Self::Item> {
        if self.raw_vec().len() > SPICE_CELL_CTRLSZ {
            self.raw_vec_mut().pop()
        } else {
            None
        }
    }

    fn slice(&self) -> &[Self::Item] {
        &self.raw_vec()[SPICE_CELL_CTRLSZ..]
    }
    fn slice_mut(&mut self) -> &mut [Self::Item] {
        &mut self.raw_vec_mut()[SPICE_CELL_CTRLSZ..]
    }

    unsafe fn as_spice_cell_mut(&mut self) -> SysCell;

    unsafe fn with_sys_cell<F, R>(&mut self, cb: F) -> R
    where
        F: Fn(*mut SysCell) -> R,
    {
        let mut cell = self.as_spice_cell_mut();
        let res = cb(&mut cell);
        // cell might have grown, so we need to adjust our vec
        // the api guarantees that cell.card <= cell.size so we can just set_len
        // this is what makes this api unsafe! users can break this by touching SysCell
        if cell.card as usize != self.raw_vec().len() {
            self.raw_vec_mut().set_len(cell.card as usize);
        }

        res
    }
}

pub struct SpiceCellDouble {
    vec: Vec<f64>,
}

impl SpiceCell for SpiceCellDouble {
    type Item = f64;

    fn create() -> Self {
        SpiceCellDouble {
            vec: vec![0.0; SPICE_CELL_CTRLSZ],
        }
    }

    fn create_capacity(cap: usize) -> Self {
        let mut vec = vec![0.0; SPICE_CELL_CTRLSZ];
        vec.reserve_exact(cap);
        Self { vec }
    }

    fn raw_vec(&self) -> &Vec<Self::Item> {
        &self.vec
    }

    fn raw_vec_mut(&mut self) -> &mut Vec<Self::Item> {
        &mut self.vec
    }

    fn from_slice(s: &[f64]) -> Self {
        let mut vec = Vec::new();
        vec.resize(SPICE_CELL_CTRLSZ, 0.0);
        vec.extend_from_slice(s);

        SpiceCellDouble { vec }
    }

    fn from_vec(v: &Vec<f64>) -> Self {
        let mut vec = Vec::new();
        vec.resize(SPICE_CELL_CTRLSZ, 0.0);
        vec.extend(v.iter());

        SpiceCellDouble { vec }
    }

    fn into_vec(mut self) -> Vec<f64> {
        self.vec.split_off(SPICE_CELL_CTRLSZ - 1)
    }

    unsafe fn as_spice_cell_mut(&mut self) -> SysCell {
        SysCell {
            dtype: crate::sys::_SpiceDataType_SPICE_DP,
            length: 0,
            size: self.vec.capacity() as SpiceInt,
            card: self.vec.len() as SpiceInt,
            isSet: 0,
            adjust: 0,
            init: 1,
            base: self.vec.as_ptr() as *mut SpiceDouble as *mut c_void,
            data: self.slice_mut().as_ptr() as *mut SpiceDouble as *mut c_void,
        }
    }
}

pub struct SpiceCellInt {
    vec: Vec<SpiceInt>,
}

impl SpiceCell for SpiceCellInt {
    type Item = SpiceInt;

    fn create() -> Self {
        Self {
            vec: vec![0; SPICE_CELL_CTRLSZ],
        }
    }

    fn create_capacity(cap: usize) -> Self {
        let mut vec = vec![0; SPICE_CELL_CTRLSZ];
        vec.reserve_exact(cap);
        Self { vec }
    }

    fn raw_vec(&self) -> &Vec<Self::Item> {
        &self.vec
    }

    fn raw_vec_mut(&mut self) -> &mut Vec<Self::Item> {
        &mut self.vec
    }

    fn from_slice(s: &[Self::Item]) -> Self {
        let mut vec = Vec::new();
        vec.resize(SPICE_CELL_CTRLSZ, 0);
        vec.extend_from_slice(s);

        Self { vec }
    }

    fn from_vec(s: &Vec<Self::Item>) -> Self {
        let mut vec = Vec::new();
        vec.resize(SPICE_CELL_CTRLSZ, 0);
        vec.extend(s.iter());

        Self { vec }
    }

    fn into_vec(mut self) -> Vec<Self::Item> {
        self.vec.split_off(SPICE_CELL_CTRLSZ - 1)
    }

    unsafe fn as_spice_cell_mut(&mut self) -> SysCell {
        SysCell {
            dtype: crate::sys::_SpiceDataType_SPICE_INT,
            length: 0,
            size: self.vec.capacity() as SpiceInt,
            card: self.vec.len() as SpiceInt,
            isSet: 0,
            adjust: 0,
            init: 1,
            base: self.vec.as_ptr() as *mut SpiceInt as *mut c_void,
            data: self.slice_mut().as_ptr() as *mut SpiceInt as *mut c_void,
        }
    }
}

pub struct SpiceCellChar {
    vec: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_tools::test_compare_some;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_double_cell() {
        let mut d_cell = SpiceCellDouble::create_capacity(128);
        assert_eq!(d_cell.vec.capacity(), 128 + SPICE_CELL_CTRLSZ);
        assert_eq!(d_cell.vec.len(), SPICE_CELL_CTRLSZ);
        d_cell.push(3.0);
        d_cell.push(4.0);
        d_cell.push(123.0);
        assert_eq!(d_cell.vec.len(), SPICE_CELL_CTRLSZ + 3);
        d_cell.pop();
        d_cell.pop();
        d_cell.pop();
        assert!(d_cell.pop().is_none());
        assert_eq!(d_cell.vec.len(), SPICE_CELL_CTRLSZ);
        assert_eq!(d_cell.vec.capacity(), 128 + SPICE_CELL_CTRLSZ);
    }

    #[test]
    #[serial]
    fn test_int_cell() {
        let mut d_cell = SpiceCellInt::create_capacity(123);
        assert_eq!(d_cell.vec.capacity(), 123 + SPICE_CELL_CTRLSZ);
        assert_eq!(d_cell.vec.len(), SPICE_CELL_CTRLSZ);
        d_cell.push(3);
        d_cell.push(4);
        d_cell.push(123);
        assert_eq!(d_cell.vec.len(), SPICE_CELL_CTRLSZ + 3);

        assert!(test_compare_some(d_cell.pop(), 123));
        assert!(test_compare_some(d_cell.pop(), 4));
        assert!(test_compare_some(d_cell.pop(), 3));
        assert!(d_cell.pop().is_none());
        assert_eq!(d_cell.vec.len(), SPICE_CELL_CTRLSZ);
        assert_eq!(d_cell.vec.capacity(), 123 + SPICE_CELL_CTRLSZ);
    }
}
