use crate::treepp::*;
use crate::{m31_from_bottom, m31_mul_by_constant, m31_neg, m31_to_bits};

pub use crate::karatsuba_complex::*;

use crate::m31::{m31_add, m31_double, m31_mul_common, m31_sub};

pub fn push_qm31_zero() -> Script {
    script! {
        { 0 }
        { 0 }
        { 0 }
        { 0 }
    }
}

pub fn push_qm31_one() -> Script {
    script! {
        { 0 }
        { 0 }
        { 0 }
        { 1 }
    }
}

pub fn qm31_from_bottom() -> Script {
    script! {
        m31_from_bottom
        m31_from_bottom
        m31_from_bottom
        m31_from_bottom
    }
}

pub fn qm31_add() -> Script {
    script! {
        for i in 0..3 {
            { 4 - i } OP_ROLL
            m31_add
            OP_TOALTSTACK
        }
        m31_add
        for _ in 0..3 {
            OP_FROMALTSTACK
        }
    }
}

pub fn qm31_equalverify() -> Script {
    script! {
        for i in 0..3 {
            { 4 - i } OP_ROLL
            OP_EQUALVERIFY
        }
        OP_EQUALVERIFY
    }
}

pub fn qm31_sub() -> Script {
    script! {
        for i in 0..3 {
            { 4 - i } OP_ROLL OP_SWAP
            m31_sub
            OP_TOALTSTACK
        }
        m31_sub
        for _ in 0..3 {
            OP_FROMALTSTACK
        }
    }
}

pub fn qm31_neg() -> Script {
    script! {
        m31_neg OP_TOALTSTACK
        m31_neg OP_TOALTSTACK
        m31_neg OP_TOALTSTACK
        m31_neg
        OP_FROMALTSTACK OP_FROMALTSTACK OP_FROMALTSTACK
    }
}

pub fn qm31_double() -> Script {
    script! {
        for _ in 0..3 {
            m31_double
            OP_TOALTSTACK
        }
        m31_double
        for _ in 0..3 {
            OP_FROMALTSTACK
        }
    }
}

pub fn qm31_square() -> Script {
    script! {
        { qm31_copy(0) }
        qm31_mul
    }
}

pub fn qm31_mul() -> Script {
    script! {
        karatsuba_complex_big
        4 OP_ROLL
        OP_DUP
        m31_double
        6 OP_ROLL
        OP_DUP
        m31_double
        OP_ROT
        OP_ROT
        m31_sub
        3 OP_ROLL
        m31_add
        OP_ROT
        OP_ROT
        m31_add
        OP_ROT
        m31_add
        OP_SWAP
    }
}

pub fn qm31_mul_by_constant(a2: u32, b2: u32, c2: u32, d2: u32) -> Script {
    script! {
        { karatsuba_complex_big_constant(a2, b2, c2, d2) }
        4 OP_ROLL
        OP_DUP
        m31_double
        6 OP_ROLL
        OP_DUP
        m31_double
        OP_ROT
        OP_ROT
        m31_sub
        3 OP_ROLL
        m31_add
        OP_ROT
        OP_ROT
        m31_add
        OP_ROT
        m31_add
        OP_SWAP
    }
}

pub fn qm31_mul_m31() -> Script {
    // input stack:
    //
    // u31ext
    // d, c, b, a
    //
    // m31
    // e

    script! {
        m31_to_bits

        // duplicate 3 times
        for _ in 0..31 {
            30 OP_PICK
        }
        for _ in 0..31 {
            OP_TOALTSTACK
        }

        for _ in 0..31 {
            30 OP_PICK
        }
        for _ in 0..31 {
            OP_TOALTSTACK
        }

        for _ in 0..31 {
            30 OP_PICK
        }
        for _ in 0..31 {
            OP_TOALTSTACK
        }

        for _ in 0..31 {
            OP_TOALTSTACK
        }

        // d
        3 OP_ROLL
        m31_mul_common

        // c
        3 OP_ROLL
        m31_mul_common

        // b
        3 OP_ROLL
        m31_mul_common

        // a
        3 OP_ROLL
        m31_mul_common
    }
}

pub fn qm31_mul_m31_by_constant(constant: u32) -> Script {
    // input stack:
    //
    // u31ext
    // d, c, b, a

    script! {
        OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK
        { m31_mul_by_constant(constant) }
        OP_FROMALTSTACK
        { m31_mul_by_constant(constant) }
        OP_FROMALTSTACK
        { m31_mul_by_constant(constant) }
        OP_FROMALTSTACK
        { m31_mul_by_constant(constant) }
    }
}

pub fn qm31_toaltstack() -> Script {
    script! {
        for _ in 0..4 {
            OP_TOALTSTACK
        }
    }
}

pub fn qm31_fromaltstack() -> Script {
    script! {
        for _ in 0..4 {
            OP_FROMALTSTACK
        }
    }
}

