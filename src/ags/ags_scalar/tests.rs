#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;
//use rstest::rstest;
//use rstest_reuse::{self, *};

#[test]
fn test_ags_binary_to_ascii_scalar() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut buf = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        buf.push(i);
    }
    let r = _binary_to_ascii_scalar(&ags.binmap, &mut buf);
    assert!(r.is_ok());
    assert_eq!(buf, &test_utils::_CMAP64);
}

#[test]
fn test_ags_ascii_to_binary_scalar() {
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let mut valid = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        valid.push(i);
    }
    let mut buf = Vec::<u8>::with_capacity(64);
    for i in 0..64 {
        buf.push(test_utils::_CMAP64[i]);
    }
    let r = _ascii_to_binary_scalar(&ags.a128map, &mut buf);
    assert!(r.is_ok());
    assert_eq!(buf, valid);
}
