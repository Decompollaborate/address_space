/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{
    fmt,
    ops::{self, Add},
};

use super::{Rom, Size, Vram};

/// A generic address range from a start to an end address (end exclusive).
///
/// This type represents a range of addresses, where both the start and end
/// are of the same type `T` (typically [`Vram`] or [`Rom`]). The range is
/// half-open, meaning the end address is exclusive.
///
/// `AddressRange` can be used to:
/// - Check if an address is within the range
/// - Calculate the size of the range (for specific types)
/// - Expand the range to encompass another range
/// - Check for overlaps with other ranges
///
/// # Examples
///
/// ```
/// use address_space::{AddressRange, Vram, Size};
///
/// let range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
///
/// assert!(range.in_range(Vram::new(0x80000500)));
/// assert!(!range.in_range(Vram::new(0x80002000)));
/// assert_eq!(range.size(), Size::new(0x1000));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddressRange<T> {
    start: T,
    end: T,
}

impl<T> AddressRange<T>
where
    T: Copy + PartialOrd + fmt::Debug,
{
    /// Constructs an `AddressRange` from a start and end address.
    ///
    /// Returns `None` if `start > end`, indicating an invalid range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram};
    ///
    /// let range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000));
    /// assert!(range.is_some());
    ///
    /// let invalid = AddressRange::new(Vram::new(0x80001000), Vram::new(0x80000000));
    /// assert!(invalid.is_none());
    /// ```
    #[must_use]
    pub fn new(start: T, end: T) -> Option<Self> {
        if start > end {
            None
        } else {
            Some(Self { start, end })
        }
    }

    /// Constructs an `AddressRange` from a start address and a size.
    ///
    /// Returns `None` if adding the size to the start would cause an overflow
    /// (i.e., if `start + size > end`).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram, Size};
    ///
    /// let range = AddressRange::new_size(
    ///     Vram::new(0x80000000),
    ///     Size::new(0x1000)
    /// );
    /// assert!(range.is_some());
    /// ```
    #[must_use]
    pub fn new_size<S>(start: T, size: S) -> Option<Self>
    where
        T: Add<S, Output = T>,
    {
        let end = start.add(size);

        if start > end {
            None
        } else {
            Some(Self { start, end })
        }
    }

    /// Returns the start address of this range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram};
    ///
    /// let range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
    /// assert_eq!(range.start(), Vram::new(0x80000000));
    /// ```
    #[must_use]
    pub fn start(&self) -> T {
        self.start
    }

    /// Returns the end address of this range (exclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram};
    ///
    /// let range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
    /// assert_eq!(range.end(), Vram::new(0x80001000));
    /// ```
    #[must_use]
    pub fn end(&self) -> T {
        self.end
    }
}

impl AddressRange<Vram> {
    /// Calculates the size of this VRAM range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram, Size};
    ///
    /// let range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
    /// assert_eq!(range.size(), Size::new(0x1000));
    /// ```
    #[must_use]
    pub fn size(&self) -> Size {
        self.end.sub_vram(&self.start)
    }
}

impl AddressRange<Rom> {
    /// Calculates the size of this ROM range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Rom, Size};
    ///
    /// let range = AddressRange::new(Rom::new(0x1000), Rom::new(0x2000)).unwrap();
    /// assert_eq!(range.size(), Size::new(0x1000));
    /// ```
    #[must_use]
    pub fn size(&self) -> Size {
        self.end.sub_rom(&self.start)
    }
}

impl<T> AddressRange<T>
where
    T: Copy + PartialOrd,
{
    /// Returns whether an address is within this range (end exclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram};
    ///
    /// let range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
    ///
    /// assert!(range.in_range(Vram::new(0x80000500)));
    /// assert!(range.in_range(Vram::new(0x80000000))); // start is included
    /// assert!(!range.in_range(Vram::new(0x80001000))); // end is excluded
    /// ```
    #[must_use]
    pub fn in_range(&self, value: T) -> bool {
        self.start <= value && value < self.end
    }

    /// Returns whether an address is within this range (end inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram};
    ///
    /// let range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
    ///
    /// assert!(range.in_range_inclusive_end(Vram::new(0x80001000))); // end is included
    /// assert!(!range.in_range_inclusive_end(Vram::new(0x80001001)));
    /// ```
    #[must_use]
    pub fn in_range_inclusive_end(&self, value: T) -> bool {
        self.start <= value && value <= self.end
    }

    fn decrease_start(&mut self, value: T) {
        if value < self.start {
            self.start = value;
        }
    }
    fn increase_end(&mut self, value: T) {
        if value >= self.end {
            self.end = value;
        }
    }

    /// Expands this range to encompass another range.
    ///
    /// After calling this, this range will start at the minimum of the two starts
    /// and end at the maximum of the two ends.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram};
    ///
    /// let mut range1 = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
    /// let range2 = AddressRange::new(Vram::new(0x80000500), Vram::new(0x80002000)).unwrap();
    ///
    /// range1.expand_range(&range2);
    /// assert_eq!(range1.start(), Vram::new(0x80000000));
    /// assert_eq!(range1.end(), Vram::new(0x80002000));
    /// ```
    pub fn expand_range(&mut self, other: &Self) {
        self.decrease_start(other.start);
        self.increase_end(other.end);
    }

    /// Returns whether this range overlaps with another range.
    ///
    /// Two ranges overlap if they share any address in common.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{AddressRange, Vram};
    ///
    /// let range1 = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
    /// let range2 = AddressRange::new(Vram::new(0x80000500), Vram::new(0x80002000)).unwrap();
    /// let range3 = AddressRange::new(Vram::new(0x80002000), Vram::new(0x80003000)).unwrap();
    ///
    /// assert!(range1.overlaps(&range2)); // Partial overlap
    /// assert!(!range1.overlaps(&range3)); // No overlap (adjacent)
    /// ```
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

impl<T> fmt::Display for AddressRange<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.start, self.end)
    }
}

impl ops::Index<AddressRange<Rom>> for [u8] {
    type Output = [u8];

    #[inline]
    #[allow(clippy::indexing_slicing)]
    fn index(&self, index: AddressRange<Rom>) -> &Self::Output {
        &self[index.start.inner() as usize..index.end.inner() as usize]
    }
}

impl<T> ops::RangeBounds<T> for AddressRange<T> {
    fn start_bound(&self) -> ops::Bound<&T> {
        ops::Bound::Included(&self.start)
    }

    fn end_bound(&self) -> ops::Bound<&T> {
        ops::Bound::Excluded(&self.end)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_address_range_overlaps_no() {
        let x = AddressRange::new(0, 0x10).unwrap();
        let y = AddressRange::new(0x10, 0x20).unwrap();

        assert!(!x.overlaps(&y));
        assert!(!y.overlaps(&x));
    }

    #[test]
    fn test_address_range_overlaps_embedded() {
        let x = AddressRange::new(0, 0x10).unwrap();
        let y = AddressRange::new(0x4, 0x8).unwrap();

        assert!(x.overlaps(&y));
        assert!(y.overlaps(&x));
    }

    #[test]
    fn test_address_range_overlaps_half() {
        let x = AddressRange::new(0x4, 0x10).unwrap();
        let y = AddressRange::new(0x8, 0x18).unwrap();

        assert!(x.overlaps(&y));
        assert!(y.overlaps(&x));

        let x = AddressRange::new(0x4, 0x10).unwrap();
        let y = AddressRange::new(0x2, 0x8).unwrap();

        assert!(x.overlaps(&y));
        assert!(y.overlaps(&x));
    }
}