pub fn qm31_copy(offset: usize) -> Script {
    let a = offset * 4 + 4 - 1;

    script! {
        for _ in 0..4 {
            { a } OP_PICK
        }
    }
}

pub fn qm31_roll(offset: usize) -> Script {
    let a = offset * 4 + 4 - 1;

    script! {
        for _ in 0..4 {
            { a } OP_ROLL
        }
    }
}

pub fn qm31_dup() -> Script {
    qm31_copy(0)
}

pub fn qm31_swap() -> Script {
    qm31_roll(1)
}

pub fn qm31_over() -> Script {
    qm31_copy(1)
}

pub fn qm31_rot() -> Script {
    qm31_roll(2)
}

#[cfg(test)]
mod test {
    use crate::{
        qm31_add, qm31_copy, qm31_equalverify, qm31_mul, qm31_mul_m31, qm31_mul_m31_by_constant,
        qm31_roll, qm31_sub,
    };
    use core::ops::{Add, Mul, Neg};
    use p3_field::extension::Complex;
    use p3_field::{AbstractExtensionField, AbstractField, PrimeField32};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    type F = p3_field::extension::BinomialExtensionField<Complex<p3_mersenne_31::Mersenne31>, 2>;

    #[test]
    fn test_qm31_add() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 add: {}", qm31_add().len());

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<F>();

            let c = a.add(b);

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { b[1].imag().as_canonical_u32() }
                { b[1].real().as_canonical_u32() }
                { b[0].imag().as_canonical_u32() }
                { b[0].real().as_canonical_u32() }
                qm31_add
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_PUSHNUM_1
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_qm31_double() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let c = a.double();

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                qm31_double
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_PUSHNUM_1
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_qm31_sub() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 sub: {}", qm31_sub().len());

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<F>();
            let c = a.add(b.neg());

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { b[1].imag().as_canonical_u32() }
                { b[1].real().as_canonical_u32() }
                { b[0].imag().as_canonical_u32() }
                { b[0].real().as_canonical_u32() }
                qm31_sub
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_PUSHNUM_1
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_qm31_neg() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 neg: {}", qm31_neg().len());

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let c = a.neg();

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                qm31_neg
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_PUSHNUM_1
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_qm31_square() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 square: {}", qm31_square().len());

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let c = a.square();

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                qm31_square
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_PUSHNUM_1
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_qm31_mul() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 mul: {}", qm31_mul().len());

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<F>();
            let c = a.mul(b);

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { b[1].imag().as_canonical_u32() }
                { b[1].real().as_canonical_u32() }
                { b[0].imag().as_canonical_u32() }
                { b[0].real().as_canonical_u32() }
                qm31_mul
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_PUSHNUM_1
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_qm31_mul_by_constant() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let mut total_len = 0;

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<F>();
            let c = a.mul(b);

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let mul_script = qm31_mul_by_constant(
                b[1].imag().as_canonical_u32(),
                b[1].real().as_canonical_u32(),
                b[0].imag().as_canonical_u32(),
                b[0].real().as_canonical_u32(),
            );
            total_len += mul_script.len();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { mul_script.clone() }
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_PUSHNUM_1
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("qm31 mul_by_constant: {}", total_len as f64 / 100.0);
    }

    #[test]
    fn test_qm31_mul_m31() {
        let mul_script = qm31_mul_m31();

        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 mul_by_m31: {}", mul_script.len());

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<p3_mersenne_31::Mersenne31>();

            let c = a * F::new(
                Complex::<p3_mersenne_31::Mersenne31>::new(b, p3_mersenne_31::Mersenne31::zero()),
                Complex::<p3_mersenne_31::Mersenne31>::zero(),
            );

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { b.as_canonical_u32() }
                { mul_script.clone() }
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_qm31_mul_m31_by_constant() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        let mut total_len = 0;

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<p3_mersenne_31::Mersenne31>();

            let mul_script = qm31_mul_m31_by_constant(b.as_canonical_u32());
            total_len += mul_script.len();

            let c = a * F::new(
                Complex::<p3_mersenne_31::Mersenne31>::new(b, p3_mersenne_31::Mersenne31::zero()),
                Complex::<p3_mersenne_31::Mersenne31>::zero(),
            );

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { mul_script.clone() }
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                qm31_equalverify
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("qm31 mul_by_m31_by_constant: {}", total_len as f64 / 100.0);
    }

    #[test]
    fn test_qm31_copy() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();

        let copy_script = qm31_copy(1);

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { copy_script.clone() }
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            qm31_equalverify
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            qm31_equalverify
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            qm31_equalverify
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_roll() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();

        let roll_script = qm31_roll(1);

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { roll_script.clone() }
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            qm31_equalverify
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            qm31_equalverify
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
