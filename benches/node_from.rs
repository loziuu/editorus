use criterion::{criterion_group, criterion_main, Criterion};
use editorus::rope::node::Node;

static LOREM: &'static str = "lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

extern crate editorus;

fn node_from(c: &mut Criterion) {
    let mut group = c.benchmark_group("NODE_FROM");

    group.bench_function("small str", |b| {
        b.iter(|| {
            let _ = Node::from("Small");
        })
    });

    group.bench_function("medium str, but less than MAX LEAF", |b| {
        b.iter(|| {
            let _ = Node::from(LOREM);
        })
    });

    group.bench_function("bigger than max lefa", |b| {
        let mut val = format!("{}{}", LOREM, LOREM);
        for _ in 0..10 {
            val = format!("{}{}", val, val);
        }
        b.iter(|| {
            let _ = Node::from(val.as_str());
        })
    });
}

criterion_group!(benches, node_from);
criterion_main!(benches);
