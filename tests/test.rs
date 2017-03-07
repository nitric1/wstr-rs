#[macro_use]
extern crate wstr;
extern crate widestring;

use widestring::WideStr;

#[test]
fn wstr_works() {
    println!("{:?}", wstr!("hello$"));
    assert_eq!(widestr!("abcd"), WideStr::from_slice(&[0x61u16, 0x62, 0x63, 0x64]));
    assert_eq!(wstr!("가나다"), &[0xAC00u16, 0xB098, 0xB2E4]);
    assert_eq!(widestr!("\u{1F007}\u{1F010}\u{1F019}"), WideStr::from_slice(&[0xD83Cu16, 0xDC07, 0xD83C, 0xDC10, 0xD83C, 0xDC19]));
}
