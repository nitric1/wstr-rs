#[macro_use]
extern crate wstr;

#[cfg(feature = "widestring")]
extern crate widestring;

#[test]
fn wstr_works() {
    assert_eq!(wstr!("abcd"), &[0x61u16, 0x62, 0x63, 0x64]);
    assert_eq!(wstr!("가나다"), &[0xAC00u16, 0xB098, 0xB2E4]);
    assert_eq!(wstr!("\u{1F007}\u{1F010}\u{1F019}"), &[0xD83Cu16, 0xDC07, 0xD83C, 0xDC10, 0xD83C, 0xDC19]);
}

#[cfg(feature = "widestring")]
#[test]
fn widestr_works() {
    assert_eq!(widestr!("abcd"), widestring::WideStr::from_slice(&[0x61u16, 0x62, 0x63, 0x64]));
    assert_eq!(widestr!("가나다"), widestring::WideStr::from_slice(&[0xAC00u16, 0xB098, 0xB2E4]));
    assert_eq!(widestr!("\u{1F007}\u{1F010}\u{1F019}"), widestring::WideStr::from_slice(&[0xD83Cu16, 0xDC07, 0xD83C, 0xDC10, 0xD83C, 0xDC19]));
}
