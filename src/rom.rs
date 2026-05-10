/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{fmt, ops};

use super::Size;

/// A ROM (Read-Only Memory) address.
///
/// This type represents an address within the ROM file/image. ROM addresses are
/// used to locate data and code within the cartridge ROM.
///
/// A `Rom` address can be modified by a [`Size`] instance through addition,
/// generating a new ROM address. It is also possible to calculate the difference
/// between two ROM addresses, which will return a [`Size`] instance.
///
/// To get the raw inner value use the [`inner`] method.
///
/// # Examples
///
/// ```
/// use address_space::{Rom, Size};
///
/// let rom1 = Rom::new(0x1000);
/// let rom2 = Rom::new(0x1010);
/// let size = Size::new(0x100);
///
/// // Adding a size to a ROM address
/// assert_eq!(rom1.add_size(&size), Rom::new(0x1100));
///
/// // Subtracting two ROM addresses
/// assert_eq!(rom2.sub_rom(&rom1), Size::new(0x10));
/// ```
///
/// [`inner`]: Rom::inner
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rom {
    inner: u32,
}

impl Rom {
    /// Constructs a `Rom` from a given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Rom;
    ///
    /// let rom = Rom::new(0x1000);
    /// assert_eq!(rom.inner(), 0x1000);
    /// ```
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    /// Returns the internal ROM address value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Rom;
    ///
    /// let rom = Rom::new(0x1234);
    /// assert_eq!(rom.inner(), 0x1234);
    /// ```
    #[must_use]
    pub const fn inner(&self) -> u32 {
        self.inner
    }
}

impl Rom {
    /// Adds a [`Size`] to this ROM address, generating a new ROM address.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Rom, Size};
    ///
    /// let rom = Rom::new(0x1000);
    /// let size = Size::new(0x100);
    ///
    /// assert_eq!(rom.add_size(&size), Rom::new(0x1100));
    /// ```
    ///
    /// ```
    /// use address_space::{Rom, Size};
    ///
    /// let rom = Rom::new(0x1000);
    /// let size = Size::new(0xFFFFFFF0);
    ///
    /// assert_eq!(rom.add_size(&size), Rom::new(0xFF0));
    /// ```
    #[must_use]
    pub fn add_size(&self, size: &Size) -> Self {
        size.add_rom(self)
    }

    /// Adds a [`Size`] to this ROM address, generating new ROM address if successful.
    ///
    /// Returns `None` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Rom, Size};
    ///
    /// let rom = Rom::new(0x1000);
    /// let size = Size::new(0x100);
    ///
    /// assert_eq!(rom.add_size_checked(&size), Some(Rom::new(0x1100)));
    /// ```
    ///
    /// ```
    /// use address_space::{Rom, Size};
    ///
    /// let rom = Rom::new(0x1000);
    /// let size = Size::new(0xFFFFFFF0);
    ///
    /// assert_eq!(rom.add_size_checked(&size), None);
    /// ```
    #[must_use]
    pub fn add_size_checked(&self, size: &Size) -> Option<Self> {
        size.add_rom_checked(self)
    }

    /// Subtracts another ROM address from this one, wrapping on overflow.
    ///
    /// In other words, performs `self - rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Rom, Size};
    ///
    /// let rom1 = Rom::new(0x1000);
    /// let rom2 = Rom::new(0x1050);
    ///
    /// assert_eq!(rom2.sub_rom(&rom1), Size::new(0x50));
    /// assert_eq!(rom1.sub_rom(&rom2), Size::new(0xFFFFFFB0));
    /// ```
    #[must_use]
    pub fn sub_rom(&self, rhs: &Self) -> Size {
        Size::new(self.inner.wrapping_sub(rhs.inner))
    }

    /// Subtracts another ROM address from this one, returning a [`Size`] if successful.
    ///
    /// In other words, performs `self - rhs`.
    ///
    /// Returns `None` if the subtraction would underflow (i.e., if `rhs` > `self`).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Rom, Size};
    ///
    /// let rom1 = Rom::new(0x1000);
    /// let rom2 = Rom::new(0x1050);
    ///
    /// assert_eq!(rom2.sub_rom_checked(&rom1), Some(Size::new(0x50)));
    /// assert_eq!(rom1.sub_rom_checked(&rom2), None);
    /// ```
    #[must_use]
    pub fn sub_rom_checked(&self, rhs: &Self) -> Option<Size> {
        self.inner.checked_sub(rhs.inner).map(Size::new)
    }
}

impl fmt::Debug for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rom {{ 0x{:08X} }}", self.inner)
    }
}

impl fmt::Display for Rom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08X}", self.inner)
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
