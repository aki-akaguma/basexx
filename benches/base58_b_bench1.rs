use basexx::*;
use criterion::*;

mod utils;

fn bench_base58_enc(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base58 = Base58B::default();
    c.bench_function("base58_b_enc", |b| b.iter(|| base58.encode(black_box(&v))));
}

fn bench_base58_dec(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base58 = Base58B::default();
    let a = base58.encode(black_box(&v)).unwrap();
    c.bench_function("base58_b_dec", |b| b.iter(|| base58.decode(black_box(&a))));
}

criterion_group!(benches, bench_base58_enc, bench_base58_dec);
criterion_main!(benches);
