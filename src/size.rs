/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{fmt, ops};

#[cfg(feature = "try_from")]
use core::convert::TryFrom;

use super::{Rom, UserSize, Vram, VramOffset};

/// An unsigned size value.
///
/// This type represents a size or count of bytes. It is always non-negative
/// and wraps on overflow.
///
/// A `Size` can be added to [`Vram`] or [`Rom`] addresses to produce new
/// addresses. Multiple `Size` values can also be added together.
///
/// To get the raw inner value use the [`inner`] method.
///
/// # Examples
///
/// ```
/// use address_space::{Size, Vram, Rom};
///
/// let size = Size::new(0x100);
/// let vram = Vram::new(0x80000000);
/// let rom = Rom::new(0x1000);
///
/// // Adding size to addresses
/// assert_eq!(vram + size, Vram::new(0x80000100));
/// assert_eq!(rom + size, Rom::new(0x1100));
///
/// // Adding sizes together
/// let size2 = Size::new(0x200);
/// assert_eq!(size + size2, Size::new(0x300));
/// ```
///
/// [`inner`]: Size::inner
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size {
    inner: u32,
}

impl Size {
    /// Constructs a `Size` from a given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Size;
    ///
    /// let size = Size::new(0x100);
    /// assert_eq!(size.inner(), 0x100);
    /// ```
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    /// Attempts to convert a [`VramOffset`] to a `Size`.
    ///
    /// Returns `Err` if the offset is negative.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `value` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Size, VramOffset};
    ///
    /// let positive_offset = VramOffset::new(0x100);
    /// assert_eq!(Size::try_from(positive_offset).unwrap(), Size::new(0x100));
    ///
    /// let negative_offset = VramOffset::new(-0x50);
    /// assert!(Size::try_from(negative_offset).is_err());
    /// ```
    pub fn try_from(value: VramOffset) -> Result<Self, ConvertToSizeError> {
        if value.inner() < 0 {
            Err(ConvertToSizeError {
                inner: value.inner(),
            })
        } else {
            Ok(Self::new(value.inner() as u32))
        }
    }

    /// Returns the internal size value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Size;
    ///
    /// let size = Size::new(0x1234);
    /// assert_eq!(size.inner(), 0x1234);
    /// ```
    #[must_use]
    pub const fn inner(&self) -> u32 {
        self.inner
    }

    /// Returns whether this size is zero (empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Size;
    ///
    /// assert!(Size::new(0).is_empty());
    /// assert!(!Size::new(1).is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    /// Adds two sizes together.
    ///
    /// Wraps on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Size;
    ///
    /// let size1 = Size::new(0x100);
    /// let size2 = Size::new(0x200);
    /// assert_eq!(size1.add_size(&size2), Size::new(0x300));
    /// ```
    #[must_use]
    pub fn add_size(&self, rhs: &Self) -> Self {
        Self::new(self.inner().wrapping_add(rhs.inner()))
    }

    /// Adds two sizes together, if successful.
    ///
    /// Returns `None` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Size;
    ///
    /// let size1 = Size::new(0x100);
    /// let size2 = Size::new(0x200);
    /// assert_eq!(size1.add_size_checked(&size2), Some(Size::new(0x300)));
    /// ```
    #[must_use]
    pub fn add_size_checked(&self, rhs: &Self) -> Option<Self> {
        self.inner().checked_add(rhs.inner()).map(Self::new)
    }

    /// Adds two sizes together.
    ///
    /// Wraps on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Size;
    ///
    /// let size1 = Size::new(0x100);
    /// let size2 = Size::new(0x200);
    /// assert_eq!(size1.add_size(&size2), Size::new(0x300));
    /// ```
    #[must_use]
    pub fn add_user_size(&self, rhs: &UserSize) -> Self {
        Self::new(self.inner().wrapping_add(rhs.inner().get()))
    }

    /// Adds two sizes together, if successful.
    ///
    /// Returns `None` on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::Size;
    ///
    /// let size1 = Size::new(0x100);
    /// let size2 = Size::new(0x200);
    /// assert_eq!(size1.add_size_checked(&size2), Some(Size::new(0x300)));
    /// ```
    #[must_use]
    pub fn add_user_size_checked(&self, rhs: &UserSize) -> Option<Self> {
        self.inner().checked_add(rhs.inner().get()).map(Self::new)
    }

    /// Adds this size to a [`Vram`] address, returning a new VRAM address.
    ///
    /// Wraps on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Size, Vram};
    ///
    /// let size = Size::new(0x100);
    /// let vram = Vram::new(0x80000000);
    /// assert_eq!(size.add_vram(&vram), Vram::new(0x80000100));
    /// ```
    ///
    /// ```
    /// use address_space::{Size, Vram};
    ///
    /// let size = Size::new(0x80000100);
    /// let vram = Vram::new(0x80000000);
    /// assert_eq!(size.add_vram(&vram), Vram::new(0x100));
    /// ```
    #[must_use]
    pub fn add_vram(&self, rhs: &Vram) -> Vram {
        Vram::new(self.inner().wrapping_add(rhs.inner()))
    }

