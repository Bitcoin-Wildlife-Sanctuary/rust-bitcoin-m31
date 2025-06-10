use crate::treepp::*;

/// Modulus of the M31 field.
pub const MOD: u32 = (1 << 31) - 1;

/// Push a zero M31 element.
///
/// Output: 0
///
pub fn push_m31_zero() -> Script {
    script! {
        0
    }
}

/// Push a one M31 element.
///
/// Output: 1
///
pub fn push_m31_one() -> Script {
    script! {
        1
    }
}

/// Push a zero twisted M31 element.
///
/// Output: 1 - 2^ 31
///
pub fn push_n31_zero() -> Script {
    script! {
        { -(MOD as i64) }
    }
}

/// Push a one twisted M31 element.
///
/// Output: 2 - 2^31
///
pub fn push_n31_one() -> Script {
    script! {
        { 1 - (MOD as i64) }
    }
}

/// Pull an M31 element from the bottom of the stack.
///
/// Hint:
/// - m31
///
/// Output:
/// - m31
///
pub fn m31_from_bottom() -> Script {
    script! {
        OP_DEPTH OP_1SUB OP_ROLL
    }
}

/// Convert an M31 element into a twisted M31 element.
///
/// Input:
/// - m31
///
/// Output:
/// - n31
///
pub fn m31_to_n31() -> Script {
    script! {
        { MOD } OP_SUB
    }
}

/// Convert a twisted M31 element into an M31 element.
///
/// Input:
/// - n31
///
/// Output:
/// - m31
///
pub fn n31_to_m31() -> Script {
    script! {
        { MOD } OP_ADD
    }
}

/// Add a twisted M31 element to an M31 element.
///
/// Input:
/// - m31 representing a
/// - n31 representing b
///
/// Output:
/// - m31 representing a + b
///
pub fn m31_add_n31() -> Script {
    script! {
        OP_ADD
        m31_adjust
    }
}

/// Add an M31 element to a twisted M31 element.
///
/// Input:
/// - n31 representing a
/// - m31 representing b
///
/// Output:
/// - n31 representing a + b
///
pub fn n31_add_m31() -> Script {
    script! {
        OP_ADD
        n31_adjust
    }
}

fn m31_adjust() -> Script {
    script! {
        OP_DUP
        0 OP_LESSTHAN
        OP_IF { MOD } OP_ADD OP_ENDIF
    }
}

fn n31_adjust() -> Script {
    script! {
        OP_DUP
        0 OP_GREATERTHANOREQUAL
        OP_IF { MOD } OP_SUB OP_ENDIF
    }
}

/// Add two M31 elements.
///
/// Input:
/// - m31
/// - m31
///
/// Output:
/// - m31
///
pub fn m31_add() -> Script {
    script! {
        m31_to_n31
        m31_add_n31
    }
}

/// Add two twisted M31 elements.
///
/// Input:
/// - n31
/// - n31
///
/// Output:
/// - n31
///
pub fn n31_add() -> Script {
    script! {
        n31_to_m31
        n31_add_m31
    }
}

/// Double an M31 element.
///
/// Input:
/// - m31
///
/// Output:
/// - m31
///
pub fn m31_double() -> Script {
    script! {
        OP_DUP
        m31_add
    }
}

/// Double a twisted M31 element.
///
/// Input:
/// - n31
///
/// Output:
/// - n31
///
pub fn n31_double() -> Script {
    script! {
        OP_DUP
        n31_add
    }
}

/// Subtract two M31 elements.
///
/// Input:
/// - m31 representing a
/// - m31 representing b
///
/// Output:
/// - m31 representing a - b
///
pub fn m31_sub() -> Script {
    script! {
        OP_SUB
        m31_adjust
    }
}

/// Subtract two twisted M31 elements.
///
/// Input:
/// - n31 representing a
/// - n31 representing b
///
/// Output:
/// - n31 representing a - b
///
pub fn n31_sub() -> Script {
    script! {
        OP_SUB
        n31_adjust
    }
}

/// Negate an M31 element.
///
/// Input:
/// - m31
///
/// Output:
/// - m31
///
pub fn m31_neg() -> Script {
    script! {
        { MOD }
        OP_SWAP
        OP_SUB
        OP_DUP { MOD } OP_EQUAL OP_IF
            OP_DROP 0
        OP_ENDIF
    }
}

