#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

#[test]
fn it_works_0() {
    let inp = [0u8, 0, 1, 1].to_vec();
    let oup = "115S".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_1() {
    let inp = b"ABCDEFGHIJKL".to_vec();
    let oup = "2ERjaFfYv6E4EfgR1".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "HBb7dQEaKrdXjkN".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_3() {
    let inp = b"ABCDEFGHIJ".to_vec();
    let oup = "4fedr2e4UP7vBb".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP58);
    let r1 = _encode_base58(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base58(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
