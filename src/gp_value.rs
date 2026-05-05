/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GpValue {
    inner: u32,
}

impl GpValue {
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    #[must_use]
    pub const fn inner(&self) -> u32 {
        self.inner
    }
}

impl fmt::Debug for GpValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GpValue {{ 0x{:08X} }}", self.inner)
    }
}
