use bench_keccak256::{HashFn, ALL};
use rand::prelude::*;
use std::{env::current_exe, hint::black_box, process::exit};

#[allow(unused_macros)]
macro_rules! time {
    ($desc:expr, $runs:expr, $val:expr) => {
        match (::std::time::Instant::now(), $val) {
            (timer, res) => {
                let elapsed = timer.elapsed();
                eprintln!(
                    "[{}:{}] \"{}\" ran in {:.6}s ({}ns per run, {} runs)",
                    file!(),
                    line!(),
                    $desc,
                    elapsed.as_secs_f32(),
                    elapsed.as_nanos() / ($runs as u128),
                    $runs,
                );
                res
            }
        }
    };
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [backend, mode, args @ ..] = &args[..] else {
        eprintln!(
            "Usage: {} <backend> <mode> [args]...",
            current_exe().unwrap().display()
        );
        exit(1);
    };

    let Some(&(_, hash_fn)) = ALL.iter().find(|&&(f, _)| f.eq_ignore_ascii_case(backend)) else {
        eprintln!("Unknown backend: {backend}");
        exit(1);
    };

    match mode.as_str() {
        "size" => size(hash_fn, args),
        "count" => count(hash_fn, args),
        "info" => {
            if backend == "keccak-asm" && ALL.iter().any(|x| x.0 == "keccak-asm") {
                eprintln!("keccak-asm impl: {}", keccak_asm::IMPL);
            }
        }
        mode => {
            eprintln!("Unknown mode: {mode}");
            exit(1);
        }
    }
}

fn size(hash_fn: HashFn, args: &[String]) {
    let [n] = args else {
        eprintln!("Usage: {} size <size>", current_exe().unwrap().display());
        exit(1);
    };
    let n: usize = n.parse().unwrap();
    let mut input = vec![0u8; n];
    rand::thread_rng().fill_bytes(&mut input);
    let output = &mut [0u8; 32];
    hash_fn(black_box(&input[..]), black_box(output));
    black_box(output);
}

fn count(hash_fn: HashFn, args: &[String]) {
    let [n] = args else {
        eprintln!("Usage: {} count <count>", current_exe().unwrap().display());
        exit(1);
    };
    let n: usize = n.parse().unwrap();
    let mut input = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut input);
    let input = &input[..];
    let output = &mut [0u8; 32];
    for _ in 0..black_box(n) {
        hash_fn(black_box(input), black_box(output));
    }
    black_box(output);
}
