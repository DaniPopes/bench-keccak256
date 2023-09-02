use cfg_if::cfg_if;
use rand::prelude::*;

fn main() {
    let n: usize = std::env::args().skip(1).next().unwrap().parse().unwrap();

    let mut input = vec![0u8; n];
    rand::thread_rng().fill_bytes(&mut input);
    let input = &input[..];
    let output = &mut [0u8; 32];

    keccak256(input, output);
}

cfg_if! {
    if #[cfg(feature = "sha3")] {
        use bench_keccak256::sha3 as keccak256;
    } else if #[cfg(feature = "tiny-keccak")] {
        use bench_keccak256::tiny_keccak as keccak256;
    // } else if #[cfg(feature = "sha3-asm")] {
    //     use bench_keccak256::sha3_asm as keccak256;
    } else if #[cfg(feature = "keccak-asm")] {
        use bench_keccak256::keccak_asm as keccak256;
    } else {
        compile_error!("Must select a Keccak-256 backend with a feature flag");
    }
}
