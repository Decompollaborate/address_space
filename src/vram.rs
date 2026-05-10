/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{fmt, ops};

use super::{utils, Size, VramOffset};

/// A VRAM (Virtual RAM) address.
///
/// This type represents an address within the Virtual RAM address space.
///
/// A `Vram` address can be modified by a [`VramOffset`] instance through
/// addition, generating a new `Vram` value. It is also possible to calculate
/// the difference between two `Vram` addresses, which will return a
/// [`VramOffset`] instance (which may be negative).
///
/// To get the raw inner value use the [`inner`] method.
///
/// # Examples
///
/// ```
/// use address_space::{Vram, VramOffset, Size};
///
/// let vram1 = Vram::new(0x80000000);
/// let vram2 = Vram::new(0x80000100);
/// let offset = VramOffset::new(0x10);
///
/// // Adding an offset to a VRAM address
/// assert_eq!(vram1 + offset, Vram::new(0x80000010));
///
/// // Subtracting two VRAM addresses
/// assert_eq!(vram2 - vram1, VramOffset::new(0x100));
/// ```
///
/// [`VramOffset`]: crate::VramOffset
/// [`inner`]: Vram::inner
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vram {
    inner: u32,
}

impl Vram {
    /// Constructs a `Vram` from a given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Vram;
    ///
    /// let vram = Vram::new(0x80000000);
    /// assert_eq!(vram.inner(), 0x80000000);
    /// ```
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    /// Returns the internal VRAM address value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Vram;
    ///
    /// let vram = Vram::new(0x80000ABC);
    /// assert_eq!(vram.inner(), 0x80000ABC);
    /// ```
    #[must_use]
    pub const fn inner(&self) -> u32 {
        self.inner
    }

    /// Check if the current VRAM is a NULL pointer (zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Vram;
    ///
    /// let vram = Vram::new(0x80000ABC);
    /// assert!(!vram.is_null());
    ///
    /// let vram = Vram::new(0);
    /// assert!(vram.is_null());
    /// ```
    #[must_use]
    pub const fn is_null(&self) -> bool {
        self.inner == 0
    }
}

impl Vram {
    /// Adds a [`Size`] to this VRAM address, generating a new VRAM address.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram = Vram::new(0x80001000);
    /// let size = Size::new(0x100);
    ///
    /// assert_eq!(vram.add_size(&size), Vram::new(0x80001100));
    /// ```
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram = Vram::new(0x80001000);
    /// let size = Size::new(0x88880000);
    ///
    /// assert_eq!(vram.add_size(&size), Vram::new(0x08881000));
    /// ```
    #[must_use]
    pub fn add_size(&self, size: &Size) -> Self {
        size.add_vram(self)
    }

    /// Adds a [`Size`] to this VRAM address, generating new VRAM address if successful.
    ///
    /// Returns `None` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram = Vram::new(0x80001000);
    /// let size = Size::new(0x100);
    ///
    /// assert_eq!(vram.add_size_checked(&size), Some(Vram::new(0x80001100)));
    /// ```
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram = Vram::new(0x80001000);
    /// let size = Size::new(0xFFFFFFF0);
    ///
    /// assert_eq!(vram.add_size_checked(&size), None);
    /// ```
    #[must_use]
    pub fn add_size_checked(&self, size: &Size) -> Option<Self> {
        size.add_vram_checked(self)
    }

    /// Subtracts another VRAM address from this one, wrapping on overflow.
    ///
    /// In other words, performs `self - rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram1 = Vram::new(0x80000100);
    /// let vram2 = Vram::new(0x80000000);
    ///
    /// assert_eq!(vram1.sub_vram(&vram2), Size::new(0x100));
    /// ```
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram1 = Vram::new(0x80000400);
    /// let vram2 = Vram::new(0x80000600);
    ///
    /// assert_eq!(vram1.sub_vram(&vram2), Size::new(0xFFFFFE00));
    /// ```
    #[must_use]
    pub fn sub_vram(&self, rhs: &Self) -> Size {
        Size::new(self.inner.wrapping_sub(rhs.inner))
    }

