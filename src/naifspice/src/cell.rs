//! Cells are not thread safe, because error handling is not
//! Thus, to avoid UB, we need some way to safe cells access, as these all can err
//! Basic access is fine, but everything using api functions is not

pub use crate::sys::SpiceCell as SysCell;
use crate::sys::*;
use std::ffi::c_void;
use std::ffi::{CStr, CString};
use std::string::FromUtf8Error;

static SPICE_CELL_CTRLSZ: usize = naifspice_sys::SPICE_CELL_CTRLSZ as usize;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum SpiceCellConversionError {
    MalformedString,
    LengthToShort,
}

/**
 * Describes the common api for the high level abstraction of spice cells.
 */
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

    fn length(&self) -> usize {
        self.raw_vec().len() - SPICE_CELL_CTRLSZ
    }

    unsafe fn as_spice_cell_mut(&mut self) -> SysCell;

    unsafe fn with_sys_cell<F, R>(&mut self, cb: F) -> Result<R, SpiceCellConversionError>
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

        Ok(res)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

/**
 * Char cells are different from the rest, as they are two dimensional.
 * This implementation below is a bit naive, but it should get the job done for everything not super huge
*/
#[derive(Debug, Clone)]
pub struct SpiceCellChar {
    vec: Vec<String>,
    element_length: usize,
}

const SPICE_CELL_CHAR_DEFAULT_LENGTH: usize = 128;

impl SpiceCellChar {
    /**
     * Create a SpiceCellChar with an arbitrary capacity and length
     */
    fn create_with_element_length(cap: usize, length: usize) -> Self {
        let mut vec = Vec::new();
        vec.reserve(cap);
        Self {
            vec,
            element_length: length,
        }
    }

    /**
     * Change the length used when converting to native SpiceCells. Usefull when the default
     * of SPICE_CELL_CHAR_DEFAULT_LENGTH = 128 is not enough to store the data returned by spice.
     */
    fn set_element_length(&mut self, l: usize) {
        self.element_length = l;
    }

    /**
     * Gets the element length (aka length of sys cell)
     */
    fn element_length(&self) -> usize {
        self.element_length
    }
}

impl SpiceCell for SpiceCellChar {
    type Item = String;

    fn create() -> Self {
        Self {
            vec: Vec::new(),
            element_length: SPICE_CELL_CHAR_DEFAULT_LENGTH,
        }
    }

    fn create_capacity(cap: usize) -> Self {
        let mut vec = Vec::new();
        vec.reserve(cap);
        Self {
            vec,
            element_length: SPICE_CELL_CHAR_DEFAULT_LENGTH,
        }
    }

    fn raw_vec(&self) -> &Vec<Self::Item> {
        &self.vec
    }

    fn raw_vec_mut(&mut self) -> &mut Vec<Self::Item> {
        &mut self.vec
    }

    fn from_slice(s: &[Self::Item]) -> Self {
        let mut vec = Vec::new();
        vec.extend_from_slice(s);

        Self {
            vec,
            element_length: SPICE_CELL_CHAR_DEFAULT_LENGTH,
        }
    }

    fn from_vec(s: &Vec<Self::Item>) -> Self {
        Self {
            vec: s.clone(),
            element_length: SPICE_CELL_CHAR_DEFAULT_LENGTH,
        }
    }

    fn into_vec(self) -> Vec<Self::Item> {
        self.vec
    }

    fn push(&mut self, val: Self::Item) {
        self.vec.push(val);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.vec.pop()
    }

    fn slice(&self) -> &[Self::Item] {
        self.vec.as_slice()
    }

    fn slice_mut(&mut self) -> &mut [Self::Item] {
        self.vec.as_mut_slice()
    }

    fn length(&self) -> usize {
        self.raw_vec().len()
    }

    unsafe fn as_spice_cell_mut(&mut self) -> SysCell {
        panic!("It not make sense to call as_spice_cell_mut on CharCells");
    }

