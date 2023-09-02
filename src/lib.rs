/// Keccak-256 hash function type.
pub type HashFn = fn(input: &[u8], output: &mut [u8; 32]);

/// All the Keccak-256 backends.
///
/// `(name, fn)`
pub const ALL: &[(&str, HashFn)] = &[
    #[cfg(feature = "sha3")]
    ("sha3", sha3),
    #[cfg(feature = "tiny-keccak")]
    ("tiny-keccak", tiny_keccak),
    // #[cfg(feature = "sha3-asm")]
    // ("sha3-asm", sha3_asm),
    #[cfg(feature = "keccak-asm")]
    ("keccak-asm", keccak_asm),
];

/// [`sha3`](::sha3)
#[inline(never)]
#[cfg(feature = "sha3")]
pub fn sha3(input: &[u8], output: &mut [u8; 32]) {
    use sha3::Digest;
    let mut h = sha3::Keccak256::new();
    h.update(input);
    h.finalize_into(output.into());
}

/// [`tiny_keccak`](::tiny_keccak)
#[inline(never)]
#[cfg(feature = "tiny-keccak")]
pub fn tiny_keccak(input: &[u8], output: &mut [u8; 32]) {
    use tiny_keccak::Hasher;
    let mut h = tiny_keccak::Keccak::v256();
    h.update(input);
    h.finalize(output);
}

// /// [`sha3_asm`](::sha3_asm)
// #[inline(never)]
// #[cfg(feature = "sha3-asm")]
// pub fn sha3_asm(input: &[u8], output: &mut [u8; 32]) {
//     use sha3_asm::Digest;
//     let mut h = sha3_asm::Keccak256::new();
//     h.update(input);
//     h.finalize_into(output.into());
// }

/// [`keccak_asm`](::keccak_asm)
#[inline(never)]
#[cfg(feature = "keccak-asm")]
pub fn keccak_asm(input: &[u8], output: &mut [u8; 32]) {
    use keccak_asm::Digest;
    let mut h = keccak_asm::Keccak256::new();
    h.update(input);
    h.finalize_into(output.into());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_all() {
        if bench_keccak256::ALL.is_empty() {
            panic!("No Keccak-256 backends selected");
        }

        let max_sz = 1024;
        let cnt = 100;

        let mut input = vec![0u8; max_sz];
        let output = &mut [0u8; 32];

        let mut set = std::collections::HashSet::new();
        let rng = &mut rand::thread_rng();
        for _ in 0..cnt {
            let sz = rng.gen_range(0..max_sz);
            let input = &mut input[..sz];
            rng.fill_bytes(input);

            for &(_name, hash_fn) in ALL {
                hash_fn(input, output);
                set.insert(*output);
            }

            if set.len() != 1 {
                eprintln!("input: {}", hex::encode(&*input));
                for &(name, hash_fn) in ALL {
                    hash_fn(input, output);
                    eprintln!("- {name}() -> {}", hex::encode(&*output));
                }
                panic!("Hashes do not match");
            }

            set.clear();
        }
    }
}
