#![cfg(feature = "re2")]

use std::ffi::CString;
use std::os::raw::c_char;

pub fn hello_is_match(pattern: &str, input: &str) -> bool {
    let pattern = CString::new(pattern).unwrap();
    let input = CString::new(input).unwrap();
    unsafe { is_match(pattern.as_ptr(), input.as_ptr()) }
}

extern "C" {
    fn is_match(pattern: *const c_char, string: *const c_char) -> bool;
}
