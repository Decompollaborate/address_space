/* SPDX-FileCopyrightText: © 2026 Decompollaborate */
/* SPDX-License-Identifier: MIT OR Apache-2.0 */

//! # Address Space
//!
//! A Rust crate for working with MIPS address spaces, providing types for managing
//! ROM and VRAM addresses, sizes, and ranges.
//!
//! ## Overview
//!
//! This crate provides a collection of types to handle address calculations
//! for MIPS emulation and decomposition projects:
//!
//! - [`Vram`]: Virtual RAM address representation.
//! - [`VramOffset`]: Offset or difference between VRAM addresses.
//! - [`Rom`]: ROM address representation.
//! - [`Size`]: Unsigned size representation.
//! - [`UserSize`]: Non-zero size wrapper.
//! - [`GpValue`]: GP register value representation.
//! - [`AddressRange<T>`]: Generic address range for any address type.
//! - [`RomVramRange`]: Paired ROM and VRAM range for address translation.
//!
//! ### Quick Start
//!
//! ```
//! use address_space::{Vram, Rom, Size, AddressRange, RomVramRange};
//!
//! # fn test_func() -> Option<()> {
//!
//! // Create addresses
//! let vram = Vram::new(0x80000000);
//! let rom = Rom::new(0x1000);
//!
//! // Perform arithmetic
//! let size = Size::new(0x100);
//! let new_vram = vram + size;
//! let new_rom = rom + size;
//!
//! // Work with ranges
//! let range = AddressRange::new_size(vram, size)?;
//! assert_eq!(range.start(), vram);
//! assert_eq!(range.size(), size);
//! assert_eq!(range.end(), Vram::new(0x80000100));
//!
//! // Convert between ROM and VRAM
//! let size = Size::new(0x4000);
//! let rom_vram = RomVramRange::new(
//!     AddressRange::new_size(Rom::new(0x1000), size)?,
//!     AddressRange::new_size(Vram::new(0x80000000), size)?,
//!     4,
//! )?;
//! let rom_addr = Rom::new(0x1500);
//! let vram_addr = rom_vram.vram_from_rom(rom_addr);
//! assert_eq!(vram_addr, Some(Vram::new(0x80000500)));
//!
//! # Some(())
//! # }
//! # test_func();
//! ```
//!
//! ### Wrapping Arithmetic
//!
//! Arithmetic operations on types use wrapping arithmetic by default.
//!
//! #### Examples
//!
//! ```
//! use address_space::{Vram, Rom, Size};
//!
//! // Wrapping addition
//! let vram = Vram::new(0x80000000);
//! let size = Size::new(0x80001000);
//! assert_eq!(vram + size, Vram::new(0x1000));
//!
//! let size1 = Size::new(0xFFFF0000);
//! let size2 = Size::new(0x00010000);
//! assert_eq!(size1 + size2, Size::new(0)); // wraps around
//!
//! // Wrapping subtraction
//! let rom3 = Rom::new(0x1000);
//! let rom4 = Rom::new(0x1100);
//! assert_eq!(rom3.sub_rom(&rom4), Size::new(0xFFFFFF00));
//! ```
//!
//! ## Features
//!
//! - `std`: Enable standard library support.
//! - `try_from`: Implement `TryFrom` conversions (bumps MSRV to 1.34).
//! - `error`: Implement the `Error` trait on error types (bumps MSRV to 1.81).
//! - `serde`: Implement serde `Serialize` and `Deserialize` traits.
//!
//! ## `no_std` Support
//!
//! This crate is `no_std` and `no_alloc` by default.

/*
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::too_many_lines)] // ?
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cast_lossless)] // maybe warn?
#![allow(clippy::match_same_arms)] // maybe warn?
#![allow(clippy::trivially_copy_pass_by_ref)] // ?
#![allow(clippy::unused_self)]
#![allow(clippy::doc_markdown)] // ?
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::missing_errors_doc)] // maybe warn?
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::unreadable_literal)]
*/

/*
#![warn(clippy::restriction)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::same_name_method)]
#![allow(clippy::min_ident_chars)]
#![allow(clippy::as_conversions)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::implicit_return)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::missing_inline_in_public_items)] // !
#![allow(clippy::default_numeric_fallback)]
#![allow(clippy::missing_docs_in_private_items)] // warn!
#![allow(clippy::indexing_slicing)] // TODO: consider
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::allow_attributes)] // TODO: consider
#![allow(clippy::todo)] // TODO: consider
#![allow(clippy::pub_use)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::shadow_unrelated)] // TODO: consider
#![allow(clippy::wildcard_enum_match_arm)]
#![allow(clippy::if_then_some_else_none)] // TODO: consider
#![allow(clippy::unwrap_used)] // TODO: consider
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::single_char_lifetime_names)] // TODO: consider
#![allow(clippy::field_scoped_visibility_modifiers)]
#![allow(clippy::else_if_without_else)]
#![allow(clippy::empty_enum_variants_with_brackets)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::error_impl_error)]
#![allow(clippy::self_named_module_files)]
*/

/*
#![warn(clippy::nursery)]
#![allow(clippy::redundant_pub_crate)]
*/
#![deny(unreachable_patterns)]
#![allow(clippy::exhaustive_enums)]
#![warn(clippy::use_self)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::pattern_type_mismatch)]
// #![warn(clippy::missing_inline_in_public_items)] // TODO
#![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::doc_markdown)] // ?
#![warn(clippy::missing_errors_doc)]
#![allow(clippy::pub_with_shorthand)]
#![warn(clippy::pub_without_shorthand)]
// #![warn(clippy::option_if_let_else)] // It can get kinda ugly. Reconsider later
#![warn(clippy::option_map_or_none)]
#![warn(clippy::bind_instead_of_map)]
#![warn(clippy::cognitive_complexity)] // Maybe remove in the future (?)
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::ref_option)]
#![warn(clippy::manual_let_else)]
#![allow(clippy::manual_non_exhaustive)]
#![allow(clippy::pattern_type_mismatch)]
#![allow(clippy::collapsible_match)] // Automatic fixing can break code. Also it doesn't look better either
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::manual_is_variant_and)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::unused_result_ok)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::panic)]
#![warn(clippy::semicolon_if_nothing_returned)]
//
#![cfg_attr(not(feature = "std"), no_std)]
//
#![cfg_attr(docsrs, feature(doc_cfg))]

mod vram;
mod vram_offset;

mod rom;

mod size;
mod user_size;

mod gp_value;

mod address_range;
mod rom_vram_range;

pub(crate) mod utils;

pub use self::vram::Vram;
pub use self::vram_offset::VramOffset;

pub use self::rom::Rom;

pub use self::size::{ConvertToSizeError, Size};
pub use self::user_size::UserSize;

pub use self::gp_value::GpValue;

pub use self::address_range::AddressRange;
pub use self::rom_vram_range::RomVramRange;
