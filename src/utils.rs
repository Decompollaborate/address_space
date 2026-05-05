/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

pub const fn u8_ilog2(value: u8) -> u32 {
    const BITS: u32 = 8;

    BITS - 1 - value.leading_zeros()
}

pub const fn u32_wrapping_add_signed(value: u32, other: i32) -> u32 {
    value.wrapping_add(other as u32)
}

pub const fn i32_wrapping_sub_unsigned(value: i32, other: u32) -> i32 {
    value.wrapping_sub(other as i32)
}
