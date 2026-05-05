/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use core::{
    fmt,
    ops::{self, Add},
};

use super::{Rom, Size, Vram};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressRange<T> {
    start: T,
    end: T,
}

impl<T> AddressRange<T>
where
    T: Copy + PartialOrd + fmt::Debug,
{
    #[must_use]
    pub fn new(start: T, end: T) -> Option<Self> {
        if start > end {
            None
        } else {
            Some(Self { start, end })
        }
    }

    #[must_use]
    pub fn new_by_size<S>(start: T, size: S) -> Option<Self>
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

    #[must_use]
    pub fn start(&self) -> T {
        self.start
    }

    #[must_use]
    pub fn end(&self) -> T {
        self.end
    }
}

impl AddressRange<Vram> {
    #[must_use]
    pub fn size(&self) -> Size {
        // Casting to unsigned should be fine because we know `self.end` is always greater or equal than `self.start`.
        Size::new(self.end.sub_vram(&self.start).inner() as u32)
    }
}

impl AddressRange<Rom> {
    #[must_use]
    pub const fn size(&self) -> Size {
        // TODO: Add a substraction method on Rom
        Size::new(self.end.inner() - self.start.inner())
    }
}

impl<T> AddressRange<T>
where
    T: Copy + PartialOrd,
{
    #[must_use]
    pub fn in_range(&self, value: T) -> bool {
        self.start <= value && value < self.end
    }
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
    pub fn expand_range(&mut self, other: &Self) {
        self.decrease_start(other.start);
        self.increase_end(other.end);
    }

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
