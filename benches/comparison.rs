use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::BinaryHeap;

use skewheap::SkewHeap;

fn skew_heap(c: &mut Criterion) {
    let mut group = c.benchmark_group("skew heap");

    let counts = [100, 500, 1000, 5000];

    for count in counts {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let mut items: Vec<u32> = (0..count).collect();
            items.shuffle(&mut thread_rng());

            b.iter(|| {
                let mut s = SkewHeap::new();

                for n in &items {
                    s.put(n);
                }

                while !s.is_empty() {
                    s.take();
                }
            })
        });
    }

    group.finish();
}

fn binary_heap(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary heap");

    let counts = [100, 500, 1000, 5000];

    for count in counts {
        group.throughput(Throughput::Elements(count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            let mut items: Vec<u32> = (0..count).collect();
            items.shuffle(&mut thread_rng());

            b.iter(|| {
                let mut b = BinaryHeap::new();

                for n in &items {
                    b.push(n);
                }

                while !b.is_empty() {
                    b.pop();
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, skew_heap, binary_heap);
criterion_main!(benches);
