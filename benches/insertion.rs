use std::fs::File;

use criterion::{criterion_group, criterion_main, Criterion};
use editorus::editor::session::Session;

fn insertion_benchmark(c: &mut Criterion) {
    let file = File::open("./test.txt").unwrap();
    let mut session = Session::open_file(file).unwrap();
    session.cursor_down();
    session.cursor_right();

    c.bench_function("Insert char", |b| {
        b.iter(|| {
            session.insert(&b"A".to_vec());
        })
    });
}

fn single_line_insertion_benchmark(c: &mut Criterion) {
    let file = File::open("./single.txt").unwrap();
    let mut session = Session::open_file(file).unwrap();
    session.cursor_right();
    session.cursor_right();
    session.cursor_right();
    session.cursor_right();
    session.cursor_right();

    c.bench_function("Insert char in single line file", |b| {
        b.iter(|| {
            session.insert(&b"Check".to_vec());
        })
    });
}

criterion_group!(benches, single_line_insertion_benchmark);
criterion_main!(benches);
