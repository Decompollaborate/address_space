# Address space

A Rust crate for working with MIPS address spaces, providing types for managing
ROM and VRAM addresses, sizes, and ranges.

## Overview

This crate provides a collection of types to handle address calculations for
MIPS assembly analysis and handling:

- `Vram`: Virtual RAM address representation.
- `Rom`: ROM address representation.
- `Size`: Unsigned size representation.
- `UserSize`: Non-zero size wrapper.
- `VramOffset`: Offset or difference between VRAM addresses.
- `GpValue`: GP register value representation.
- `AddressRange<T>`: Generic address range for any address type.
- `RomVramRange`: Paired ROM and VRAM range for address translation.

### Quick Start

```rust
use address_space::{Vram, Rom, Size, AddressRange, RomVramRange};

// Create addresses
let vram = Vram::new(0x80000000);
let rom = Rom::new(0x1000);

// Perform arithmetic
let size = Size::new(0x100);
let new_vram = vram + size;  // Vram::new(0x80000100)
let new_rom = rom + size;    // Rom::new(0x1100)

// Work with ranges
let range = AddressRange::new_size(vram, size)?;
assert_eq!(range.start(), vram);
assert_eq!(range.size(), size);
assert_eq!(range.end(), Vram::new(0x80000100));

// Convert between ROM and VRAM
let size = Size::new(0x4000);
let rom_vram = RomVramRange::new(
    AddressRange::new_size(Rom::new(0x1000), size)?,
    AddressRange::new_size(Vram::new(0x80000000), size)?,
    4,
)?;
let rom_addr = Rom::new(0x1500);
let vram_addr = rom_vram.vram_from_rom(rom_addr);
assert_eq!(vram_addr, Some(Vram::new(0x80000500)));
```

### Wrapping Arithmetic

Arithmetic operations on types use wrapping arithmetic by default.

#### Examples

```rust
use address_space::{Vram, Rom, Size};

// Wrapping addition
let vram = Vram::new(0x80000000);
let size = Size::new(0x80001000);
assert_eq!(vram + size, Vram::new(0x1000));

let size1 = Size::new(0xFFFF0000);
let size2 = Size::new(0x00010000);
assert_eq!(size1 + size2, Size::new(0)); // wraps around

// Wrapping subtraction
let rom3 = Rom::new(0x1000);
let rom4 = Rom::new(0x1100);
assert_eq!(rom3.sub_rom(&rom4), Size::new(0xFFFFFF00));
```

## Crate features

This crate supports the following optional features. None are enabled by
default.

- `std`: Enable standard library support. Disabled by default.
  - Currently this is a stub, there are no changes by enabling this feature,
    besides not using `#![no_std]`.
- `try_from`: Implement `TryFrom` trait conversions for types.
  - This feature bumps the [MSRV](#msrv) to **1.34**.
- `error`: Implement the standard `Error` trait on error types.
  - This feature enables `try_from` and bumps the [MSRV](#msrv) to **1.81**.
- `serde`: Enable serialization and deserialization support via the `serde`
  crate.

## `no_std` and `no_alloc` Support

This crate is fully `no_std` and `no_alloc` by default.

## MSRV

The minimum supported Rust version (MSRV) for this crate is **1.31**.

Current policy: MSRV may be bumped on **minor** updates.

This policy may change in major updates.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Versioning and changelog

This library follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
We try to always keep backwards compatibility, so no breaking changes should
happen until a major release (i.e. jumping from 1.X.X to 2.0.0).

To see what changed on each release visit either the
[CHANGELOG.md](https://github.com/Decompollaborate/address_space/blob/main/CHANGELOG.md)
file or check the [releases page on Github](https://github.com/Decompollaborate/address_space/releases).
You can also use [this link](https://github.com/Decompollaborate/address_space/releases/latest)
to check the latest release.
