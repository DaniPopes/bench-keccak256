/// Keccak-256 hash function type.
pub type HashFn = fn(input: &[u8], output: &mut [u8; 32]);

/// All the Keccak-256 backends.
///
/// `(name, fn)`
pub const ALL: &[(&str, HashFn)] = &[
    ("sha3", sha3),
    ("tiny-keccak", tiny_keccak),
    ("keccak-asm", keccak_asm),
    ("xkcp", xkcp),
];

/// [`sha3`](::sha3)
#[inline(never)]
pub fn sha3(input: &[u8], output: &mut [u8; 32]) {
    use sha3::Digest;
    let mut h = sha3::Keccak256::new();
    h.update(input);
    h.finalize_into(output.into());
}

/// [`tiny_keccak`](::tiny_keccak)
#[inline(never)]
pub fn tiny_keccak(input: &[u8], output: &mut [u8; 32]) {
    use tiny_keccak::Hasher;
    let mut h = tiny_keccak::Keccak::v256();
    h.update(input);
    h.finalize(output);
}

/// [`keccak_asm`](::keccak_asm)
#[inline(never)]
pub fn keccak_asm(input: &[u8], output: &mut [u8; 32]) {
    use keccak_asm::Digest;
    let mut h = keccak_asm::Keccak256::new();
    h.update(input);
    h.finalize_into(output.into());
}

/// [`xkcp_rs`]
#[inline(never)]
pub fn xkcp(input: &[u8], output: &mut [u8; 32]) {
    xkcp_rs::keccak256(input, output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_all() {
        if ALL.is_empty() {
            panic!("No Keccak-256 backends available");
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
                #[allow(clippy::needless_borrows_for_generic_args)]
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
