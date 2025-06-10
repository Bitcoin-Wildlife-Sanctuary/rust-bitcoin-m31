use crate::treepp::*;
use crate::{
    cm31_dup, cm31_fromaltstack, cm31_mul, cm31_neg, cm31_rot, cm31_toaltstack, m31_from_bottom,
    m31_mul, m31_mul_by_constant, m31_neg,
};

pub use crate::karatsuba::*;

use crate::m31::{m31_add, m31_double, m31_sub};

/// Push a one QM31.
///
/// Output:
/// - qm31 representing 0: 0, 0, 0, 0
///
pub fn push_qm31_zero() -> Script {
    script! {
        { 0 }
        { 0 }
        { 0 }
        { 0 }
    }
}

/// Push a zero QM31.
///
/// Output:
/// - qm31 representing 1: 0, 0, 0, 1
///
pub fn push_qm31_one() -> Script {
    script! {
        { 0 }
        { 0 }
        { 0 }
        { 1 }
    }
}

/// Pull a QM31 element from the bottom of the stack.
///
/// Hint:
/// - qm31
///
/// Output:
/// - qm31
///
pub fn qm31_from_bottom() -> Script {
    script! {
        m31_from_bottom
        m31_from_bottom
        m31_from_bottom
        m31_from_bottom
    }
}

/// Add two QM31 elements.
///
/// Input:
/// - qm31
/// - qm31
///
/// Output:
/// - qm31
///
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

/// Add an M31 element to a QM31 element.
///
/// Input:
/// - qm31
/// - m31
///
/// Output:
/// - qm31
///
pub fn qm31_add_m31() -> Script {
    script! {
        m31_add
    }
}

/// Require two QM31 elements to be equal.
///
/// Input:
/// - qm31
/// - qm31
///
/// Fail the execution if they are not equal.
///
pub fn qm31_equalverify() -> Script {
    script! {
        for i in 0..3 {
            { 4 - i } OP_ROLL
            OP_EQUALVERIFY
        }
        OP_EQUALVERIFY
    }
}

/// Subtract a QM31 element from the other.
///
/// Input:
/// - qm31 representing A
/// - qm31 representing B
///
/// Output:
/// - qm31 representing A - B
///
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

/// Negate a QM31 element.
///
/// Input:
/// - qm31
///
/// Output:
/// - qm31
///
pub fn qm31_neg() -> Script {
    script! {
        m31_neg OP_TOALTSTACK
        m31_neg OP_TOALTSTACK
        m31_neg OP_TOALTSTACK
        m31_neg
        OP_FROMALTSTACK OP_FROMALTSTACK OP_FROMALTSTACK
    }
}

/// Double a QM31 element.
///
/// Input:
/// - qm31
///
/// Output:
/// - qm31
///
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

/// Square a QM31 element.
///
/// Input:
/// - qm31
///
/// Output:
/// - qm31
///
pub fn qm31_square() -> Script {
    script! {
        { qm31_copy(0) }
        qm31_mul
    }
}

