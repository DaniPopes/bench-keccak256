use anyhow::{bail, Result};
use bench_keccak256::{HashFn, ALL};
use rand::prelude::*;
use std::{env::current_exe, hint::black_box, process::exit};

macro_rules! usage {
    ($($t:tt)*) => {
        bail!("{}", usage(format_args!($($t)*)))
    };
}

fn main() {
    match _main() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}

fn _main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [backend, mode, args @ ..] = &args[..] else {
        usage!("<backend> <mode> [args]...");
    };

    let Some(&(_, hash_fn)) = ALL.iter().find(|&&(f, _)| f == backend) else {
        bail!("Unknown backend: {backend}");
    };

    match mode.as_str() {
        "count" => count(hash_fn, args)?,
        "info" => match backend.as_str() {
            "keccak-asm" => {
                eprintln!("keccak-asm impl: {}", keccak_asm::IMPL);
            }
            "xkcp" => {
                eprintln!(
                    "xkcp impl:       {}",
                    xkcp_rs::ffi::KeccakP1600_implementation.to_str().unwrap(),
                );
            }
            _ => {}
        },
        mode => bail!("Unknown mode: {mode}"),
    }

    Ok(())
}

fn count(hash_fn: HashFn, args: &[String]) -> Result<()> {
    let [n, args @ ..] = args else {
        usage!("count <count> [size]");
    };
    let count = n.parse::<usize>()?;
    let size = match args {
        [] => 32,
        [size] => size.parse()?,
        _ => usage!("count {count} [size]"),
    };

    let mut input = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut input);
    let input = &input[..];
    let output = &mut [0u8; 32];
    for _ in 0..black_box(count) {
        hash_fn(black_box(input), black_box(output));
    }
    black_box(output);

    Ok(())
}

fn usage(rest: std::fmt::Arguments<'_>) -> String {
    let exe = current_exe().unwrap();
    let mut exe = exe.as_path();
    if let Ok(curr_dir) = std::env::current_dir() {
        exe = exe.strip_prefix(curr_dir).unwrap_or(exe);
    }
    format!("Usage: {} {rest}", exe.display())
}
