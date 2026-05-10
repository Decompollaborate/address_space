/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use super::{AddressRange, Rom, Size, Vram};

/// A paired ROM and VRAM address range for address space translation.
///
/// This type represents a "mapping" between ROM addresses and VRAM addresses.
/// It maintains both a ROM range and a corresponding VRAM range that must have
/// compatible sizes.
///
/// For a valid `RomVramRange`:
/// - The VRAM range must be at least as large as the ROM range. It may be
///   larger because it may include the `nobits` (bss) section size.
/// - Both ranges must have the same alignment relative to the provided
///   alignment value.
///
/// # Examples
///
/// ```
/// use address_space::{RomVramRange, AddressRange, Rom, Vram};
///
/// let rom_range = AddressRange::new(Rom::new(0x1000), Rom::new(0x2000)).unwrap();
/// let vram_range = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80001000)).unwrap();
///
/// // Create a range with 4-byte alignment
/// let range = RomVramRange::new(rom_range, vram_range, 4);
/// assert!(range.is_some());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RomVramRange {
    rom: AddressRange<Rom>,
    vram: AddressRange<Vram>,
}

impl RomVramRange {
    /// Creates a new ROM-VRAM range.
    ///
    /// Returns `None` if:
    /// - The VRAM range is smaller than the ROM range.
    /// - The alignment of the start addresses differs (modulo `alignment`).
    ///   - This check is skipped if `alignment` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    ///
    /// assert!(RomVramRange::new(rom, vram, 4).is_some());
    /// ```
    #[must_use]
    pub fn new(rom: AddressRange<Rom>, vram: AddressRange<Vram>, alignment: u32) -> Option<Self> {
        if vram.size() < rom.size() {
            return None;
        }
        if alignment != 0 && vram.start().inner() % alignment != rom.start().inner() % alignment {
            return None;
        }

        Some(Self { rom, vram })
    }

    /// Creates a new ROM-VRAM range from Option<[`AddressRange`]>.
    ///
    /// Returns `None` if:
    /// - The VRAM range is smaller than the ROM range.
    /// - The alignment of the start addresses differs (modulo `alignment`).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Size, Rom, Vram};
    ///
    /// let size = Size::new(0x100);
    /// let rom = AddressRange::new_size(Rom::new(0x1000), size);
    /// let vram = AddressRange::new_size(Vram::new(0x80000000), size);
    ///
    /// assert!(RomVramRange::new_option(rom, vram, 4).is_some());
    /// ```
    ///
    /// [`AddressRange`]: crate::AddressRange
    #[must_use]
    pub fn new_option(
        rom: Option<AddressRange<Rom>>,
        vram: Option<AddressRange<Vram>>,
        alignment: u32,
    ) -> Option<Self> {
        Self::new(rom?, vram?, alignment)
    }

    /// Creates a new ROM-VRAM range from raw ROM and VRAM ranges.
    ///
    /// Returns `None` if:
    /// - The VRAM range is smaller than the ROM range.
    /// - The alignment of the start addresses differs (modulo `alignment`).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom_start = Rom::new(0x1000);
    /// let rom_end = Rom::new(0x1100);
    /// let vram_start = Vram::new(0x80000000);
    /// let vram_end = Vram::new(0x80000100);
    ///
    /// let range = RomVramRange::new_raw(rom_start, rom_end, vram_start, vram_end, 4);
    ///
    /// assert!(range.is_some());
    /// ```
    #[must_use]
    pub fn new_raw(
        rom_start: Rom,
        rom_end: Rom,
        vram_start: Vram,
        vram_end: Vram,
        alignment: u32,
    ) -> Option<Self> {
        let rom = AddressRange::new(rom_start, rom_end)?;
        let vram = AddressRange::new(vram_start, vram_end)?;
        Self::new(rom, vram, alignment)
    }

