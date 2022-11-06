#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "re2")]
mod re2;

#[cfg(feature = "re2")]
pub use self::re2::hello_is_match;

pub fn multiply_add(x: i32, y: i32, z: i32) -> i32 {
    x * unsafe { sum(y, z) }
}

extern "C" {
    fn sum(x: i32, y: i32) -> i32;
}
