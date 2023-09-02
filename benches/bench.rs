use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::prelude::*;
use std::time::Duration;

fn bench_all(c: &mut Criterion) {
    let counts: &[usize] = &[32, 128, 1024, 4096, 16384, 65536, 262144, 1048576];
    let max_sz = *counts.iter().max().unwrap();
    let mut buffer = vec![0u8; max_sz];
    let output = &mut [0u8; 32];

    let rng = &mut rand::thread_rng();
    for &(name, hash_fn) in bench_keccak256::ALL {
        let mut g = c.benchmark_group(name);
        // g.sample_size(100);
        g.warm_up_time(Duration::from_secs(1));
        g.measurement_time(Duration::from_secs(5));
        g.noise_threshold(0.02);

        for &count in counts {
            assert!(count <= max_sz);
            let input = &mut buffer[..max_sz];

            g.throughput(Throughput::Bytes(count as u64));
            g.bench_function(BenchmarkId::from_parameter(count), |b| {
                b.iter(|| {
                    rng.fill_bytes(input);
                    hash_fn(input, output);
                });
            });
        }
    }
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
