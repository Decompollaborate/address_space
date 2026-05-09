/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::fmt;

/// Represents a value stored in a GP (Global Pointer) register.
///
/// The GP register is a special register commonly used in MIPS code to hold
/// a base pointer to data. This type simply wraps a 32-bit value that would be
/// stored in such a register.
///
/// # Examples
///
/// ```
/// use address_space::GpValue;
///
/// let gp = GpValue::new(0x800A0000);
/// assert_eq!(gp.inner(), 0x800A0000);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GpValue {
    inner: u32,
}

impl GpValue {
    /// Constructs a `GpValue` from a given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::GpValue;
    ///
    /// let gp = GpValue::new(0xDEADBEEF);
    /// assert_eq!(gp.inner(), 0xDEADBEEF);
    /// ```
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    /// Returns the internal GP register value.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::GpValue;
    ///
    /// let gp = GpValue::new(0x12345678);
    /// assert_eq!(gp.inner(), 0x12345678);
    /// ```
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