/// Negate a twisted M31 element.
///
/// Input:
/// - n31
///
/// Output:
/// - n31
///
pub fn n31_neg() -> Script {
    script! {
        { -(MOD as i64) }
        OP_SWAP
        OP_SUB
        OP_DUP { -(MOD as i64) } OP_EQUAL OP_IF
            OP_DROP 0
        OP_ENDIF
    }
}

/// Convert an M31 element into the bit representation, in an low-endian manner.
///
/// Output:
/// - bits (higher bits first, then lower bits, which are closer to the top of the stack)
///
pub fn m31_to_bits() -> Script {
    script! {
        for i in 0..30 {
            OP_DUP
            { 1 << (30 - i) } OP_GREATERTHANOREQUAL
            OP_SWAP OP_OVER
            OP_IF { 1 << (30 - i) } OP_SUB OP_ENDIF
        }
    }
}

fn get_window(index: u32) -> Script {
    use m31_mul_optimized::*;

    let s = W_WIDTH * (N_WINDOW - index - 1);
    let e = N_BITS.min(s + W_WIDTH);

    script! {
        // in: {m} {g}
        for i in (s..e).rev() {
            if index > 0 { OP_SWAP }        // {g} {acc?} {m}
            { 1 << i }                      // {g} {acc?} {m} {1<<i}
            OP_2DUP OP_GREATERTHANOREQUAL   // {g} {acc?} {m} {1<<i} {bit}
            OP_IF OP_SUB 0 OP_ENDIF OP_NOT  // {g} {acc?} {m-1<<i} {bit}
            if i < e - 1 {
                OP_ROT OP_DUP OP_ADD OP_ADD // {g} {m-1<<i} {2acc?+bit}
            }
        }
    }
}

/// Multiply two M31 elements.
///
/// Input:
/// - m31
/// - m31
///
/// Output:
/// - m31
///
pub fn m31_mul() -> Script {
    use m31_mul_optimized::*;

    let pre_compute_table = script! {
        for i in 2..=W_WIDTH {
            for j in 1 << (i - 1)..1 << i {
                if j % 2 == 0 {
                    { pick(j / 2 - 1) }
                    { n31_double(j) }
                } else {
                    OP_DUP
                    { pick(j - 1) }
                    { n31_add(j + 1) }
                }
            }
        }
        for i in 2..1<<W_WIDTH { {i-1} OP_ROLL }
        0
    };

    script! {
        { MOD } OP_ROT
        { m31_to_n31(1) }
        { pre_compute_table }
        0 // acc
        { 2 + (1 << W_WIDTH) } OP_ROLL
        for i in 0..N_WINDOW {
            { get_window(i) } 2 OP_ADD OP_PICK
            OP_ROT { m31_add_n31(3 + (1 << W_WIDTH)) }
            if i < N_WINDOW - 1 {
                for _ in 0..W_WIDTH { { m31_double(2 + (1 << W_WIDTH)) } }
            }
        }
        OP_TOALTSTACK
        for _ in 0..=(1 << W_WIDTH) / 2 { OP_2DROP }
        OP_FROMALTSTACK
    }
}

