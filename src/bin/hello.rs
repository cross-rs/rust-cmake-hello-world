#[cfg(feature = "re2")]
use rust_cmake_hello_world::hello_is_match;
use rust_cmake_hello_world::multiply_add;

pub fn main() {
    println!("multiply_add(2, 3, 4)={:?}", multiply_add(2, 3, 4));
    #[cfg(feature = "re2")]
    {
        let pattern = "Hello.*";
        let input = "Hello World";
        let matches = match hello_is_match(pattern, input) {
            true => "matches",
            false => "does not match",
        };
        println!("{pattern:?} {matches} the input {input:?}");
    }
}
