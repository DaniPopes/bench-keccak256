use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use std::hint::black_box;
use std::time::Duration;

fn bench_all(c: &mut Criterion) {
    let counts: &[usize] = &[32, 128, 1024, 16384, 131072, 1048576, 16777216];
    let max_sz = *counts.iter().max().unwrap();
    let mut buffer = vec![0u8; max_sz];
    let output = &mut [0u8; 32];

    let rng = &mut rand::rng();
    for &(name, hash_fn) in bench_keccak256::ALL {
        let mut g = c.benchmark_group(name);
        g.sample_size(50);
        g.warm_up_time(Duration::from_secs(3));
        g.measurement_time(Duration::from_secs(10));
        g.noise_threshold(0.02);

        for &count in counts {
            assert!(count <= max_sz);
            let input = &mut buffer[..max_sz];
            rng.fill_bytes(input);

            // g.throughput(criterion::Throughput::Bytes(count as u64));
            g.bench_function(BenchmarkId::from_parameter(count), |b| {
                b.iter(|| hash_fn(black_box(input), black_box(output)));
            });
        }
    }
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
