use basexx::*;
use criterion::*;

mod utils;

fn bench_base58_1(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base58 = Base58B::default();
    c.bench_function("base58_b_1", |b| b.iter(|| base58.encode(black_box(&v))));
}

fn bench_base58_2(c: &mut Criterion) {
    let v = utils::read_data_t1();
    let base58 = Base58B::default();
    let a = base58.encode(black_box(&v)).unwrap();
    c.bench_function("base58_b_2", |b| b.iter(|| base58.decode(black_box(&a))));
}

criterion_group!(benches, bench_base58_1, bench_base58_2);
criterion_main!(benches);
