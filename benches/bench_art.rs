use std::process::Output;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lettuce::wavl::WAVLTree;
use pprof::validate;

fn benchmark_insert(c: &mut Criterion) {
    c.bench_function("art insert", |b| {
        b.iter(|| {
            let mut tree = WAVLTree::new();
            for i in 0..1000 {
                tree.insert(black_box(i), i);
            }
        });
    });
}

fn benchmark_contains(c: &mut Criterion) {
    c.bench_function("art find", |b| {
        b.iter(|| {
            let mut tree = WAVLTree::new();
            for i in 0..1000 {
                tree.insert(i, i);
            }
            // tree.contains(black_box(&500));
        });
    });
}

criterion_group!(benches, benchmark_insert, benchmark_contains);

criterion_main!(benches);
