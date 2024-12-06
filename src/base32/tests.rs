#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

#[test]
fn it_works_0() {
    let inp = [0u8, 0, 1, 1].to_vec();
    let oup = "AAAACAI".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = _encode_base32(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base32(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_1() {
    let inp = b"ABCDEFGHIJKL".to_vec();
    let oup = "IFBEGRCFIZDUQSKKJNGA".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = _encode_base32(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base32(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "IFBEGRCFIZDUQSKKJM".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = _encode_base32(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base32(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_3() {
    let inp = b"ABCDEFGHIJ".to_vec();
    let oup = "IFBEGRCFIZDUQSKK".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP32);
    let r1 = _encode_base32(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base32(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
