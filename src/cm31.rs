use crate::treepp::*;
use crate::{
    karatsuba_small, karatsuba_small_constant, m31_add, m31_double, m31_from_bottom, m31_mul,
    m31_mul_by_constant, m31_neg, m31_sub,
};

/// Push a zero CM31 element.
///
/// Output:
/// - cm31 representing 0: 0, 0
///
pub fn push_cm31_zero() -> Script {
    script! {
        { 0 } { 0 }
    }
}

/// Push a one CM31 element.
///
/// Output:
/// - cm31 representing 1: 0, 1
///
pub fn push_cm31_one() -> Script {
    script! {
        { 0 } { 1 }
    }
}

/// Add two CM31 elements.
///
/// Input:
/// - cm31
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_add() -> Script {
    script! {
        OP_ROT m31_add
        OP_TOALTSTACK m31_add OP_FROMALTSTACK
    }
}

/// Double two CM31 elements.
///
/// Input:
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_double() -> Script {
    script! {
        m31_double OP_TOALTSTACK
        m31_double OP_FROMALTSTACK
    }
}

/// Require two CM31 elements to be equal.
///
/// Input:
/// - cm31
/// - cm31
///
/// Fail if the two CM31 elements are not equal.
///
pub fn cm31_equalverify() -> Script {
    script! {
        OP_ROT OP_EQUALVERIFY
        OP_EQUALVERIFY
    }
}

/// Subtract two CM31 elements.
///
/// Input:
/// - cm31
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_sub() -> Script {
    script! {
        OP_ROT OP_SWAP m31_sub
        OP_TOALTSTACK m31_sub OP_FROMALTSTACK
    }
}

/// Negate a CM31 element.
///
/// Input:
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_neg() -> Script {
    script! {
        m31_neg
        OP_SWAP m31_neg
        OP_SWAP
    }
}

/// Multiply two CM31 elements.
///
/// Input:
/// - cm31
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_mul() -> Script {
    karatsuba_small()
}

/// Multiply a CM31 element with a constant CM31 element.
///
/// Input:
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_mul_by_constant(a2: u32, b2: u32) -> Script {
    karatsuba_small_constant(a2, b2)
}

/// Multiply a CM31 element with an M31 element.
///
/// Input:
/// - cm31
/// - m31
///
/// Output:
/// - cm31
///
pub fn cm31_mul_m31() -> Script {
    script! {
        OP_DUP
        OP_ROT m31_mul OP_TOALTSTACK
        m31_mul OP_FROMALTSTACK
    }
}

/// Multiply a CM31 element with a constant M31 element.
///
/// Input:
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_mul_m31_by_constant(m31: u32) -> Script {
    script! {
        { m31_mul_by_constant(m31) } OP_TOALTSTACK
        { m31_mul_by_constant(m31) } OP_FROMALTSTACK
    }
}

/// Drop a cm31
///
/// Input:
/// - cm31
///
pub fn cm31_drop() -> Script {
    script! {
        OP_2DROP
    }
}

/// Swap two CM31 elements.
///
/// Input:
/// - cm31 representing a
/// - cm31 representing b
///
/// Output:
/// - cm31 representing b
/// - cm31 representing a
///
pub fn cm31_swap() -> Script {
    script! {
        OP_2SWAP
    }
}

/// Duplicate a CM31 element.
///
/// Input:
/// - cm31
///
/// Output:
/// - cm31
/// - cm31
///
pub fn cm31_dup() -> Script {
    script! {
        OP_2DUP
    }
}

/// Move a CM31 element to the altstack.
pub fn cm31_toaltstack() -> Script {
    script! {
        OP_TOALTSTACK OP_TOALTSTACK
    }
}

/// Retrieve a CM31 element from the altstack.
pub fn cm31_fromaltstack() -> Script {
    script! {
        OP_FROMALTSTACK OP_FROMALTSTACK
    }
}

/// Copy a CM31 element from the stack.
pub fn cm31_copy(offset: usize) -> Script {
    let a = offset * 2 + 2 - 1;

    script! {
        for _ in 0..2 {
            { a } OP_PICK
        }
    }
}

/// Roll a CM31 element in the stack.
pub fn cm31_roll(offset: usize) -> Script {
    let a = offset * 2 + 2 - 1;

    script! {
        for _ in 0..2 {
            { a } OP_ROLL
        }
    }
}

/// Copy the second-to-top CM31 element.
pub fn cm31_over() -> Script {
    script! {
        OP_2OVER
    }
}

/// Rotate CM31 elements.
pub fn cm31_rot() -> Script {
    cm31_roll(2)
}

/// Pull a CM31 element from the bottom of the stack.
///
/// Hint:
/// - cm31
///
/// Output:
/// - cm31
///
pub fn cm31_from_bottom() -> Script {
    script! {
        m31_from_bottom
        m31_from_bottom
    }
}

#[cfg(test)]
mod test {
    use crate::treepp::*;
    use crate::{
        cm31_add, cm31_copy, cm31_double, cm31_equalverify, cm31_mul, cm31_mul_by_constant,
        cm31_mul_m31, cm31_mul_m31_by_constant, cm31_roll, cm31_sub,
    };
    use p3_field::extension::Complex;
    use p3_field::{AbstractField, PrimeField32};
    use p3_mersenne_31::Mersenne31;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use std::ops::{Add, Mul, Sub};

