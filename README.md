## rust-bitcoin-m31

This repository implements M31 field arithmetic in Bitcoin Script.

### Performance

For M31, we have:

- addition: 18 weight units
- subtraction: 12 weight units
- multiplication: 1060 weight units
- multiplication by constant: ~750 weight units (M31)

For the complex extension of M31 using x^2 + 1, we have:

- addition: 39 weight units
- subtraction: 28 weight units
- multiplication: 3277 weight units
- multiplication by constant: ~2323 weight units
- multiplication by M31: 2124 weight units
- multiplication by M31 constant: ~1499 weight units

For the degree-4 extension of M31 using y^2 - 2 - i over the complex field x^2 + 1, we have:

- addition: 84 weight units
- subtraction: 63 weight units
- multiplication: 10126 weight units
- multiplication by constant: ~7187 weight units
- multiplication by M31: 4254 weight units
- multiplication by M31 constant: ~3002 weight units

### Credits

Thanks to [Robin Linus](https://robinlinus.com/) for pointing out an optimization that reduces the multiplication from 1767 to 1736 (`1 OP_ROLL` is 
equivalent to `OP_SWAP`). 

Thanks to [Shahar Papini](https://x.com/PapiniShahar) from Starkware for pointing out that double Karatsuba can improve the performance for QM31.

A windowing method is used to reduce the multiplication overhead further, but it was not as powerful as expected.

The introduction of a dual form, `n31`, for which `m31 + n31` are more efficient than `m31 + m31` or `n31 + n31`, brings 
the cost from 1505 to 1415 for BabyBear and from 14404 to 13594 for BabyBear4.

When multiplying a degree-4 element with a degree-1 base element, we reuse the bit decomposition, this avoids the redundancy 
of doing the bit decomposition multiple times, from 5660 to 4702. We note that an alternative route is to produce a 
larger lookup table for the degree-1 base element and share this table between the four subelements in the degree-4 
element. But our attempts show that it is slower than this naive approach (which is expected because the naive method 
already uses a lookup table). 

If one of the multipliers is a constant, we have more efficient multiplication using a relaxed NAF representation, 
which saves from 1415 down to \~744 for M31 on degree-1 element multiplication in this special case. We use "\~" to 
emphasize that this cost is variable and depends on the constant.

Thanks to [Belove Bist](https://x.com/BeloveBist) from Alpen Labs for an improved version of M31 multiplication that reuses
the M31 modulus instead of allocating them repeatedly (since the M31 modulus would take about five bytes). This brings in 
significant improvement in terms of multiplication. The M31 multiplication reduces from 1415 weight units to 1060, CM31 
multiplication reduces from 4351 to 3277, and QM31 multiplication reduces from 13321 to 10126.