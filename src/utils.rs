/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

/// Calculates the binary logarithm (base-2) of a `u8` value.
///
/// This is equivalent to `log2(value)` but works with integer-only operations.
/// Returns the position of the highest set bit.
///
/// # Panics
///
/// This function will panic if `value` is zero, as log2(0) is undefined.
// MSRV TODO: remove this function when MSRV >= 1.67
pub fn u32_ilog2(value: u32) -> u32 {
    // MSRV TODO: use `u32::BITS` when MSRV >= 1.53
    const BITS: u32 = 32;

    BITS - 1 - value.leading_zeros()
}

/// Performs wrapping addition of a `u32` and a signed `i32`.
// MSRV TODO: remove this function when MSRV >= 1.66
pub fn u32_wrapping_add_signed(value: u32, other: i32) -> u32 {
    value.wrapping_add(other as u32)
}

/// Performs wrapping subtraction of an unsigned `u32` from a signed `i32`.
// MSRV TODO: remove this function when MSRV >= 1.66
pub fn i32_wrapping_sub_unsigned(value: i32, other: u32) -> i32 {
    value.wrapping_sub(other as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_ilog2() {
        assert_eq!(u32_ilog2(1), 0); // log2(1) = 0
        assert_eq!(u32_ilog2(2), 1); // log2(2) = 1
        assert_eq!(u32_ilog2(4), 2); // log2(4) = 2
        assert_eq!(u32_ilog2(8), 3); // log2(8) = 3
        assert_eq!(u32_ilog2(255), 7); // log2(255) = 7
    }

    #[test]
    fn test_u32_wrapping_add_signed() {
        assert_eq!(u32_wrapping_add_signed(0x80000000, 0x100), 0x80000100);
        assert_eq!(u32_wrapping_add_signed(0x80000100, -0x100), 0x80000000);
    }

    #[test]
    fn test_i32_wrapping_sub_unsigned() {
        assert_eq!(i32_wrapping_sub_unsigned(0x40000100, 0x100), 0x40000000);
        assert_eq!(i32_wrapping_sub_unsigned(0x00000000, 0x100), -0x100);
        assert_eq!(i32_wrapping_sub_unsigned(-2, 0xFFFFFFFF), -1);
    }
}