/// Multiply an M31 by a constant
///
/// Input:
/// - m31
///
/// Output:
/// - m31
///
pub fn m31_mul_by_constant(constant: u32) -> Script {
    let mut naf = find_naf(constant);

    if naf.len() > 3 {
        let len = naf.len();
        if naf[len - 2] == 0 && naf[len - 3] == -1 {
            naf[len - 3] = 1;
            naf[len - 2] = 1;
            naf.resize(len - 1, 0);
        }
    }

    let mut cur = 0usize;
    let mut script_bytes = vec![];

    let double = m31_double();
    while cur < naf.len() && naf[cur] == 0 {
        script_bytes.extend_from_slice(double.as_bytes());
        cur += 1;
    }

    if cur < naf.len() {
        if naf[cur] == 1 {
            script_bytes.extend_from_slice(&[0x76]); // OP_DUP
            script_bytes.extend_from_slice(double.as_bytes());
            cur += 1;
        } else if naf[cur] == -1 {
            script_bytes.extend_from_slice(
                script! {
                    OP_DUP m31_neg OP_SWAP
                }
                .as_bytes(),
            );
            script_bytes.extend_from_slice(double.as_bytes());
            cur += 1;
        } else {
            unreachable!()
        }
    } else {
        script_bytes.extend_from_slice(
            script! {
                OP_DROP { 0 }
            }
            .as_bytes(),
        );

        return Script::from(script_bytes);
    }

    if cur < naf.len() {
        while cur < naf.len() {
            if naf[cur] == 0 {
                script_bytes.extend_from_slice(double.as_bytes());
            } else if naf[cur] == 1 {
                script_bytes.extend_from_slice(
                    script! {
                        OP_SWAP OP_OVER m31_add OP_SWAP
                    }
                    .as_bytes(),
                );
                if cur != naf.len() - 1 {
                    script_bytes.extend_from_slice(double.as_bytes());
                }
            } else if naf[cur] == -1 {
                script_bytes.extend_from_slice(
                    script! {
                        OP_SWAP OP_OVER m31_sub OP_SWAP
                    }
                    .as_bytes(),
                );
                if cur != naf.len() - 1 {
                    script_bytes.extend_from_slice(double.as_bytes());
                }
            }
            cur += 1;
        }
    }

    script_bytes.extend_from_slice(&[0x75]); // OP_DROP
    Script::from(script_bytes)
}

/// Compute the NAF (non-adjacent form) of num
/// Adapted from https://github.com/arkworks-rs/algebra/blob/master/ff/src/biginteger/arithmetic.rs
pub fn find_naf(mut num: u32) -> Vec<i8> {
    let mut res = vec![];

    while num != 0 {
        let z: i8;
        if num % 2 == 1 {
            z = 2 - (num % 4) as i8;
            if z >= 0 {
                num -= z as u32;
            } else {
                num += (-z) as u32;
            }
        } else {
            z = 0;
        }
        res.push(z);
        num >>= 1;
    }

    res
}

/// Square an M31 element.
///
/// Input:
/// - m31
///
/// Output:
/// - m31
///
pub fn m31_square() -> Script {
    script! {
        OP_DUP
        m31_mul
    }
}

#[cfg(test)]
mod test {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    #[test]
    fn test_m31_add() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("m31 add: {}", m31_add().len());