    unsafe fn with_sys_cell<F, R>(&mut self, cb: F) -> Result<R, SpiceCellConversionError>
    where
        F: Fn(*mut SysCell) -> R,
    {
        // we need everything in continuous memory unfortunately
        let mut bytes_vec = Vec::new();
        bytes_vec.resize(self.element_length * SPICE_CELL_CTRLSZ, 0);

        for s in &self.vec {
            let mut v = Vec::new();
            v.reserve(self.element_length);
            v.extend_from_slice(s.as_bytes());
            if v.len() < self.element_length {
                v.resize(self.element_length, 0);
            } else {
                return Err(SpiceCellConversionError::LengthToShort);
            }

            // this is really important
            assert_eq!(v.len(), self.element_length);
            // we done for now
            bytes_vec.extend(v);
        }
        // if we have strings reserved, we append them as 0 bytes
        if self.vec.capacity() > self.vec.len() {
            for _ in 0..(self.vec.capacity() - self.vec.len()) {
                let mut v = Vec::new();
                v.resize(self.element_length, 0);
                bytes_vec.extend(v);
            }
        }
        let mut cell = SysCell {
            dtype: crate::sys::_SpiceDataType_SPICE_CHR,
            length: self.element_length as crate::sys::SpiceInt,
            size: self.vec.capacity() as crate::sys::SpiceInt,
            card: self.vec.len() as crate::sys::SpiceInt,
            isSet: 0,
            adjust: 0,
            init: 1,
            base: bytes_vec.as_ptr() as *mut SpiceChar as *mut c_void,
            data: bytes_vec[SPICE_CELL_CTRLSZ * self.element_length..].as_ptr() as *mut SpiceChar
                as *mut c_void,
        };
        // we
        let res = cb(&mut cell);
        // since we copied everything we have to transform back everything
        // skips control bytes
        let mut new_vec = Vec::new();
        for i in 0..(cell.card as usize) {
            // we dont need 0 bytes, the rest is just offset-jumping around in the raw bytes.
            let offset =
                SPICE_CELL_CTRLSZ * self.element_length + (i * self.element_length) as usize;
            let mut bytes_now: Vec<u8> = bytes_vec
                .iter()
                .skip(offset)
                .take_while(|x| **x != 0)
                .cloned()
                .collect();
            match String::from_utf8(bytes_now) {
                Ok(s) => new_vec.push(s),
                Err(_) => return Err(SpiceCellConversionError::MalformedString),
            }
        }

        self.vec.clear();
        self.vec = new_vec;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_tools::test_compare_some;
    use serial_test::serial;
    use std::ptr::slice_from_raw_parts_mut;

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

    #[test]
    #[serial]
    fn test_char_cell() {
        let mut char_cell = SpiceCellChar::create_with_element_length(1024, 64);
        char_cell.push("Hi!".to_string());
        char_cell.push("dont!".to_string());
        char_cell.push("break!".to_string());
        assert!(test_compare_some(char_cell.pop(), "break!".to_string()));
        assert!(test_compare_some(char_cell.pop(), "dont!".to_string()));
        assert!(test_compare_some(char_cell.pop(), "Hi!".to_string()));
        assert!(char_cell.pop().is_none());
    }

    #[test]
    #[serial]
    fn test_char_cell_basic_transform() {
        let mut char_cell = SpiceCellChar::create_with_element_length(1024, 64);
        char_cell.push("Hi!".to_string());
        char_cell.push("dont!".to_string());
        char_cell.push("break!".to_string());
        let res = unsafe { char_cell.with_sys_cell(|x| {}) };
        assert!(res.is_ok());
        assert!(test_compare_some(char_cell.pop(), "break!".to_string()));
        assert!(test_compare_some(char_cell.pop(), "dont!".to_string()));
        assert!(test_compare_some(char_cell.pop(), "Hi!".to_string()));
        assert!(char_cell.pop().is_none());
    }

    #[test]
    #[serial]
    fn test_char_cell_legnth() {
        let mut char_cell = SpiceCellChar::create_with_element_length(1024, 5);
        char_cell.push("12345".to_string());
        let res = unsafe { char_cell.with_sys_cell(|x| {}) };
        assert!(test_compare_some(
            res.err(),
            SpiceCellConversionError::LengthToShort
        ));
    }

    #[test]
    #[serial]
    fn test_char_cell_mods() {
        let mut char_cell = SpiceCellChar::create_with_element_length(1024, 123);
        char_cell.push("ABCD".to_string());
        let res = unsafe {
            char_cell.with_sys_cell(|x| {
                *((*x).data as *mut u8) = 0x20;
            })
        };
        assert!(res.is_ok());
        let s = char_cell.pop().unwrap();
        assert_eq!(s, " BCD".to_string());
    }

    #[test]
    #[serial]
    fn test_char_cell_decode_fail() {
        let mut char_cell = SpiceCellChar::create_with_element_length(1024, 123);
        char_cell.push("A\x28".to_string());
        let res = unsafe {
            char_cell.with_sys_cell(|x| {
                *((*x).data as *mut u8) = 0xc3;
            })
        };
        assert!(test_compare_some(
            res.err(),
            SpiceCellConversionError::MalformedString
        ));
        // fail? cell should be unchanged
        assert!(test_compare_some(char_cell.pop(), "A\x28".to_string()));
    }

    #[test]
    #[serial]
    fn test_cell_clone() {
        let mut char_cell = SpiceCellChar::create_with_element_length(1024, 123);
        char_cell.push("Hallo".to_string());
        let mut b = char_cell.clone();
        assert!(test_compare_some(b.pop(), "Hallo".to_string()));
        assert!(test_compare_some(char_cell.pop(), "Hallo".to_string()));
    }
}
