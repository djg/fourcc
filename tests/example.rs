#[macro_use]
extern crate fourcc;

#[test]
fn basic() {
    assert_eq!(fourcc!("asys"), 0x61737973);
}

#[test]
fn big_endian() {
    assert_eq!(fourcc!("asys", big), 0x61737973);
}

#[test]
fn little_endian() {
    assert_eq!(fourcc!("asys", little), 0x73797361);
}
