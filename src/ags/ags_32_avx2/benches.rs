use super::*;
use crate::test_utils;
use criterion::*;

#[allow(dead_code)]
pub fn bench_ags_32_avx2_enc(c: &mut Criterion) {
    if !is_x86_feature_detected!("avx2") {
        return;
    }
    let mut v = test_utils::read_t4_base32_ascii().as_bytes().to_vec();
    let ags = AsciiGraphicSet::with_slice(&test_utils::_CMAP32);
    let _ = ags.ascii_to_binary(&mut v);
    c.bench_function("ags_32_avx2_enc", |b| {
        b.iter(|| {
            let mut v = v.clone();
            //let _ = ags.binary_to_ascii(&mut v);
            let _ = unsafe { _binary_to_ascii_32_avx2(&ags.binmap, &mut v) };
        })
    });
}
