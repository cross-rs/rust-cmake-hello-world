#![no_std]

pub fn multiply_add(x: i32, y: i32, z: i32) -> i32 {
    x * unsafe { sum(y, z) }
}

extern "C" {
    fn sum(x: i32, y: i32) -> i32;
}