    /// Creates a new ROM-VRAM range that share the same `Size` for both.
    ///
    /// Returns `None` if:
    /// - The given size overflows the given ROM.
    /// - The given size overflows the given VRAM.
    /// - The alignment of the start addresses differs (modulo `alignment`).
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram, Size};
    ///
    /// let rom_start = Rom::new(0x1000);
    /// let vram_start = Vram::new(0x80000000);
    /// let size = Size::new(0x100);
    ///
    /// let range = RomVramRange::new_size(rom_start, vram_start, size, 4);
    ///
    /// assert!(range.is_some());
    /// ```
    #[must_use]
    pub fn new_size(rom_start: Rom, vram_start: Vram, size: Size, alignment: u32) -> Option<Self> {
        let rom = AddressRange::new_size(rom_start, size)?;
        let vram = AddressRange::new_size(vram_start, size)?;
        Self::new(rom, vram, alignment)
    }

    /// Returns a reference to the ROM range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    /// let range = RomVramRange::new(rom, vram, 4).unwrap();
    ///
    /// assert_eq!(range.rom().start(), Rom::new(0x1000));
    /// ```
    #[must_use]
    pub const fn rom(&self) -> &AddressRange<Rom> {
        &self.rom
    }

    /// Returns a reference to the VRAM range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    /// let range = RomVramRange::new(rom, vram, 4).unwrap();
    ///
    /// assert_eq!(range.vram().start(), Vram::new(0x80000000));
    /// ```
    #[must_use]
    pub const fn vram(&self) -> &AddressRange<Vram> {
        &self.vram
    }

    /// Returns whether a ROM address is within this ROM range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    /// let range = RomVramRange::new(rom, vram, 4).unwrap();
    ///
    /// assert!(range.in_rom_range(Rom::new(0x1050)));
    /// assert!(!range.in_rom_range(Rom::new(0x2000)));
    /// ```
    #[must_use]
    pub fn in_rom_range(&self, rom: Rom) -> bool {
        self.rom.in_range(rom)
    }

    /// Returns whether a VRAM address is within this VRAM range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    /// let range = RomVramRange::new(rom, vram, 4).unwrap();
    ///
    /// assert!(range.in_vram_range(Vram::new(0x80000050)));
    /// assert!(!range.in_vram_range(Vram::new(0x80001000)));
    /// ```
    #[must_use]
    pub fn in_vram_range(&self, vram: Vram) -> bool {
        self.vram.in_range(vram)
    }

    /// Converts a ROM address to its corresponding VRAM address.
    ///
    /// Returns `None` if the ROM address is not within this ROM range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    /// let range = RomVramRange::new(rom, vram, 4).unwrap();
    ///
    /// assert_eq!(range.vram_from_rom(Rom::new(0x1050)), Some(Vram::new(0x80000050)));
    /// assert_eq!(range.vram_from_rom(Rom::new(0x2000)), None);
    /// ```
    #[must_use]
    pub fn vram_from_rom(&self, rom: Rom) -> Option<Vram> {
        if self.rom.in_range(rom) {
            let diff = rom.sub_rom(&self.rom.start());
            Some(self.vram.start().add_size(&diff))
        } else {
            None
        }
    }

    /// Converts a VRAM address to its corresponding ROM address.
    ///
    /// Returns `None` if the VRAM address is not within this VRAM range, or if
    /// the resulting ROM is outside the ROM's range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    /// let range = RomVramRange::new(rom, vram, 4).unwrap();
    ///
    /// assert_eq!(range.rom_from_vram(Vram::new(0x80000050)), Some(Rom::new(0x1050)));
    /// assert_eq!(range.rom_from_vram(Vram::new(0x80001000)), None);
    /// ```
    #[must_use]
    pub fn rom_from_vram(&self, vram: Vram) -> Option<Rom> {
        if self.vram.in_range(vram) {
            let diff = vram.sub_vram(&self.vram.start());

            let rom = self.rom.start().add_size(&diff);
            // VRAM may be larger than ROM, so we need to check this
            if self.rom.in_range(rom) {
                Some(rom)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl RomVramRange {
    fn expand_rom_range(&mut self, other: &AddressRange<Rom>) {
        self.rom.expand_range(other);
    }
    fn expand_vram_range(&mut self, other: &AddressRange<Vram>) {
        self.vram.expand_range(other);
    }

    /// Expands this ROM-VRAM range to encompass another.
    ///
    /// After calling this, both the ROM and VRAM ranges will be expanded
    /// to encompass the corresponding ranges of the other range.
    ///
    /// # Examples
    ///
    /// ```
    /// use address_space::{RomVramRange, AddressRange, Rom, Vram};
    ///
    /// let rom1 = AddressRange::new(Rom::new(0x1000), Rom::new(0x1100)).unwrap();
    /// let vram1 = AddressRange::new(Vram::new(0x80000000), Vram::new(0x80000100)).unwrap();
    /// let mut range1 = RomVramRange::new(rom1, vram1, 4).unwrap();
    ///
    /// let rom2 = AddressRange::new(Rom::new(0x1050), Rom::new(0x1200)).unwrap();
    /// let vram2 = AddressRange::new(Vram::new(0x80000050), Vram::new(0x80000200)).unwrap();
    /// let range2 = RomVramRange::new(rom2, vram2, 4).unwrap();
    ///
    /// range1.expand_ranges(&range2);
    /// assert_eq!(range1.rom().start(), Rom::new(0x1000));
    /// assert_eq!(range1.rom().end(), Rom::new(0x1200));
    /// ```
    pub fn expand_ranges(&mut self, other: &Self) {
        self.expand_rom_range(&other.rom);
        self.expand_vram_range(&other.vram);
    }
}
