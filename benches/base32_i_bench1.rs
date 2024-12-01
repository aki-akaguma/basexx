use basexx::*;
use criterion::*;

mod utils;

fn bench_base32_enc(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base32 = Base32I::default();
    c.bench_function("base32_i_enc", |b| b.iter(|| base32.encode(black_box(&v))));
}

fn bench_base32_dec(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base32 = Base32I::default();
    let a = base32.encode(&v).unwrap();
    c.bench_function("base32_i_dec", |b| b.iter(|| base32.decode(black_box(&a))));
}

criterion_group!(benches, bench_base32_enc, bench_base32_dec);
criterion_main!(benches);
