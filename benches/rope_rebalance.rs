use criterion::{criterion_group, criterion_main, Criterion};
use editorus::rope::rope::Rope;

static LOREM: &'static str = "lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

extern crate editorus;

fn rope_rebalance(c: &mut Criterion) {
    let mut group = c.benchmark_group("REBALANCE");

    group.bench_function("Small rope", |b| {
        b.iter(|| {
            let mut rope = Rope::new();
            rope.append(LOREM);
            rope.rebalance();
        })
    });

    group.bench_function("Medium rope", |b| {
        b.iter(|| {
            let mut rope = Rope::new();
            for _ in 0..500 {
                rope.append(LOREM);
            }
            rope.do_nothing_at();
            rope.rebalance();
        })
    });

    group.bench_function("Big rope", |b| {
        b.iter(|| {
            let mut rope = Rope::new();
            for _ in 0..1500 {
                rope.append(LOREM);
            }
            rope.do_nothing_at();
            rope.rebalance();
        })
    });

    group.bench_function("Large rope", |b| {
        b.iter(|| {
            let mut rope = Rope::new();
            for _ in 0..2500 {
                rope.append(LOREM);
            }
            rope.do_nothing_at();
            rope.rebalance();
        })
    });
}

criterion_group!(benches, rope_rebalance);
criterion_main!(benches);
