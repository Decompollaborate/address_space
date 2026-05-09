/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{fmt, ops};

use super::Vram;

/// Holds an offset (in bytes) or difference between two [`Vram`] addresses.
///
/// Unlike [`Size`], a `VramOffset` can represent negative offsets, making it
/// suitable for computing differences between VRAM addresses or for branches
/// that may go forward or backward.
///
/// This struct can be used to modify a [`Vram`] instance through addition.
/// It can be added to a VRAM address using the [`add_vram`] method or the `+`
/// operator.
///
/// To get the raw inner value use the [`inner`] method.
///
/// # Examples
///
/// ```
/// use address_space::{VramOffset, Vram};
///
/// let offset = VramOffset::new(-0x10);
/// let vram = Vram::new(0x80000100);
///
/// assert_eq!(vram + offset, Vram::new(0x800000F0));
/// ```
///
/// # Determining Branch Direction
///
/// `VramOffset` is useful for working with branch offsets:
///
/// ```
/// use address_space::VramOffset;
///
/// let forward_branch = VramOffset::new(0x1000);
/// let backward_branch = VramOffset::new(-0x500);
///
/// assert!(forward_branch.is_positive());
/// assert!(backward_branch.is_negative());
/// ```
///
/// [`Vram`]: crate::vram::Vram
/// [`Size`]: crate::size::Size
/// [`add_vram`]: VramOffset::add_vram
/// [`inner`]: VramOffset::inner
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VramOffset {
    inner: i32,
}

impl VramOffset {
    /// Constructs a `VramOffset` from a given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::VramOffset;
    ///
    /// let positive = VramOffset::new(0x100);
    /// let negative = VramOffset::new(-0x50);
    /// assert_eq!(positive.inner(), 0x100);
    /// assert_eq!(negative.inner(), -0x50);
    /// ```
    #[must_use]
    pub const fn new(value: i32) -> Self {
        Self { inner: value }
    }

    /// Returns the internal offset value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::VramOffset;
    ///
    /// let offset = VramOffset::new(-0x1000);
    /// assert_eq!(offset.inner(), -0x1000);
    /// ```
    #[must_use]
    pub const fn inner(&self) -> i32 {
        self.inner
    }

    /// Adds this offset to the passed [`Vram`] value, generating a new
    /// [`Vram`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0x8);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert_eq!(offset.add_vram(&vram), Vram::new(0x80000108));
    /// ```
    ///
    /// [`Vram`]: crate::vram::Vram
    #[must_use]
    pub fn add_vram(&self, rhs: &Vram) -> Vram {
        rhs.add_offset(self)
    }

    /// Returns whether this offset is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert!(offset.is_zero());
    /// assert_eq!(offset + vram, vram);
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.inner == 0
    }

    /// Returns whether this offset is positive.
    ///
    /// If this is a branch offset then it can be interpreted as a forward branch.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(0x20);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert!(offset.is_positive());
    /// assert_eq!((offset + vram).inner(), 0x80000120);
    /// ```
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        self.inner > 0
    }

    /// Returns whether this offset is negative.
    ///
    /// If this is a branch offset then it can be interpreted as a backwards branch (i.e. a loop).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{VramOffset, Vram};
    ///
    /// let offset = VramOffset::new(-0x20);
    /// let vram = Vram::new(0x80000100);
    ///
    /// assert!(offset.is_negative());
    /// assert_eq!((offset + vram).inner(), 0x800000E0);
    /// ```
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.inner < 0
    }
}

impl ops::Add<Vram> for VramOffset {
    type Output = Vram;

    fn add(self, rhs: Vram) -> Self::Output {
        self.add_vram(&rhs)
    }
}

impl ops::Add<&Vram> for VramOffset {
    type Output = Vram;

    fn add(self, rhs: &Vram) -> Self::Output {
        self.add_vram(rhs)
    }
}

impl fmt::Debug for VramOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VramOffset {{ ")?;

        // `-2^31` fits on an i32, but `-(-2^31)` doesn't, so we cast to i64 to
        // avoid overflowing.
        let mut inner = self.inner as i64;
        if inner < 0 {
            inner = -inner;
            write!(f, "-")?;
        }
        write!(f, "0x{:X} }}", inner)
    }
}

impl fmt::Display for VramOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.inner)
    }
}
