/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{fmt, num::NonZeroU32};

use super::{Rom, Size, Vram};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserSize {
    inner: NonZeroU32,
}

impl UserSize {
    #[must_use]
    pub const fn new(value: NonZeroU32) -> Self {
        Self { inner: value }
    }

    #[must_use]
    pub fn new_checked(value: u32) -> Option<Self> {
        Self::new_option(NonZeroU32::new(value))
    }

    #[must_use]
    pub fn new_option(value: Option<NonZeroU32>) -> Option<Self> {
        value.map(Self::new)
    }

    #[must_use]
    pub const fn inner(&self) -> NonZeroU32 {
        self.inner
    }

    #[must_use]
    pub fn add_user_size(&self, rhs: &Self) -> Option<Self> {
        let slf = self.inner().get();
        let temp = slf.checked_add(rhs.inner().get())?;

        Self::new_option(NonZeroU32::new(temp))
    }

    #[must_use]
    pub fn add_size(&self, rhs: &Size) -> Option<Self> {
        let slf = self.inner().get();
        let temp = slf.checked_add(rhs.inner())?;

        Self::new_option(NonZeroU32::new(temp))
    }

    #[must_use]
    pub fn add_vram(&self, rhs: &Vram) -> Option<Vram> {
        let slf = self.inner().get();
        let temp = slf.checked_add(rhs.inner())?;

        Some(Vram::new(temp))
    }

    #[must_use]
    pub fn add_rom(&self, rhs: &Rom) -> Option<Rom> {
        let slf = self.inner().get();
        let temp = slf.checked_add(rhs.inner())?;

        Some(Rom::new(temp))
    }
}

impl fmt::Debug for UserSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UserSize {{ 0x{:02X} }}", self.inner)
    }
}
impl fmt::Display for UserSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.inner)
    }
}

impl From<UserSize> for Size {
    fn from(value: UserSize) -> Self {
        Self::new(value.inner.get())
    }
}

impl From<Option<UserSize>> for Size {
    fn from(value: Option<UserSize>) -> Self {
        let val = match value {
            Some(x) => x.inner().get(),
            None => 0,
        };
        Self::new(val)
    }
}