        for _ in 0..100 {
            let a: u32 = prng.random();
            let b: u32 = prng.random();

            let a_m31 = a % MOD;
            let b_m31 = b % MOD;
            let sum_m31 = (a_m31 + b_m31) % MOD;

            let script = script! {
                { a_m31 }
                { b_m31 }
                m31_add
                { sum_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_m31_sub() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("m31 sub: {}", m31_sub().len());

        for _ in 0..100 {
            let a: u32 = prng.random();
            let b: u32 = prng.random();

            let a_m31 = a % MOD;
            let b_m31 = b % MOD;
            let diff_m31 = (MOD + a_m31 - b_m31) % MOD;

            let script = script! {
                { a_m31 }
                { b_m31 }
                m31_sub
                { diff_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_m31_to_bits() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a: u32 = prng.random();
            let m31 = a % MOD;

            let mut bits = vec![];
            let mut cur = m31;
            for _ in 0..31 {
                bits.push(cur % 2);
                cur >>= 1;
            }
            assert_eq!(cur, 0);

            let script = script! {
                { m31 }
                m31_to_bits
                for i in 0..30 {
                    { bits[i as usize] } OP_EQUALVERIFY
                }
                { bits[30] } OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_m31_square() {
        let mut prng = ChaCha20Rng::seed_from_u64(6u64);
        eprintln!("m31 square: {}", m31_square().len());

        for _ in 0..100 {
            let a: u32 = prng.random();

            let a_m31 = a % MOD;
            let prod_m31 = ((((a_m31 as u64) * (a_m31 as u64)) % (MOD as u64)) & 0xffffffff) as u32;

            let script = script! {
                { a_m31 }
                m31_square
                { prod_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_m31_mul() {
        let mut prng = ChaCha20Rng::seed_from_u64(6u64);
        eprintln!("m31 mul: {}", m31_mul().len());

        for _ in 0..100 {
            let a: u32 = prng.random();
            let b: u32 = prng.random();

            let a_m31 = a % MOD;
            let b_m31 = b % MOD;
            let prod_m31 = ((((a_m31 as u64) * (b_m31 as u64)) % (MOD as u64)) & 0xffffffff) as u32;

            let script = script! {
                { a_m31 }
                { b_m31 }
                m31_mul
                { prod_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_m31_mul_by_constant() {
        let mut prng = ChaCha20Rng::seed_from_u64(6u64);

        let mut total_len = 0;
        for _ in 0..100 {
            let a: u32 = prng.random();
            let b: u32 = prng.random();

            let a_m31 = a % MOD;
            let b_m31 = b % MOD;

            let mul_script = m31_mul_by_constant(b_m31);
            total_len += mul_script.len();

            let prod_m31 = ((((a_m31 as u64) * (b_m31 as u64)) % (MOD as u64)) & 0xffffffff) as u32;

            let script = script! {
                { a_m31 }
                { mul_script.clone() }
                { prod_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("m31 mul_by_constant: {}", total_len as f64 / 100.0);
    }

    #[test]
    fn test_m31_neg() {
        let mut prng = ChaCha20Rng::seed_from_u64(6u64);
        eprintln!("m31 neg: {}", m31_neg().len());

        for _ in 0..100 {
            let a: u32 = prng.random();

            let a_m31 = a % MOD;
            let b_m31 = MOD - a_m31;

            let script = script! {
                { a_m31 }
                m31_neg
                { b_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        let script = script! {
            { 0 }
            m31_neg
            { 0 }
            OP_EQUAL
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}

mod m31_mul_optimized {
    use super::*;

    pub const W_WIDTH: u32 = 2;
    pub const N_BITS: u32 = u32::BITS - MOD.leading_zeros();
    pub const N_WINDOW: u32 = (N_BITS + W_WIDTH - 1) / W_WIDTH;

    /// Copy item at given depth to the top
    pub fn pick(depth: u32) -> Script {
        match depth {
            0 => script! { OP_DUP },
            1 => script! { OP_OVER },
            _ => script! { {depth} OP_PICK },
        }
    }

    pub fn m31_to_n31(mod_depth: u32) -> Script {
        script! {
            if mod_depth > 0 {
                { pick(mod_depth) }
            } else { { MOD } }
            OP_SUB
        }
    }

    fn n31_to_m31(mod_depth: u32) -> Script {
        script! {
            if mod_depth > 0 {
                { pick(mod_depth) }
            } else { { MOD } }
            OP_ADD
        }
    }

    pub fn n31_double(mod_depth: u32) -> Script {
        script! {
            OP_DUP
            { n31_add(if mod_depth == 0 { 0 } else { mod_depth + 1 }) }
        }
    }

    pub fn n31_add(mod_depth: u32) -> Script {
        script! {
            { n31_to_m31(mod_depth) }
            { n31_add_m31(mod_depth) }
        }
    }

    fn n31_add_m31(mut mod_depth: u32) -> Script {
        if mod_depth > 0 {
            mod_depth -= 1;
        }
        script! {
            OP_ADD { n31_adjust(mod_depth) }
        }
    }

    fn n31_adjust(mod_depth: u32) -> Script {
        script! {
            OP_DUP
            0 OP_GREATERTHANOREQUAL
            OP_IF
                if mod_depth > 0 {
                    { pick(mod_depth) }
                } else { { MOD } }
                OP_SUB
            OP_ENDIF
        }
    }

    pub fn m31_add_n31(mut mod_depth: u32) -> Script {
        if mod_depth > 0 {
            mod_depth -= 1;
        }
        script! {
            OP_ADD { m31_adjust(mod_depth) }
        }
    }

    fn m31_adjust(mod_depth: u32) -> Script {
        script! {
            OP_DUP
            0 OP_LESSTHAN
            OP_IF
                if mod_depth > 0 {
                    { pick(mod_depth) }
                } else { { MOD } }
                OP_ADD
            OP_ENDIF
        }
    }

    pub fn m31_double(mut mod_depth: u32) -> Script {
        if mod_depth > 0 {
            mod_depth += 1
        }
        script! {
            OP_DUP { m31_add(mod_depth) }
        }
    }

    fn m31_add(mod_depth: u32) -> Script {
        script! {
            { m31_to_n31(mod_depth) }
            { m31_add_n31(mod_depth) }
        }
    }
}