    /// Adds this size to a [`Vram`] address, returning a VRAM address if
    /// successful.
    ///
    /// Returns `None` if the addition would overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Size, Vram};
    ///
    /// let size = Size::new(0x100);
    /// let vram = Vram::new(0x80000000);
    /// assert_eq!(size.add_vram_checked(&vram), Some(Vram::new(0x80000100)));
    /// ```
    ///
    /// ```
    /// use address_space::{Size, Vram};
    ///
    /// let size = Size::new(0x80000100);
    /// let vram = Vram::new(0x80000000);
    /// assert_eq!(size.add_vram_checked(&vram), None);
    /// ```
    #[must_use]
    pub fn add_vram_checked(&self, rhs: &Vram) -> Option<Vram> {
        self.inner().checked_add(rhs.inner()).map(Vram::new)
    }

    /// Adds this size to a [`Rom`] address, returning a new ROM address.
    ///
    /// Wraps on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Size, Rom};
    ///
    /// let size = Size::new(0x100);
    /// let rom = Rom::new(0x1000);
    /// assert_eq!(size.add_rom(&rom), Rom::new(0x1100));
    /// ```
    ///
    /// ```
    /// use address_space::{Size, Rom};
    ///
    /// let size = Size::new(0xFFFFFFF0);
    /// let rom = Rom::new(0x1000);
    /// assert_eq!(size.add_rom(&rom), Rom::new(0xFF0));
    /// ```
    #[must_use]
    pub fn add_rom(&self, rhs: &Rom) -> Rom {
        Rom::new(self.inner().wrapping_add(rhs.inner()))
    }

    /// Adds this size to a [`Rom`] address, returning a ROM address if
    /// successful.
    ///
    /// Returns `None` if the addition would overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{Size, Rom};
    ///
    /// let size = Size::new(0x100);
    /// let rom = Rom::new(0x1000);
    /// assert_eq!(size.add_rom_checked(&rom), Some(Rom::new(0x1100)));
    /// ```
    ///
    /// ```
    /// use address_space::{Size, Rom};
    ///
    /// let size = Size::new(0xFFFFFFF0);
    /// let rom = Rom::new(0x1000);
    /// assert_eq!(size.add_rom_checked(&rom), None);
    /// ```
    #[must_use]
    pub fn add_rom_checked(&self, rhs: &Rom) -> Option<Rom> {
        self.inner().checked_add(rhs.inner()).map(Rom::new)
    }
}

impl ops::Add<Self> for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_size(&rhs)
    }
}
impl ops::AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Add<UserSize> for Size {
    type Output = Self;

    fn add(self, rhs: UserSize) -> Self::Output {
        self.add_user_size(&rhs)
    }
}
impl ops::AddAssign<UserSize> for Size {
    fn add_assign(&mut self, rhs: UserSize) {
        *self = *self + rhs;
    }
}

impl ops::Add<Size> for UserSize {
    type Output = Size;

    fn add(self, rhs: Size) -> Self::Output {
        rhs.add_user_size(&self)
    }
}

impl ops::Add<Vram> for Size {
    type Output = Vram;

    fn add(self, rhs: Vram) -> Self::Output {
        self.add_vram(&rhs)
    }
}

impl ops::Add<Size> for Vram {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        rhs.add_vram(&self)
    }
}
impl ops::AddAssign<Size> for Vram {
    fn add_assign(&mut self, rhs: Size) {
        *self = *self + rhs;
    }
}

impl ops::Add<Rom> for Size {
    type Output = Rom;

    fn add(self, rhs: Rom) -> Self::Output {
        self.add_rom(&rhs)
    }
}

impl ops::Add<Size> for Rom {
    type Output = Self;

    fn add(self, rhs: Size) -> Self::Output {
        rhs.add_rom(&self)
    }
}
impl ops::AddAssign<Size> for Rom {
    fn add_assign(&mut self, rhs: Size) {
        *self = *self + rhs;
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size {{ 0x{:02X} }}", self.inner)
    }
}
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.inner)
    }
}

/// Error type for conversion failures from [`VramOffset`] to [`Size`].
///
/// This error is returned when attempting to convert a negative [`VramOffset`]
/// to a `Size`, since sizes must be non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConvertToSizeError {
    inner: i32,
}

impl fmt::Display for ConvertToSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Can't convert negative value {} (-0x{:X}) to `Size`.",
            self.inner, -self.inner
        )
    }
}

#[cfg(feature = "error")]
impl core::error::Error for ConvertToSizeError {}

#[cfg(feature = "try_from")]
impl TryFrom<VramOffset> for Size {
    type Error = ConvertToSizeError;

    fn try_from(value: VramOffset) -> Result<Self, Self::Error> {
        Self::try_from(value)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    #[should_panic]
    fn conversion_error_from_vram_offset() {
        let a = Vram::new(0x80000010);
        let b = Vram::new(0x80000200);
        let diff = a - b;

        Size::try_from(diff).unwrap();
    }
}
