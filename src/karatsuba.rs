use crate::treepp::*;
use crate::{m31_add, m31_mul, m31_mul_by_constant, m31_sub, MOD};

/// Algorithm for doing Karatsuba for 2 elements, where both elements are from the stack.
///
/// Input:
/// - A1 B1
/// - A2 B2
///
/// Output:
/// - A1B2 + A2B1
/// - B1B2 - A1A2
///
pub fn karatsuba_small() -> Script {
    script! {
        OP_OVER 4 OP_PICK
        m31_mul
        OP_TOALTSTACK
        OP_DUP
        3 OP_PICK
        m31_mul
        OP_TOALTSTACK
        m31_add
        OP_TOALTSTACK
        m31_add
        OP_FROMALTSTACK
        m31_mul
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_2DUP
        m31_add
        3 OP_ROLL
        OP_SWAP
        m31_sub
        OP_TOALTSTACK
        m31_sub
        OP_FROMALTSTACK
        OP_SWAP
    }
}
/// Algorithm for doing Karatsuba for 2 elements, where the other element is a constant.
///
/// Input:
/// - A1
/// - B1
///
/// Output:
/// - A1B2 + A2B1
/// - B1B2 - A1A2
///
pub fn karatsuba_small_constant(a2: u32, b2: u32) -> Script {
    script! {
        // compute A1A2
        OP_OVER
        { m31_mul_by_constant(a2) }
        OP_TOALTSTACK

        // compute B1B2
        OP_DUP
        { m31_mul_by_constant(b2) }
        OP_TOALTSTACK

        // stack: A1 B1
        // altstack: A1A2 B1B2

        // compute s = (A1 + B1) * (A2 + B2) = A1A2 + A1B2 + A2B1 + B1B2
        m31_add
        { m31_mul_by_constant((a2 + b2) % MOD) }

        OP_FROMALTSTACK OP_FROMALTSTACK
        OP_2DUP m31_add

        // stack: s, B1B2, A1A2, A1A2+B1B2

        // compute B1B2 - A1A2
        OP_TOALTSTACK
        m31_sub

        OP_SWAP OP_FROMALTSTACK
        // stack: a3, s, A1A2+B1B2
        m31_sub
        OP_SWAP
    }
}

/// Algorithm for doing Karatsuba for 4 elements, where both elements are given in the stack.
///
/// Input:
/// - A1 B1 C1 D1
/// - A2 B2 C2 D2
///
/// Output:
/// - (A1, B1) * (A2, B2) - 2 elements
/// - (A1, B1) * (C2, D2) + (A2, B2) * (C1, D1) - 2 elements
/// - (C1, D1) * (C2, D2) - 2 elements
///
pub fn karatsuba_big() -> Script {
    script! {
        7 OP_PICK
        7 OP_PICK
        5 OP_PICK
        5 OP_PICK
        karatsuba_small
        OP_TOALTSTACK
        OP_TOALTSTACK
        OP_2DUP
        7 OP_PICK
        7 OP_PICK
        karatsuba_small
        OP_TOALTSTACK
        OP_TOALTSTACK
        OP_ROT
        m31_add
        OP_TOALTSTACK
        m31_add
        OP_TOALTSTACK
        OP_ROT
        m31_add
        OP_TOALTSTACK
        m31_add
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        karatsuba_small
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        5 OP_ROLL
        2 OP_PICK
        5 OP_PICK
        m31_add
        m31_sub
        5 OP_ROLL
        2 OP_PICK
        5 OP_PICK
        m31_add
        m31_sub
        5 OP_ROLL
        5 OP_ROLL
    }
}

/// Algorithm for doing Karatsuba for 4 elements, where the other element is a constant.
///
/// Input:
/// - A1 B1 C1 D1
///
/// Output:
/// - (A1, B1) * (A2, B2) - 2 elements
/// - (A1, B1) * (C2, D2) + (A2, B2) * (C1, D1) - 2 elements
/// - (C1, D1) * (C2, D2) - 2 elements
///
pub fn karatsuba_big_constant(a2: u32, b2: u32, c2: u32, d2: u32) -> Script {
    script! {
        // compute (A1, B1) * (A2, B2), which would end up 2 elements
        3 OP_PICK
        3 OP_PICK
        { karatsuba_small_constant(a2, b2) }
        OP_TOALTSTACK OP_TOALTSTACK

        // compute (C1, D1) * (C2, D2), which would end up 2 elements
        OP_OVER OP_OVER
        { karatsuba_small_constant(c2, d2) }
        OP_TOALTSTACK OP_TOALTSTACK

        // compute (A1 + C1, B1 + D1)
        OP_ROT m31_add
        OP_TOALTSTACK m31_add OP_FROMALTSTACK

        { karatsuba_small_constant((a2 + c2) % MOD, (b2 + d2) % MOD) }

        // stack: (A1 + C1, B1 + D1) * (A2 + C2, B2 + D2) (2 elements)
        // altstack: (A1, B1) * (A2, B2) (2 elements), (C1, D1) * (C2, D2) (2 elements)

        OP_FROMALTSTACK OP_FROMALTSTACK OP_2DUP

        4 OP_ROLL OP_SWAP m31_sub
        OP_SWAP 4 OP_ROLL OP_SWAP m31_sub OP_SWAP

        OP_FROMALTSTACK OP_FROMALTSTACK OP_2DUP
        4 OP_ROLL OP_SWAP m31_sub
        OP_SWAP 4 OP_ROLL OP_SWAP m31_sub OP_SWAP

        // stack: (C1, D1) * (C2, D2), (A1, B1) * (A2, B2), (A1, B1) * (C2, D2) + (A2, B2) * (C1, D1)

        // adjust the order
        5 OP_ROLL
        5 OP_ROLL
    }
}

