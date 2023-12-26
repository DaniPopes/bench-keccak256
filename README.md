# bench-keccak256

Keccak-256 benchmarks.

## Backends

- [xkcp](https://github.com/XKCP/XKCP) (through [xkcp-rs](https://github.com/DaniPopes/xkcp-rs))
- [keccak-asm](https://github.com/DaniPopes/keccak-asm) (wraps [cryptogams](https://github.com/dot-asm/cryptogams), used by OpenSSL)
- [tiny-keccak](https://crates.io/crates/tiny-keccak)
- [sha3](https://crates.io/crates/sha3)

## Results

Defs:
- default: no `RUSTFLAGS` set, `make hyperfine RUSTFLAGS=""`
- native: `-Ctarget-cpu=native`, `make hyperfine`

Results:
- MacBook Pro (14-inch, M1 Max, 2021)
  2. 1.00 ± 0.00 xkcp
  3. 1.02 ± 0.02 keccak-asm
  4. 1.17 ± 0.02 sha3
  5. 1.19 ± 0.02 tiny-keccak
- MacBook Air (15-inch, M2, 2023)
  1. 1.00 ± 0.00 keccak-asm
  2. 1.01 ± 0.01 xkcp
  3. 1.15 ± 0.01 sha3
  4. 1.20 ± 0.01 tiny-keccak
- AMD Ryzen 9 5900X; default
  1. 1.00 ± 0.00 keccak-asm
  2. 1.28 ± 0.03 sha3
  3. 1.29 ± 0.04 tiny-keccak
  4. 1.49 ± 0.04 xkcp
- AMD Ryzen 9 5900X; +AVX
  1. 1.00 ± 0.00 keccak-asm
  2. 1.28 ± 0.05 sha3
  3. 1.28 ± 0.05 tiny-keccak
  4. 1.90 ± 0.06 xkcp
- AMD Ryzen 9 5900X; native (AVX2); keccak-asm manual x86_64 override
  1. 1.00 ± 0.00 keccak-asm
  2. 1.04 ± 0.03 tiny-keccak
  3. 1.08 ± 0.03 sha3
  4. 1.15 ± 0.03 xkcp
- AMD Ryzen 9 5900X; native (AVX2)
  1. 1.00 ± 0.00 tiny-keccak
  2. 1.05 ± 0.04 sha3
  3. 1.11 ± 0.03 xkcp
  4. 1.22 ± 0.04 keccak-asm
- AMD EPYC 9124; default (1)
  1. 1.00 ± 0.00 keccak-asm
  2. 1.22 ± 0.02 tiny-keccak
  3. 1.27 ± 0.02 sha3
  4. 1.66 ± 0.03 xkcp
- AMD EPYC 9124; default (2)
  1. 1.00 ± 0.00 keccak-asm
  2. 1.27 ± 0.05 tiny-keccak
  3. 1.30 ± 0.03 sha3
  4. 1.35 ± 0.03 xkcp
- AMD EPYC 9124; native (AVX2)
  1. 1.00 ± 0.00 xkcp
  2. 1.03 ± 0.04 tiny-keccak
  3. 1.08 ± 0.03 sha3
  4. 1.12 ± 0.02 keccak-asm
- Intel i9-12900K; default
  1. 1.00 ± 0.00 keccak-asm
  2. 1.21 ± 0.02 tiny-keccak
  3. 1.27 ± 0.02 sha3
  4. 1.32 ± 0.02 xkcp
- Intel i9-12900K; native (AVX2)
  1. 1.00 ± 0.00 tiny-keccak
  2. 1.05 ± 0.03 sha3
  3. 1.20 ± 0.03 xkcp
  4. 1.25 ± 0.03 keccak-asm
- AMD Ryzen 9 7950X; default
  1. 1.00 ± 0.00 keccak-asm
  2. 1.27 ± 0.04 tiny-keccak
  3. 1.27 ± 0.03 sha3
  4. 1.33 ± 0.03 xkcp
- AMD Ryzen 9 7950X; native (AVX512VL); keccak-asm manual x86_64 override
  1. 1.00 ± 0.00 keccak-asm
  2. 1.03 ± 0.03 tiny-keccak
  3. 1.06 ± 0.04 sha3
  4. 1.06 ± 0.04 xkcp
- AMD Ryzen 9 7950X; native (AVX512VL); keccak-asm manual AVX2 override
  1. 1.00 ± 0.00 tiny-keccak
  2. 1.01 ± 0.03 xkcp
  3. 1.03 ± 0.03 sha3
  4. 1.06 ± 0.04 keccak-asm
- AMD Ryzen 9 7950X; native (AVX512VL)
  1. 1.00 ± 0.00 keccak-asm
  2. 1.04 ± 0.03 tiny-keccak
  3. 1.06 ± 0.03 xkcp
  4. 1.09 ± 0.03 sha3

## Takeaways

- keccak-asm
  - armv8 outperforms everything else by >15%
  - x86_64 without AVX2 outperforms everything else by >20%; with AVX it's slightly faster than tiny-keccak
  - AVX2 and AVX512(F) are way slower than the regular x86_64 version, they might be obsolete by now; just use plain x86_64
  - AVX512VL is slightly (3-5%) faster than plain X86 on AMD, probably better on Intel
- `sha3/asm` feature does not impact `sha3` performance. It enables assembly backend for the [`keccak`](https://crates.io/crates/keccak) crate but it doesn't seem to be meaningful enough
- tiny_keccak outperforms or is on par with sha3
