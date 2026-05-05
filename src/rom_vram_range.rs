/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

use super::{AddressRange, Rom, Size, Vram};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RomVramRange {
    rom: AddressRange<Rom>,
    vram: AddressRange<Vram>,
}

impl RomVramRange {
    #[must_use]
    pub fn new(rom: AddressRange<Rom>, vram: AddressRange<Vram>, min_alignment: u32) -> Option<Self> {
        if vram.size() < rom.size() {
            return None;
        }
        if vram.start().inner() % min_alignment != rom.start().inner() % min_alignment {
            return None;
        }

        Some(Self { rom, vram })
    }

    #[must_use]
    pub const fn rom(&self) -> &AddressRange<Rom> {
        &self.rom
    }
    #[must_use]
    pub const fn vram(&self) -> &AddressRange<Vram> {
        &self.vram
    }

    #[must_use]
    pub fn in_rom_range(&self, rom: Rom) -> bool {
        self.rom.in_range(rom)
    }
    #[must_use]
    pub fn in_vram_range(&self, vram: Vram) -> bool {
        self.vram.in_range(vram)
    }

    #[must_use]
    pub fn vram_fom_rom(&self, rom: Rom) -> Option<Vram> {
        if self.rom.in_range(rom) {
            let diff = rom.sub_rom(&self.rom.start())?;
            Some(self.vram.start() + diff)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rom_from_vram(&self, vram: Vram) -> Option<Rom> {
        if self.vram.in_range(vram) {
            let diff = Size::try_from(vram - self.vram.start()).expect("This should not panic because `vram` is inside our range, meaning it is larger than our vram's start");
            Some(self.rom.start() + diff)
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
    pub fn expand_ranges(&mut self, other: &Self) {
        self.expand_rom_range(&other.rom);
        self.expand_vram_range(&other.vram);
    }
}
