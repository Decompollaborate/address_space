/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{fmt, num::NonZeroU32, ops};

use super::{Rom, Size, Vram};

/// A non-zero size value.
///
/// Unlike [`Size`], a `UserSize` is guaranteed to be non-zero, as it uses
/// [`NonZeroU32`] internally.
///
/// A `UserSize` can be added to other `UserSize` values, [`Size`] values,
/// or to [`Vram`] and [`Rom`] addresses.
///
/// It can also be converted from/to a regular [`Size`].
///
/// # Examples
///
/// ```
/// use address_space::{UserSize, Size, Vram};
/// use core::num::NonZeroU32;
///
/// let user_size = UserSize::new(NonZeroU32::new(0x100).unwrap());
/// let vram = Vram::new(0x80000000);
///
/// // Adding UserSize to VRAM
/// assert_eq!(user_size.add_vram(&vram), Vram::new(0x80000100));
///
/// // Converting to Size
/// let size: Size = user_size.into();
/// assert_eq!(size.inner(), 0x100);
/// ```
///
/// [`Size`]: crate::size::Size
/// [`Vram`]: crate::vram::Vram
/// [`Rom`]: crate::rom::Rom
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserSize {
    inner: NonZeroU32,
}

impl UserSize {
    /// Constructs a `UserSize` from a non-zero value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::UserSize;
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x100).unwrap());
    /// assert_eq!(user_size.inner().get(), 0x100);
    /// ```
    #[must_use]
    pub const fn new(value: NonZeroU32) -> Self {
        Self { inner: value }
    }

    /// Attempts to construct a `UserSize` from a regular `u32` value.
    ///
    /// Returns `None` if the value is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::UserSize;
    ///
    /// assert!(UserSize::new_checked(0x100).is_some());
    /// assert!(UserSize::new_checked(0).is_none());
    /// ```
    #[must_use]
    pub fn new_checked(value: u32) -> Option<Self> {
        Self::new_option(NonZeroU32::new(value))
    }

    /// Constructs a `UserSize` from an `Option<NonZeroU32>`.
    ///
    /// Returns `None` if the option is `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::UserSize;
    /// use core::num::NonZeroU32;
    ///
    /// let opt = NonZeroU32::new(0x200);
    /// assert!(UserSize::new_option(opt).is_some());
    ///
    /// assert!(UserSize::new_option(None).is_none());
    /// ```
    #[must_use]
    pub fn new_option(value: Option<NonZeroU32>) -> Option<Self> {
        value.map(Self::new)
    }

    /// Returns the internal non-zero value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::UserSize;
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x150).unwrap());
    /// assert_eq!(user_size.inner().get(), 0x150);
    /// ```
    #[must_use]
    pub const fn inner(&self) -> NonZeroU32 {
        self.inner
    }
}

impl UserSize {
    /// Adds two `UserSize` values together, if successful.
    ///
    /// Returns `None` if the addition overflows.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::UserSize;
    /// use core::num::NonZeroU32;
    ///
    /// let size1 = UserSize::new(NonZeroU32::new(0x100).unwrap());
    /// let size2 = UserSize::new(NonZeroU32::new(0x200).unwrap());
    ///
    /// assert_eq!(
    ///     size1.add_user_size_checked(&size2).unwrap().inner().get(),
    ///     0x300
    /// );
    /// ```
    #[must_use]
    pub fn add_user_size_checked(&self, rhs: &Self) -> Option<Self> {
        let slf = self.inner().get();
        let temp = slf.checked_add(rhs.inner().get())?;

        Self::new_option(NonZeroU32::new(temp))
    }

    /// Adds a [`Size`] to this `UserSize`.
    ///
    /// Returns `None` if the addition overflows or results in zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{UserSize, Size};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x100).unwrap());
    /// let size = Size::new(0x50);
    ///
    /// assert_eq!(
    ///     user_size.add_size_checked(&size).unwrap().inner().get(),
    ///     0x150
    /// );
    /// ```
    #[must_use]
    pub fn add_size_checked(&self, rhs: &Size) -> Option<Self> {
        let slf = self.inner().get();
        let temp = slf.checked_add(rhs.inner())?;

        Self::new_option(NonZeroU32::new(temp))
    }

