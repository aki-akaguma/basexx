#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;
//use rstest::rstest;
//use rstest_reuse::{self, *};

/*
const _CMAP64: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];
*/

#[test]
fn test_ascii_graphic_set_1() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    //
    assert_eq!(ags.len(), 64);
    //
    assert_eq!(ags.get(0), Some(b'A'));
    assert_eq!(ags.get(1), Some(b'B'));
    assert_eq!(ags.get(26), Some(b'a'));
    assert_eq!(ags.get(27), Some(b'b'));
    assert_eq!(ags.get(52), Some(b'0'));
    assert_eq!(ags.get(53), Some(b'1'));
    assert_eq!(ags.get(62), Some(b'+'));
    assert_eq!(ags.get(63), Some(b'/'));
    //
    assert_eq!(ags.position(b'A'), Some(0));
    assert_eq!(ags.position(b'B'), Some(1));
    assert_eq!(ags.position(b'a'), Some(26));
    assert_eq!(ags.position(b'b'), Some(27));
    assert_eq!(ags.position(b'0'), Some(52));
    assert_eq!(ags.position(b'1'), Some(53));
    assert_eq!(ags.position(b'+'), Some(62));
    assert_eq!(ags.position(b'/'), Some(63));
}

#[test]
fn test_ascii_graphic_set_binary_to_ascii() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        buf.push(i);
    }
    let r = ags.binary_to_ascii(&mut buf);
    assert!(r.is_ok());
    assert_eq!(buf, &test_utils::_CMAP64);
}

#[test]
fn test_ascii_graphic_set_ascii_to_binary_1() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut valid = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        valid.push(i);
    }
    let mut buf = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        buf.push(test_utils::_CMAP64[i]);
    }
    let r = ags.ascii_to_binary(&mut buf);
    assert!(r.is_ok());
    assert_eq!(buf, valid);
}
