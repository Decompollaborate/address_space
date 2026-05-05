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
    pub fn new(rom: AddressRange<Rom>, vram: AddressRange<Vram>) -> Self {
        assert!(
            vram.size() >= rom.size(),
            "Can't create RomVramRange: The size of the Vram range must be equal or greater than the size of the Rom range.\nVram range is 0x{} ~ 0x{} (Size {}).\n Rom range is 0x{:08X} ~ 0x{:08X} (Size {}).",
            vram.start(),
            vram.end(),
            vram.size(),
            rom.start().inner(),
            rom.end().inner(),
            rom.size(),
        );
        assert!(
            vram.start().inner() % 4 == rom.start().inner() % 4,
            "vram ({:?}) and rom ({:?}) must have the same alignment",
            vram, rom,
        );

        Self { rom, vram }
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
            let diff = rom - self.rom.start();
            Some(self.vram.start() + diff)
        } else {
            None
        }
    }

    #[must_use]
    pub fn rom_from_vram(&self, vram: Vram) -> Option<Rom> {
        if self.vram.in_range(vram) {
            let diff = Size::try_from(vram - self.vram.start())
                .expect("This should not panic");
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
