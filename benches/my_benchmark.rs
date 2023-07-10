use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maloc_vec::m_vec::*;

fn criterion_benchmark_add_elements(c: &mut Criterion) {
    c.bench_function("add_elements", |b| {
        let mut mvec: MVec<i32> = MVec::new();
        b.iter(|| {
            mvec.add(black_box(42));
        });
    });
}

criterion_group!(benches, criterion_benchmark_add_elements);
criterion_main!(benches);
