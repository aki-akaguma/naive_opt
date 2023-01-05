# Changelog: naive_opt

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased


## 0.1.20 (2023-01-05)
### Removed
* "memx/std" of Cargo.toml, this is a old feature.

## 0.1.19 (2023-01-05) YANKED
### Added
* lto = true into profile.release of Carg.toml

### Changed
* update benchmark results
* update crates: criterion(0.4)
* change criterion unit 'uc' to 'μc'

### Fixed
* clippy: this lifetime isn't used in the impl

## 0.1.18 (2022-06-13)
### Changed
* changes to edition 2021

## 0.1.17 (2022-02-11)
### Added
* add many `xxx_ignore_ascii_case()`.

## 0.1.16 (2021-11-14)
### Changed
* update crates: memx(0.1.18)
* update crates: serde_json(1.0.70)

## 0.1.15 (2021-09-11)
### Changed
* update crates: memx(0.1.17)

## 0.1.14 (2021-09-10)
### Changed
* update crates: memx(0.1.16), anyhow(1.0.43), criterion(0.3.5)

## 0.1.13 (2021-07-06)
### Changed
* update crates: memx(0.1.14)
* rewrite doc
* update licenses

## 0.1.12 (2021-06-20)
### Added
* add "Cargo.lock" into .gitignore

### Changed
* update crates: memx(0.1.12) - the important bugs fixed

## 0.1.11 (2021-06-06)
### Added
* add ascii stochastics
* add naive_opt_mc_1st_bytes() and naive_opt_mc_1st_rev_bytes()

### Changed
* faster ascii searching

## 0.1.10 (2021-06-03)
### Added
* add to test: a needle large size.
* add crate memx into depends.

### Changed
* replace new bench data: a match size is more than old.

### Removed
* remove crate libc and crate memchr.

## 0.1.9 (2021-05-09)
### Changed
* update depends: memchr(2.4.0)
* split into fallback.rs

## 0.1.8 (2021-04-20)
### Added
* add xxx_bytes()

## 0.1.7 (2021-04-09)
### Added
* add output2() into `task shape_benchmark_results`
* add libc support for 'cfg(not(target_arch = "x86_64"))'

## 0.1.6 (2021-04-04)
### Added
* add trait SearchIn.is_empty()

### Changed
* clippy fix

## 0.1.5 (2021-03-20)
### Added
* add Search::rsearch()
* add Search::rsearch_indices()
* add string_rsearch()
* add string_rsearch_indices()
* add more docs

## 0.1.4 (2021-03-19)
### Added
* add impl SearchIn for char
* add Search::includes()

## 0.1.3 (2021-03-18)
### Changed
* tune up search_indices

## 0.1.2 (2021-03-17)
### Added
* add search_indices

## 0.1.1 (2021-03-17)
### Added
* add docs

## 0.1.0 (2021-03-17)
* first commit