    /// Subtracts another VRAM address from this one, returning a [`Size`] if successful.
    ///
    /// In other words, performs `self - rhs`.
    ///
    /// Returns `None` if the subtraction would underflow (i.e., if `rhs` > `self`).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram1 = Vram::new(0x80000100);
    /// let vram2 = Vram::new(0x80000000);
    ///
    /// assert_eq!(vram1.sub_vram_checked(&vram2), Some(Size::new(0x100)));
    /// ```
    ///
    /// ```
    /// use address_space::{Vram, Size};
    ///
    /// let vram1 = Vram::new(0x80000400);
    /// let vram2 = Vram::new(0x80000600);
    ///
    /// assert_eq!(vram1.sub_vram_checked(&vram2), None);
    /// ```
    #[must_use]
    pub fn sub_vram_checked(&self, rhs: &Self) -> Option<Size> {
        self.inner.checked_sub(rhs.inner).map(Size::new)
    }

    /// Adds a [`VramOffset`] to this VRAM address, generating a new VRAM value.
    ///
    /// The offset may be negative, effectively subtracting from the address.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0x8);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert_eq!(vram.add_offset(&offset), Vram::new(0x80000108));
    /// ```
    ///
    /// ```
    /// use address_space::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(-0x8);
    /// let vram = Vram::new(0x80000110);
    ///
    /// assert_eq!(vram.add_offset(&offset), Vram::new(0x80000108));
    /// ```
    ///
    /// [`VramOffset`]: crate::vram_offset::VramOffset
    #[must_use]
    pub fn add_offset(&self, rhs: &VramOffset) -> Self {
        let value = utils::u32_wrapping_add_signed(self.inner, rhs.inner());
        Self::new(value)
    }

    /// Subtracts another VRAM address from this one, returning a signed
    /// [`VramOffset`].
    ///
    /// The returned offset can be positive or negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{VramOffset, Vram};
    ///
    /// let vram_a = Vram::new(0x80000100);
    /// let vram_b = Vram::new(0x80000140);
    ///
    /// assert_eq!(vram_a.sub_vram_signed(&vram_b), VramOffset::new(-0x40));
    /// ```
    ///
    /// [`VramOffset`]: crate::vram_offset::VramOffset
    #[must_use]
    pub fn sub_vram_signed(&self, rhs: &Self) -> VramOffset {
        let value = utils::i32_wrapping_sub_unsigned(self.inner as i32, rhs.inner());
        VramOffset::new(value)
    }

    /// Aligns down the VRAM address to the given power-of-two `alignment`.
    ///
    /// If the `alignment` parameter is not a power-of-two then it will be
    /// rounded down to the nearest power-of-two.
    ///
    /// # Panics
    ///
    /// This function will panic if `alignment` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Vram;
    ///
    /// let vram = Vram::new(0x800000A4);
    ///
    /// assert_eq!(vram.align_down(8), Vram::new(0x800000A0));
    /// assert_eq!(vram.align_down(0x80), Vram::new(0x80000080));
    /// assert_eq!(vram.align_down(0x100), Vram::new(0x80000000));
    /// assert_eq!(vram.align_down(0xA), Vram::new(0x800000A0));
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn align_down(&self, alignment: u32) -> Self {
        let shift = utils::u32_ilog2(alignment);

        // Strip the lower bits by shifting.
        Self::new((self.inner >> shift) << shift)
    }
}

impl ops::Add<VramOffset> for Vram {
    type Output = Self;

    fn add(self, rhs: VramOffset) -> Self::Output {
        self.add_offset(&rhs)
    }
}

impl ops::Add<&VramOffset> for Vram {
    type Output = Self;

    fn add(self, rhs: &VramOffset) -> Self::Output {
        self.add_offset(rhs)
    }
}

impl ops::AddAssign<VramOffset> for Vram {
    fn add_assign(&mut self, rhs: VramOffset) {
        *self = self.add_offset(&rhs);
    }
}

impl ops::AddAssign<&VramOffset> for Vram {
    fn add_assign(&mut self, rhs: &VramOffset) {
        *self = self.add_offset(rhs);
    }
}

impl ops::Sub<Self> for Vram {
    type Output = VramOffset;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_vram_signed(&rhs)
    }
}

impl ops::Sub<&Self> for Vram {
    type Output = VramOffset;

    fn sub(self, rhs: &Self) -> Self::Output {
        self.sub_vram_signed(rhs)
    }
}

impl fmt::Debug for Vram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vram {{ 0x{:08X} }}", self.inner)
    }
}

impl fmt::Display for Vram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08X}", self.inner)
    }
}
