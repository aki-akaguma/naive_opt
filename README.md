# naive_opt

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

The optimized naive string-search algorithm.

## Features

- The naive string-searching algorithm
- Enhanced with 1-byte search like the libc++ and the libstd++ string::find
- Specializing in UTF-8 strings, which is a feature of rust
- The ASCII Stochastics search
- Support the zero overhead trait.
- Support ignore ascii case match.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Compatibility

This crate is implemented to replace the rust std library.
However, the method names are different, so please rewrite your code.
It shouldn't be too difficult.

compatibility:

| rust `std::str`              | this crate                             |
|:-----------------------------|:---------------------------------------|
| `std::str::find()`           | `naive_opt::Search::search()`          |
| `std::str::rfind()`          | `naive_opt::Search::rsearch()`         |
| `std::str::contains()`       | `naive_opt::Search::includes()`        |
| `std::str::match_indices()`  | `naive_opt::Search::search_indices()`  |
| `std::str::rmatch_indices()` | `naive_opt::Search::rsearch_indices()` |

## Ignore ascii case match

This crate supports an ASCII case-insensitive match with each function.

| this crate                                               |
|:---------------------------------------------------------|
| `naive_opt::Search::search_ignore_ascii_case()`          |
| `naive_opt::Search::rsearch_ignore_ascii_case()`         |
| `naive_opt::Search::includes_ignore_ascii_case()`        |
| `naive_opt::Search::search_indices_ignore_ascii_case()`  |
| `naive_opt::Search::rsearch_indices_ignore_ascii_case()` |

## Examples

### Example function:

```rust
use naive_opt::{string_search, string_rsearch};
use naive_opt::{string_search_indices, string_rsearch_indices};

let haystack = "111 a 111b";
let needle = "a";
let r = string_search(haystack, needle);
assert_eq!(r, Some(4));

assert_eq!(string_search(haystack, "1"), Some(0));
assert_eq!(string_rsearch(haystack, "1"), Some(8));

let v: Vec<_> = string_search_indices("abc345abc901abc", "abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
let v: Vec<_> = string_rsearch_indices("abc345abc901abc", "abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
```

### Example trait: Search

```rust
use naive_opt::Search;

let haystack = "111 a 111b";
let needle = "a";
let r = haystack.search(needle);
assert_eq!(r, Some(4));

assert_eq!(haystack.search("1"), Some(0));
assert_eq!(haystack.rsearch("1"), Some(8));

let v: Vec<_> = "abc345abc901abc".search_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
let v: Vec<_> = "abc345abc901abc".rsearch_indices("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
```

### Example trait: SearchIn

```rust
use naive_opt::SearchIn;

let haystack = "111 a 111b";
let needle = "a";
let r = needle.search_in(haystack);
assert_eq!(r, Some(4));

assert_eq!("1".search_in(haystack), Some(0));
assert_eq!("1".rsearch_in(haystack), Some(8));
```

### Example Ignore ascii case match

```rust
use naive_opt::Search;

let haystack = "111 a 111b";
let needle = "A";
let r = haystack.search_ignore_ascii_case(needle);
assert_eq!(r, Some(4));
assert_eq!(haystack.rsearch_ignore_ascii_case("A"), Some(4));

let v: Vec<_> = "abc345aBc901abc".search_indices_ignore_ascii_case("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "aBc"), (12, "abc")]);
let v: Vec<_> = "abc345aBc901abc".rsearch_indices_ignore_ascii_case("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "aBc"), (0, "abc")]);

assert_eq!("<A HREF=http://".includes_ignore_ascii_case("href"), true);
```

# Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  757.800 μc |  521.920 μc |  728.020 μc |  520.580 μc |
| std_string_string       |  760.410 μc |  525.830 μc |  733.070 μc |  536.770 μc |
| func_str_str            |  102.100 μc |  121.790 μc |  122.460 μc |  122.300 μc |
| func_string_string      |  101.720 μc |  123.290 μc |  102.760 μc |  121.960 μc |
| trait_str_str           |   98.238 μc |  120.290 μc |  116.560 μc |  117.960 μc |
| trait_string_string     |   98.459 μc |  120.490 μc |   98.106 μc |  118.940 μc |
| std_indices             |  470.700 μc |  370.070 μc |  468.480 μc |  392.680 μc |
| func_indices            |  100.840 μc |  118.750 μc |  101.620 μc |  146.220 μc |
| trait_indices           |  100.920 μc |  118.810 μc |  101.070 μc |  145.120 μc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/naive_opt/blob/main/CHANGELOG.md)

# References

- [my research: string searching algorithm](https://github.com/aki-akaguma/cmp_string_searching_algorithm)
- [my research: string find](https://github.com/aki-akaguma/cmp_string_find)
- [wikipedia: string searching algprithm](https://en.wikipedia.org/wiki/String-searching_algorithm)
- [`memx`](https://crates.io/crates/memx) - rust crate for the fast mem lib

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/naive_opt.svg
[crate-link]: https://crates.io/crates/naive_opt
[docs-image]: https://docs.rs/naive_opt/badge.svg
[docs-link]: https://docs.rs/naive_opt/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/naive_opt/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/naive_opt/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/naive_opt/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/naive_opt/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/naive_opt/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/naive_opt/actions/workflows/test-windows.yml