#[cfg(test)]
mod test {
    use crate::treepp::*;
    use crate::{karatsuba_big, karatsuba_big_constant, karatsuba_small, karatsuba_small_constant};
    use core::ops::{Add, Mul, Sub};
    use p3_field::extension::Complex;
    use p3_field::PrimeField32;
    use p3_mersenne_31::Mersenne31 as P3M31;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_small_karatsuba() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a1: P3M31 = prng.random();
            let b1: P3M31 = prng.random();
            let a2: P3M31 = prng.random();
            let b2: P3M31 = prng.random();

            let first = a1.mul(b2).add(a2.mul(b1));
            let second = b1.mul(b2).sub(a1.mul(a2));

            let script = script! {
                { a1.as_canonical_u32() } { b1.as_canonical_u32() } { a2.as_canonical_u32() } { b2.as_canonical_u32() }
                karatsuba_small
                { second.as_canonical_u32() }
                OP_EQUALVERIFY
                { first.as_canonical_u32() }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_small_karatsuba_constant() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a1: P3M31 = prng.random();
            let b1: P3M31 = prng.random();
            let a2: P3M31 = prng.random();
            let b2: P3M31 = prng.random();

            let first = a1.mul(b2).add(a2.mul(b1));
            let second = b1.mul(b2).sub(a1.mul(a2));

            let script = script! {
                { a1.as_canonical_u32() } { b1.as_canonical_u32() }
                { karatsuba_small_constant(a2.as_canonical_u32(), b2.as_canonical_u32()) }
                { second.as_canonical_u32() }
                OP_EQUALVERIFY
                { first.as_canonical_u32() }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_big_karatsuba() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a1: P3M31 = prng.random();
            let b1: P3M31 = prng.random();
            let c1: P3M31 = prng.random();
            let d1: P3M31 = prng.random();

            let a2: P3M31 = prng.random();
            let b2: P3M31 = prng.random();
            let c2: P3M31 = prng.random();
            let d2: P3M31 = prng.random();

            let group1_first = a1.mul(b2).add(a2.mul(b1));
            let group1_second = b1.mul(b2).sub(a1.mul(a2));

            let group3_first = c1.mul(d2).add(c2.mul(d1));
            let group3_second = d1.mul(d2).sub(c1.mul(c2));

            let group2_first = a1.mul(d2).add(b1.mul(c2)).add(a2.mul(d1)).add(b2.mul(c1));
            let group2_second = b1.mul(d2).add(b2.mul(d1)).sub(a1.mul(c2).add(a2.mul(c1)));

            let script = script! {
                { a1.as_canonical_u32() } { b1.as_canonical_u32() } { c1.as_canonical_u32() } { d1.as_canonical_u32() }
                { a2.as_canonical_u32() } { b2.as_canonical_u32() } { c2.as_canonical_u32() } { d2.as_canonical_u32() }
                karatsuba_big
                { group3_second.as_canonical_u32() }
                OP_EQUALVERIFY
                { group3_first.as_canonical_u32() }
                OP_EQUALVERIFY
                { group2_second.as_canonical_u32() }
                OP_EQUALVERIFY
                { group2_first.as_canonical_u32() }
                OP_EQUALVERIFY
                { group1_second.as_canonical_u32() }
                OP_EQUALVERIFY
                { group1_first.as_canonical_u32() }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_big_karatsuba_constant() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a1: P3M31 = prng.random();
            let b1: P3M31 = prng.random();
            let c1: P3M31 = prng.random();
            let d1: P3M31 = prng.random();

            let a2: P3M31 = prng.random();
            let b2: P3M31 = prng.random();
            let c2: P3M31 = prng.random();
            let d2: P3M31 = prng.random();

            let group1_first = a1.mul(b2).add(a2.mul(b1));
            let group1_second = b1.mul(b2).sub(a1.mul(a2));

            let group3_first = c1.mul(d2).add(c2.mul(d1));
            let group3_second = d1.mul(d2).sub(c1.mul(c2));

            let group2_first = a1.mul(d2).add(b1.mul(c2)).add(a2.mul(d1)).add(b2.mul(c1));
            let group2_second = b1.mul(d2).add(b2.mul(d1)).sub(a1.mul(c2).add(a2.mul(c1)));

            let script = script! {
                { a1.as_canonical_u32() } { b1.as_canonical_u32() } { c1.as_canonical_u32() } { d1.as_canonical_u32() }
                { karatsuba_big_constant(a2.as_canonical_u32(), b2.as_canonical_u32(), c2.as_canonical_u32(), d2.as_canonical_u32()) }
                { group3_second.as_canonical_u32() }
                OP_EQUALVERIFY
                { group3_first.as_canonical_u32() }
                OP_EQUALVERIFY
                { group2_second.as_canonical_u32() }
                OP_EQUALVERIFY
                { group2_first.as_canonical_u32() }
                OP_EQUALVERIFY
                { group1_second.as_canonical_u32() }
                OP_EQUALVERIFY
                { group1_first.as_canonical_u32() }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_small_karatsuba_consistency() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a: Complex<p3_mersenne_31::Mersenne31> = rng.random();
            let b: Complex<p3_mersenne_31::Mersenne31> = rng.random();
            let c = a.mul(b);

            let script = script! {
                { a.imag().as_canonical_u32() } { a.real().as_canonical_u32() }
                { b.imag().as_canonical_u32() } { b.real().as_canonical_u32() }
                karatsuba_small
                { c.real().as_canonical_u32() }
                OP_EQUALVERIFY
                { c.imag().as_canonical_u32() }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }
}
