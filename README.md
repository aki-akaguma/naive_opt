# naive_opt

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
| std_str_str             |  724.360 μc |  658.760 μc |  673.970 μc |  497.570 μc |
| std_string_string       |  728.580 μc |  663.240 μc |  676.770 μc |  499.840 μc |
| func_str_str            |   91.782 μc |  115.070 μc |   93.113 μc |  112.500 μc |
| func_string_string      |   91.752 μc |  115.780 μc |   91.619 μc |  112.340 μc |
| trait_str_str           |   87.110 μc |  108.330 μc |   87.466 μc |  105.960 μc |
| trait_string_string     |   87.201 μc |  107.620 μc |   87.738 μc |  105.900 μc |
| std_indices             |  456.860 μc |  366.500 μc |  454.770 μc |  361.730 μc |
| func_indices            |   95.400 μc |  117.080 μc |   96.759 μc |  111.610 μc |
| trait_indices           |   95.474 μc |  117.030 μc |   96.715 μc |  111.620 μc |

- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  597.500 uc |  486.830 uc |  612.780 uc |  494.020 uc |
| std_string_string       |  596.120 uc |  484.470 uc |  621.090 uc |  521.630 uc |
| func_str_str            |   92.700 uc |  111.850 uc |   91.712 uc |  113.740 uc |
| func_string_string      |   92.046 uc |  111.630 uc |   91.629 uc |  114.720 uc |
| trait_str_str           |   86.913 uc |  107.620 uc |   86.574 uc |  108.830 uc |
| trait_string_string     |   86.268 uc |  107.420 uc |   87.603 uc |  107.440 uc |
| std_indices             |  537.580 uc |  403.150 uc |  530.250 uc |  405.990 uc |
| func_indices            |   87.310 uc |  108.470 uc |   87.587 uc |  109.770 uc |
| trait_indices           |   87.383 uc |  107.750 uc |   87.895 uc |  109.070 uc |

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
