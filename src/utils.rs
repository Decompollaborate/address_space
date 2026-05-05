/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

pub fn u8_ilog2(value: u8) -> u32 {
    u8::BITS - 1 - value.leading_zeros()
}
