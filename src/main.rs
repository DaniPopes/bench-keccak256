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
        #[inline(never)]
        fn keccak256(input: &[u8], output: &mut [u8; 32]) {
            use sha3::Digest;
            let mut h = sha3::Sha3_256::new();
            h.update(input);
            h.finalize_into(output.into());
        }
    } else if #[cfg(feature = "tiny-keccak")] {
        #[inline(never)]
        fn keccak256(input: &[u8], output: &mut [u8; 32]) {
            use tiny_keccak::Hasher;
            let mut h = tiny_keccak::Keccak::v256();
            h.update(input);
            h.finalize(output);
        }
    } else {
        compile_error!("Must select a Keccak-256 backend with a feature flag");
    }
}