    type F = Complex<Mersenne31>;

    #[test]
    fn test_cm31_add() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("cm31 add: {}", cm31_add().len());

        for _ in 0..100 {
            let a = prng.gen::<F>();
            let b = prng.gen::<F>();

            let c = a.add(b);

            let script = script! {
                { a.imag().as_canonical_u32() }
                { a.real().as_canonical_u32() }
                { b.imag().as_canonical_u32() }
                { b.real().as_canonical_u32() }
                cm31_add
                { c.imag().as_canonical_u32() }
                { c.real().as_canonical_u32() }
                cm31_equalverify
                OP_TRUE
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_cm31_double() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("cm31 double: {}", cm31_double().len());

        for _ in 0..100 {
            let a = prng.gen::<F>();
            let c = a.double();

            let script = script! {
                { a.imag().as_canonical_u32() }
                { a.real().as_canonical_u32() }
                cm31_double
                { c.imag().as_canonical_u32() }
                { c.real().as_canonical_u32() }
                cm31_equalverify
                OP_TRUE
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_cm31_sub() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("cm31 sub: {}", cm31_sub().len());

        for _ in 0..100 {
            let a = prng.gen::<F>();
            let b = prng.gen::<F>();

            let c = a.sub(b);

            let script = script! {
                { a.imag().as_canonical_u32() }
                { a.real().as_canonical_u32() }
                { b.imag().as_canonical_u32() }
                { b.real().as_canonical_u32() }
                cm31_sub
                { c.imag().as_canonical_u32() }
                { c.real().as_canonical_u32() }
                cm31_equalverify
                OP_TRUE
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_cm31_mul() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("cm31 mul: {}", cm31_mul().len());

        for _ in 0..100 {
            let a = prng.gen::<F>();
            let b = prng.gen::<F>();

            let c = a.mul(b);

            let script = script! {
                { a.imag().as_canonical_u32() }
                { a.real().as_canonical_u32() }
                { b.imag().as_canonical_u32() }
                { b.real().as_canonical_u32() }
                cm31_mul
                { c.imag().as_canonical_u32() }
                { c.real().as_canonical_u32() }
                cm31_equalverify
                OP_TRUE
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_cm31_mul_by_constant() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let mut total_len = 0;

        for _ in 0..100 {
            let a = prng.gen::<F>();
            let b = prng.gen::<F>();

            let mul_script =
                cm31_mul_by_constant(b.imag().as_canonical_u32(), b.real().as_canonical_u32());

            total_len += mul_script.len();

            let c = a.mul(b);

            let script = script! {
                { a.imag().as_canonical_u32() }
                { a.real().as_canonical_u32() }
                { mul_script.clone() }
                { c.imag().as_canonical_u32() }
                { c.real().as_canonical_u32() }
                cm31_equalverify
                OP_TRUE
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("cm31 mul_by_constant: {}", total_len as f64 / 100.0);
    }

    #[test]
    fn test_cm31_mul_m31() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("cm31 mul_m31: {}", cm31_mul_m31().len());

        for _ in 0..100 {
            let a = prng.gen::<F>();
            let b = prng.gen::<Mersenne31>();

            let c = a.mul(b);

            let script = script! {
                { a.imag().as_canonical_u32() }
                { a.real().as_canonical_u32() }
                { b.as_canonical_u32() }
                cm31_mul_m31
                { c.imag().as_canonical_u32() }
                { c.real().as_canonical_u32() }
                cm31_equalverify
                OP_TRUE
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_cm31_mul_m31_by_constant() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let mut total_len = 0;

        for _ in 0..100 {
            let a = prng.gen::<F>();
            let b = prng.gen::<Mersenne31>();

            let mul_script = cm31_mul_m31_by_constant(b.as_canonical_u32());

            total_len += mul_script.len();

            let c = a.mul(b);

            let script = script! {
                { a.imag().as_canonical_u32() }
                { a.real().as_canonical_u32() }
                { mul_script.clone() }
                { c.imag().as_canonical_u32() }
                { c.real().as_canonical_u32() }
                cm31_equalverify
                OP_TRUE
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("cm31 mul_m31_by_constant: {}", total_len as f64 / 100.0);
    }

    #[test]
    fn test_cm31_copy() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let copy_script = cm31_copy(1);

        let script = script! {
            { a.imag().as_canonical_u32() }
            { a.real().as_canonical_u32() }
            { b.imag().as_canonical_u32() }
            { b.real().as_canonical_u32() }
            { copy_script.clone() }
            { a.imag().as_canonical_u32() }
            { a.real().as_canonical_u32() }
            cm31_equalverify
            { b.imag().as_canonical_u32() }
            { b.real().as_canonical_u32() }
            cm31_equalverify
            { a.imag().as_canonical_u32() }
            { a.real().as_canonical_u32() }
            cm31_equalverify
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_cm31_roll() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let roll_script = cm31_roll(1);

        let script = script! {
            { a.imag().as_canonical_u32() }
            { a.real().as_canonical_u32() }
            { b.imag().as_canonical_u32() }
            { b.real().as_canonical_u32() }
            { roll_script.clone() }
            { a.imag().as_canonical_u32() }
            { a.real().as_canonical_u32() }
            cm31_equalverify
            { b.imag().as_canonical_u32() }
            { b.real().as_canonical_u32() }
            cm31_equalverify
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
