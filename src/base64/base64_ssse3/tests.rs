#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

#[test]
fn it_works_0() {
    let inp = [0u8, 0, 1, 1].to_vec();
    let oup = "AAABAQ".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = unsafe { _encode_base64_ssse3(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base64_ssse3(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
#[test]
fn it_works_1() {
    let inp = b"ABCDEFGHIJKL".to_vec();
    let oup = "QUJDREVGR0hJSktM".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = unsafe { _encode_base64_ssse3(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base64_ssse3(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
#[test]
fn it_works_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "QUJDREVGR0hJSks".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = unsafe { _encode_base64_ssse3(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base64_ssse3(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
#[test]
fn it_works_3() {
    let inp = b"ABCDEFGHIJ".to_vec();
    let oup = "QUJDREVGR0hJSg".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP64);
    let r1 = unsafe { _encode_base64_ssse3(&ags, &inp).unwrap() };
    assert_eq!(r1, oup);
    let r2 = unsafe { _decode_base64_ssse3(&ags, &r1).unwrap() };
    assert_eq!(r2, inp);
}
