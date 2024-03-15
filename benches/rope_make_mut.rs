use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use editorus::rope::{self, rope::Rope};

static LOREM: &'static str = "lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

fn make_mut(c: &mut Criterion) {
    let mut group = c.benchmark_group("MAKE_MUT");

    group.bench_function("Empty rope", |b| {
        let mut rope = Rope::new();

        b.iter(|| {
            let _ = Arc::make_mut(&mut rope.root);
        })
    });

    group.bench_function("Small rope", |b| {
        let mut rope = Rope::new();
        rope.append(LOREM);

        b.iter(|| {
            let _ = Arc::make_mut(&mut rope.root);
        })
    });

    group.bench_function("Medium rope", |b| {
        let mut rope = Rope::new();

        for _ in 0..500 {
            rope.append(LOREM);
        }

        b.iter(|| {
            let _ = Arc::make_mut(&mut rope.root);
        })
    });

    group.bench_function("Large rope", |b| {
        let mut rope = Rope::new();

        for _ in 0..2500 {
            rope.append(LOREM);
        }

        b.iter(|| {
            let _ = Arc::make_mut(&mut rope.root);
        })
    });
}

criterion_group!(benches, make_mut);
criterion_main!(benches);
