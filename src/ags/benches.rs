use super::*;
use crate::test_utils;
use criterion::*;

#[allow(dead_code)]
pub fn bench_ags_enc(c: &mut Criterion) {
    let mut v = test_utils::read_t4_base64_ascii().as_bytes().to_vec();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    let _ = ags.ascii_to_binary(&mut v);
    c.bench_function("ags_enc", |b| {
        b.iter(|| {
            let mut v = v.clone();
            let _ = ags.binary_to_ascii(&mut v);
        })
    });
}

#[allow(dead_code)]
pub fn bench_ags_dec(c: &mut Criterion) {
    let v = test_utils::read_t4_base64_ascii().as_bytes().to_vec();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP64);
    c.bench_function("ags_dec", |b| {
        b.iter(|| {
            let mut v = v.clone();
            let _ = ags.ascii_to_binary(&mut v);
        })
    });
}
