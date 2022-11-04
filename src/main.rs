fn main() {
    unsafe { hello() }
}

extern "C" {
    fn hello();
}