    /// Adds this `UserSize` to a [`Vram`] address, returning a new VRAM address.
    ///
    /// Wraps on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{UserSize, Vram};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x100).unwrap());
    /// let vram = Vram::new(0x80000000);
    ///
    /// assert_eq!(user_size.add_vram(&vram), Vram::new(0x80000100));
    /// ```
    ///
    /// ```
    /// use address_space::{UserSize, Vram};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x80000100).unwrap());
    /// let vram = Vram::new(0x80000000);
    ///
    /// assert_eq!(user_size.add_vram(&vram), Vram::new(0x100));
    /// ```
    ///
    /// [`Vram`]: crate::vram::Vram
    #[must_use]
    pub fn add_vram(&self, rhs: &Vram) -> Vram {
        let slf = self.inner().get();

        Vram::new(slf.wrapping_add(rhs.inner()))
    }

    /// Adds this `UserSize` to a [`Vram`] address, rreturning a VRAM address
    /// if successful.
    ///
    /// Returns `None` if the addition would overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{UserSize, Vram};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x100).unwrap());
    /// let vram = Vram::new(0x80000000);
    ///
    /// assert_eq!(user_size.add_vram_checked(&vram), Some(Vram::new(0x80000100)));
    /// ```
    ///
    /// ```
    /// use address_space::{UserSize, Vram};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x80000100).unwrap());
    /// let vram = Vram::new(0x80000000);
    ///
    /// assert_eq!(user_size.add_vram_checked(&vram), None);
    /// ```
    ///
    /// [`Vram`]: crate::vram::Vram
    #[must_use]
    pub fn add_vram_checked(&self, rhs: &Vram) -> Option<Vram> {
        let slf = self.inner().get();

        slf.checked_add(rhs.inner()).map(Vram::new)
    }

    /// Adds this `UserSize` to a [`Rom`] address, returning a new ROM address.
    ///
    /// Wraps on overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{UserSize, Rom};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x100).unwrap());
    /// let rom = Rom::new(0x1000);
    ///
    /// assert_eq!(user_size.add_rom(&rom), Rom::new(0x1100));
    /// ```
    ///
    /// ```
    /// use address_space::{UserSize, Rom};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0xFFFFFFF0).unwrap());
    /// let rom = Rom::new(0x1000);
    ///
    /// assert_eq!(user_size.add_rom(&rom), Rom::new(0xFF0));
    /// ```
    ///
    /// [`Rom`]: crate::rom::Rom
    #[must_use]
    pub fn add_rom(&self, rhs: &Rom) -> Rom {
        let slf = self.inner().get();

        Rom::new(slf.wrapping_add(rhs.inner()))
    }

    /// Adds this `UserSize` to a [`Rom`] address, returning a ROM address if
    /// successful.
    ///
    /// Returns `None` if the addition would overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{UserSize, Rom};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0x100).unwrap());
    /// let rom = Rom::new(0x1000);
    ///
    /// assert_eq!(user_size.add_rom_checked(&rom), Some(Rom::new(0x1100)));
    /// ```
    ///
    /// ```
    /// use address_space::{UserSize, Rom};
    /// use core::num::NonZeroU32;
    ///
    /// let user_size = UserSize::new(NonZeroU32::new(0xFFFFFFF0).unwrap());
    /// let rom = Rom::new(0x1000);
    ///
    /// assert_eq!(user_size.add_rom_checked(&rom), None);
    /// ```
    ///
    /// [`Rom`]: crate::rom::Rom
    #[must_use]
    pub fn add_rom_checked(&self, rhs: &Rom) -> Option<Rom> {
        let slf = self.inner().get();

        slf.checked_add(rhs.inner()).map(Rom::new)
    }
}

impl ops::Add<Vram> for UserSize {
    type Output = Vram;

    fn add(self, rhs: Vram) -> Self::Output {
        self.add_vram(&rhs)
    }
}

impl ops::Add<UserSize> for Vram {
    type Output = Self;

    fn add(self, rhs: UserSize) -> Self::Output {
        rhs.add_vram(&self)
    }
}
impl ops::AddAssign<UserSize> for Vram {
    fn add_assign(&mut self, rhs: UserSize) {
        *self = *self + rhs;
    }
}

impl ops::Add<Rom> for UserSize {
    type Output = Rom;

    fn add(self, rhs: Rom) -> Self::Output {
        self.add_rom(&rhs)
    }
}

impl ops::Add<UserSize> for Rom {
    type Output = Self;

    fn add(self, rhs: UserSize) -> Self::Output {
        rhs.add_rom(&self)
    }
}
impl ops::AddAssign<UserSize> for Rom {
    fn add_assign(&mut self, rhs: UserSize) {
        *self = *self + rhs;
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
