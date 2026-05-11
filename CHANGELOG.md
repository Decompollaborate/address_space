# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-05-10

### Added

- Add `Size::add_size_checked`.
- Add `Size::add_user_size`.
- Add `Size::add_user_size_checked`.
- Add multiple `core::ops::Add` trait implementations for `UserSize`.
- Add `core::ops::Add` trait implementations for `UserSize` and `Size`.

### Changed

- Rename `UserSize::add_user_size` to `add_user_size_checked`.
- Rename `UserSize::add_size` to `add_size_checked`.

## [0.1.1] - 2026-05-10

### Added

- Add `RomVramRange::new_size` method.

## [0.1.0] - 2026-05-10

### Added

- Initial release.

[unreleased]: https://github.com/Decompollaborate/address_space/compare/0.2.0...main

[0.2.0]: https://github.com/Decompollaborate/address_space/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/Decompollaborate/address_space/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/Decompollaborate/address_space/releases/tag/0.1.0