/// Multiply two QM31 elements.
///
/// Input:
/// - qm31
/// - qm31
///
/// Output:
/// - qm31
///
pub fn qm31_mul() -> Script {
    script! {
        karatsuba_big
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

/// Multiply a QM31 element with a constant QM31 element.
///
/// Input:
/// - qm31
///
/// Output:
/// - qm31
///
pub fn qm31_mul_by_constant(a2: u32, b2: u32, c2: u32, d2: u32) -> Script {
    script! {
        { karatsuba_big_constant(a2, b2, c2, d2) }
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

/// Multiply a QM31 element by an M31 element.
///
/// Input:
/// - qm31: d, c, b, a for (a + bi) + j(c + di)
/// - m31
///
/// Output:
/// - qm31
///
pub fn qm31_mul_m31() -> Script {
    script! {
        OP_DUP
        5 OP_ROLL
        m31_mul

        OP_SWAP
        OP_DUP
        5 OP_ROLL
        m31_mul

        OP_SWAP
        OP_DUP
        5 OP_ROLL
        m31_mul

        OP_SWAP
        4 OP_ROLL
        m31_mul
    }
}

/// Multiply a QM31 element by an CM31 element
///
/// Input:
/// - qm31: d, c, b, a for (a + bi) + j(c + di)
/// - cm31: v, u
///
/// Output:
/// - qm31
pub fn qm31_mul_cm31() -> Script {
    script! {
        cm31_dup
        cm31_rot cm31_mul cm31_toaltstack
        cm31_mul cm31_fromaltstack
    }
}

/// Multiply a QM31 element by a constant M31 element.
///
/// Input:
/// - qm31: d, c, b, a for (a + bi) + j(c + di)
///
/// Output:
/// - qm31
///
pub fn qm31_mul_m31_by_constant(constant: u32) -> Script {
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

/// Retrieve a QM31 element from the altstack.
pub fn qm31_toaltstack() -> Script {
    script! {
        for _ in 0..4 {
            OP_TOALTSTACK
        }
    }
}

/// Send a QM31 element to the altstack.
pub fn qm31_fromaltstack() -> Script {
    script! {
        for _ in 0..4 {
            OP_FROMALTSTACK
        }
    }
}

/// Copy a QM31 element from the stack.
pub fn qm31_copy(offset: usize) -> Script {
    let a = offset * 4 + 4 - 1;

    script! {
        for _ in 0..4 {
            { a } OP_PICK
        }
    }
}

/// Roll a QM31 element in the stack.
pub fn qm31_roll(offset: usize) -> Script {
    let a = offset * 4 + 4 - 1;

    script! {
        for _ in 0..4 {
            { a } OP_ROLL
        }
    }
}

/// Duplicate the top QM31 element.
pub fn qm31_dup() -> Script {
    qm31_copy(0)
}

/// Swap the top two QM31 elements.
pub fn qm31_swap() -> Script {
    qm31_roll(1)
}

/// Copy the second-to-top QM31 element.
pub fn qm31_over() -> Script {
    qm31_copy(1)
}

/// Rotate QM31 elements.
pub fn qm31_rot() -> Script {
    qm31_roll(2)
}

/// Drop a QM31 element.
pub fn qm31_drop() -> Script {
    script! {
        OP_2DROP OP_2DROP
    }
}

/// Shift a QM31 element by i.
///
/// Input:
/// - a, b, c, d, which represents a qm31 element: (ai + b)j + (ci + d)
///
/// Output:
/// - (-a + bi)j + (-c + di)
///
/// aka:
/// - a' = b
/// - b' = -a
/// - c' = d
/// - d' = -c
///
pub fn qm31_shift_by_i() -> Script {
    script! {
        OP_SWAP
        m31_neg
        OP_2SWAP
        OP_SWAP
        m31_neg
        OP_2SWAP
    }
}

/// Shift a QM31 element by j.
///
/// Input:
/// - a, b, c, d, which represents a qm31 element: (ai + b)j + (ci + d)
///
/// Output:
/// - (ai + b) j^2 + (ci + d)j
/// = (ai + b) (2 + i) + (ci + d) j
/// = (ci + d)j + ((2a + b)i + (2b - a))
///
/// aka:
/// - a' = c
/// - b' = d
/// - c' = 2a + b
/// - d' = 2b - a
///
pub fn qm31_shift_by_j() -> Script {
    script! {
        OP_2SWAP

        // stack: c, d, a, b
        OP_OVER
        m31_double
        OP_OVER
        m31_add

        // stack: c, d, a, b, 2a+b

        OP_SWAP
        m31_double
        OP_ROT
        m31_sub
    }
}

/// Shift a QM31 element by ij.
///
/// Input:
/// - a, b, c, d, which represents a qm31 element: (ai + b)j + (ci + d)
///
/// Output:
/// - (di - c)j + ((2b - a)i - (2a + b))
///
pub fn qm31_shift_by_ij() -> Script {
    script! {
        qm31_shift_by_i
        qm31_shift_by_j
    }
}

/// Complex conjugation of a QM31 element.
///
/// Input:
/// - qm31: d, c, b, a
///
/// Output:
/// - qm31: -d, -c, b, a
pub fn qm31_complex_conjugate() -> Script {
    script! {
        OP_2SWAP
        cm31_neg
        OP_2SWAP
    }
}

#[cfg(test)]
mod test {
    use crate::{
        qm31_add, qm31_copy, qm31_equalverify, qm31_mul, qm31_mul_m31, qm31_mul_m31_by_constant,
        qm31_roll, qm31_sub,
    };
    use core::ops::{Add, Mul, Neg};
    use p3_field::extension::Complex;
    use p3_field::{BasedVectorSpace, PrimeCharacteristicRing, PrimeField32};
    use p3_mersenne_31::Mersenne31;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    type F = p3_field::extension::BinomialExtensionField<Complex<Mersenne31>, 2>;

    #[test]
    fn test_qm31_add() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 add: {}", qm31_add().len());

        for _ in 0..100 {
            let a = rng.random::<F>();
            let b = rng.random::<F>();

            let c = a.add(b);

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let b: &[Complex<Mersenne31>] = b.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
            let a = rng.random::<F>();
            let c = a.double();

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
            let a = rng.random::<F>();
            let b = rng.random::<F>();
            let c = a.add(b.neg());

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let b: &[Complex<Mersenne31>] = b.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
            let a = rng.random::<F>();
            let c = a.neg();

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
            let a = rng.random::<F>();
            let c = a.square();

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
            let a = rng.random::<F>();
            let b = rng.random::<F>();
            let c = a.mul(b);

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let b: &[Complex<Mersenne31>] = b.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
            let a = rng.random::<F>();
            let b = rng.random::<F>();
            let c = a.mul(b);

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let b: &[Complex<Mersenne31>] = b.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
            let a = rng.random::<F>();
            let b = rng.random::<Mersenne31>();

            let c = a * F::new_complex(
                Complex::<Mersenne31>::new_complex(b, Mersenne31::default()),
                Complex::<Mersenne31>::default(),
            );

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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
    fn test_qm31_mul_cm31() {
        let mul_script = qm31_mul_cm31();

        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 mul_by_cm31: {}", mul_script.len());

        for _ in 0..100 {
            let a = rng.random::<F>();
            let b = rng.random::<Complex<Mersenne31>>();

            let c = a * b;

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { b.imag().as_canonical_u32() }
                { b.real().as_canonical_u32() }
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
            let a = rng.random::<F>();
            let b = rng.random::<Mersenne31>();

            let mul_script = qm31_mul_m31_by_constant(b.as_canonical_u32());
            total_len += mul_script.len();

            let c = a * F::new_complex(
                Complex::<Mersenne31>::new_complex(b, Mersenne31::default()),
                Complex::<Mersenne31>::default(),
            );

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

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

        let a = rng.random::<F>();
        let b = rng.random::<F>();

        let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
        let b: &[Complex<Mersenne31>] = b.as_basis_coefficients_slice();

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
    fn test_qm31_roll() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.random::<F>();
        let b = rng.random::<F>();

        let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
        let b: &[Complex<Mersenne31>] = b.as_basis_coefficients_slice();

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

    #[test]
    fn test_qm31_shift_by_i() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let shift_script = qm31_shift_by_i();
        eprintln!("qm31 shift_by_i: {}", shift_script.len());

        for _ in 0..100 {
            let a = rng.random::<F>();
            let c = a.mul(F::new_complex(
                Complex::<Mersenne31>::new_complex(
                    Mersenne31::default(),
                    Mersenne31::new_checked(1).unwrap(),
                ),
                Complex::<Mersenne31>::default(),
            ));

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { shift_script.clone() }
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
    fn test_qm31_shift_by_j() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let shift_script = qm31_shift_by_j();
        eprintln!("qm31 shift_by_j: {}", shift_script.len());

        for _ in 0..100 {
            let a = rng.random::<F>();
            let c = a.mul(F::new_complex(
                Complex::<Mersenne31>::default(),
                Complex::<Mersenne31>::new_complex(
                    Mersenne31::new_checked(1).unwrap(),
                    Mersenne31::default(),
                ),
            ));

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { shift_script.clone() }
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
    fn test_qm31_shift_by_ij() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let shift_script = qm31_shift_by_ij();
        eprintln!("qm31 shift_by_ij: {}", shift_script.len());

        for _ in 0..100 {
            let a = rng.random::<F>();
            let c = a.mul(F::new_complex(
                Complex::<Mersenne31>::default(),
                Complex::<Mersenne31>::new_complex(
                    Mersenne31::default(),
                    Mersenne31::new_checked(1).unwrap(),
                ),
            ));

            let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
            let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { shift_script.clone() }
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
    fn test_qm31_complex_conjugate() {
        let mut prng = ChaCha20Rng::seed_from_u64(0);

        let complex_conjugate_script = qm31_complex_conjugate();
        eprintln!("qm31 complex_conjugate: {}", complex_conjugate_script.len());

        let a = prng.random::<F>();
        let c = a.conjugate();

        let a: &[Complex<Mersenne31>] = a.as_basis_coefficients_slice();
        let c: &[Complex<Mersenne31>] = c.as_basis_coefficients_slice();

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { complex_conjugate_script.clone() }
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
