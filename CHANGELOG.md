# Changelog: naive_opt
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2026-05-27
### Changed
- Update `criterion` crate to 0.8 in `xbench`.

## [0.2.1] - 2026-05-24
### Added
- `make list` to `Makefile`.
- Formal Inlining Strategy documentation in `src/lib.rs` to guide future development.

### Changed
- Optimize `char` search to be zero-allocation by using `encode_utf8` with a stack buffer instead of `to_string()`.
- Refine inlining strategy (maintain `#[inline(always)]` for dispatchers and critical iterator methods; adopt `#[inline]` for core logic and shared helpers).
- Refactor `mc_1st` and `mc_last` to use unified generic search logic in `mc_generic`, significantly reducing code duplication.
- Consolidate ASCII stochastics decision logic into a shared `prefer_1st_strategy` helper for better maintainability.

### Fixed
- Clippy warning `useless_concat`.

## [0.2.0] - 2025-09-24
### Added
- Project specifications in `specs/`.
- `includes_bytes()` function.
- `includes_bytes_ignore_ascii_case()` function.
- Additional tests.

### Changed
- Update `memx` to 0.2.0.
- Set `rust-version` to "1.65.0".

### Fixed
- Issue where `SearchIndices` failed when the needle was empty.

## [0.1.25] - 2024-06-18
### Changed
- Rename `config` to `config.toml`.
- Update `criterion` crate to 0.5.1.
- Support for Rust 1.60.0 in GitHub workflows.

### Fixed
- Clippy warning `redundant_as_str`.

## [0.1.24] - 2023-08-04
### Changed
- Rename `memchr_double_iter` to `::memx::iter::memchr_dbl_iter`.
- Rename `memrchr_double_iter` to `::memx::iter::memrchr_dbl_iter`.

## [0.1.23] - 2023-02-13
### Changed
- Refactor `Makefile`.

### Removed
- `COPYING` file.

### Fixed
- `LICENSE-APACHE` and `LICENSE-MIT` files.
- Clippy warning `redundant_field_names`.
- Clippy warning `unnecessary_unwrap`.
- Clippy warning `needless_bool`.
- Clippy warning `uninlined_format_args`.

## [0.1.22] - 2023-01-31
### Added
- `test_diskstats()` in `func.rs`.
- GitHub workflows for Ubuntu, macOS, and Windows.
- Test status badges in `README.tpl`.
- Minimum supported Rust version (MSRV) 1.56.0 in `Cargo.toml`.

### Fixed
- Clippy warning `needless_borrow`.
- Clippy warning `unnecessary_to_owned`.
- Clippy warning `search_is_some`.
- Clippy warning `single_char_pattern`.

## [0.1.21] - 2023-01-10
### Added
- Badges in `README.tpl`.

### Changed
- Reformat `CHANGELOG.md`.
- Move benchmarks to `xbench` directory.

## [0.1.20] - 2023-01-05
### Removed
- Old `memx/std` feature from `Cargo.toml`.

## [0.1.19] - 2023-01-05 [YANKED]
### Added
- `lto = true` to release profile in `Cargo.toml`.

### Changed
- Update benchmark results.
- Update `criterion` crate to 0.4.
- Criterion unit from 'uc' to 'μc'.

### Fixed
- Clippy warning regarding unused lifetime in implementation.

## [0.1.18] - 2022-06-13
### Changed
- Switch to Rust 2021 edition.

## [0.1.17] - 2022-02-11
### Added
- Multiple `xxx_ignore_ascii_case()` functions.

## [0.1.16] - 2021-11-14
### Changed
- Update `memx` crate to 0.1.18.
- Update `serde_json` crate to 1.0.70.

## [0.1.15] - 2021-09-11
### Changed
- Update `memx` crate to 0.1.17.

## [0.1.14] - 2021-09-10
### Changed
- Update crates: `memx` (0.1.16), `anyhow` (1.0.43), and `criterion` (0.3.5).

## [0.1.13] - 2021-07-06
### Changed
- Update `memx` crate to 0.1.14.
- Rewrite documentation.
- Update licenses.

## [0.1.12] - 2021-06-20
### Added
- `Cargo.lock` to `.gitignore`.

### Changed
- Update `memx` crate to 0.1.12 (bug fixes).

## [0.1.11] - 2021-06-06
### Added
- ASCII stochastics.
- `naive_opt_mc_1st_bytes()` and `naive_opt_mc_1st_rev_bytes()` functions.

### Changed
- Speed of ASCII searching.

## [0.1.10] - 2021-06-03
### Added
- Test case for large needle size.
- `memx` crate to dependencies.

### Changed
- Benchmark data with larger match sizes.

### Removed
- `libc` and `memchr` crates.

## [0.1.9] - 2021-05-09
### Changed
- Update `memchr` dependency to 2.4.0.
- Split code into `fallback.rs`.

## [0.1.8] - 2021-04-20
### Added
- `xxx_bytes()` functions.

## [0.1.7] - 2021-04-09
### Added
- `output2()` to `shape_benchmark_results` task.
- `libc` support for non-x86_64 architectures.

## [0.1.6] - 2021-04-04
### Added
- `is_empty()` method to `SearchIn` trait.

### Changed
- Clippy warnings.

## [0.1.5] - 2021-03-20
### Added
- `Search::rsearch()` method.
- `Search::rsearch_indices()` method.
- `string_rsearch()` function.
- `string_rsearch_indices()` function.
- Additional documentation.

## [0.1.4] - 2021-03-19
### Added
- `SearchIn` implementation for `char`.
- `Search::includes()` method.

## [0.1.3] - 2021-03-18
### Changed
- Performance of `search_indices`.

## [0.1.2] - 2021-03-17
### Added
- `search_indices` function.

## [0.1.1] - 2021-03-17
### Added
- Documentation.

## [0.1.0] - 2021-03-17
### Added
- Initial release.

[Unreleased]: https://github.com/aki-akaguma/naive_opt/compare/v0.2.2..HEAD
[0.2.2]: https://github.com/aki-akaguma/naive_opt/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/aki-akaguma/naive_opt/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.25..v0.2.0
[0.1.25]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.24..v0.1.25
[0.1.24]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/naive_opt/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/naive_opt/releases/tag/v0.1.0
