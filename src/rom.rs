/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{fmt, ops};

use super::Size;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rom {
    inner: u32,
}

impl Rom {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    #[must_use]
    pub const fn inner(&self) -> u32 {
        self.inner
    }
}

impl Rom {
    #[must_use]
    pub fn add_size(&self, size: &Size) -> Self {
        size.add_rom(self)
    }

    #[must_use]
    pub fn sub_rom(&self, rhs: &Self) -> Option<Size> {
        self.inner.checked_sub(rhs.inner).map(Size::new)
    }
}

impl fmt::Debug for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rom {{ 0x{:08X} }}", self.inner)
    }
}

impl ops::Index<Rom> for [u8] {
    type Output = u8;

    #[inline]
    #[allow(clippy::indexing_slicing)]
    fn index(&self, idx: Rom) -> &Self::Output {
        &self[idx.inner as usize]
    }
}
