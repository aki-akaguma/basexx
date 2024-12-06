#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

#[test]
fn it_works_0() {
    let inp = [0u8, 0, 1, 1].to_vec();
    let oup = "226b".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP56);
    let r1 = _encode_base56(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base56(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_1() {
    let inp = b"ABCDEFGHIJKL".to_vec();
    let oup = "4AuuZMqSfnYxvFJ7w".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP56);
    let r1 = _encode_base56(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base56(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_2() {
    let inp = b"ABCDEFGHIJK".to_vec();
    let oup = "UT9ZN6uuGzmJTem".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP56);
    let r1 = _encode_base56(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base56(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
#[test]
fn it_works_3() {
    let inp = b"ABCDEFGHIJ".to_vec();
    let oup = "7mzucA69VmhEMc".to_string();
    let ags = AsciiGraphicSet::with_slice(&_CMAP56);
    let r1 = _encode_base56(&ags, &inp).unwrap();
    assert_eq!(r1, oup);
    let r2 = _decode_base56(&ags, &r1).unwrap();
    assert_eq!(r2, inp);
}
