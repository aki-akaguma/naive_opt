# naive_opt
The optimized naive string-search algorithm.

## Features

* The naive string-searching algorithm
* Enhanced with 1-byte search like the libc++ and the libstd++ string::find
* Specializing in UTF-8 strings, which is a feature of rust
* Support the zero overhead trait.
* minimum support: rustc 1.41.1 (f3e1a954d 2020-02-24)

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

## Benchmark

- compile by rustc 1.52.0 (88f19c6da 2021-05-03)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  228.500 uc |  177.770 uc |  232.990 uc |  176.730 uc |
| std_string_string       |  223.760 uc |  178.780 uc |  228.410 uc |  175.880 uc |
| func_str_str            |   29.561 uc |   29.222 uc |   30.636 uc |   30.412 uc |
| func_string_string      |   29.315 uc |   29.102 uc |   30.422 uc |   30.371 uc |
| trait_str_str           |   25.365 uc |   25.036 uc |   26.210 uc |   26.271 uc |
| trait_string_string     |   25.131 uc |   24.814 uc |   26.350 uc |   26.443 uc |
| std_indices             |  173.250 uc |  126.650 uc |  169.970 uc |  131.070 uc |
| func_indices            |   26.060 uc |   26.094 uc |   27.181 uc |   27.331 uc |
| trait_indices           |   26.083 uc |   26.097 uc |   27.223 uc |   27.366 uc |

- compile by rustc 1.41.1 (f3e1a954d 2020-02-24)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  207.280 uc |  172.180 uc |  208.830 uc |  164.940 uc |
| std_string_string       |  207.220 uc |  174.360 uc |  205.340 uc |  167.410 uc |
| func_str_str            |   28.212 uc |   28.243 uc |   29.534 uc |   29.582 uc |
| func_string_string      |   28.173 uc |   28.329 uc |   29.939 uc |   29.576 uc |
| trait_str_str           |   24.888 uc |   24.896 uc |   26.374 uc |   26.272 uc |
| trait_string_string     |   25.642 uc |   25.736 uc |   29.353 uc |   27.134 uc |
| std_indices             |  170.070 uc |  133.240 uc |  168.450 uc |  124.060 uc |
| func_indices            |   26.352 uc |   26.372 uc |   27.931 uc |   27.951 uc |
| trait_indices           |   26.491 uc |   26.515 uc |   28.357 uc |   28.468 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz


## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/naive_opt/blob/main/CHANGELOG.md)


## References

- [my research: string searching algorithm](https://github.com/aki-akaguma/cmp_string_searching_algorithm)
- [my research: string find](https://github.com/aki-akaguma/cmp_string_find)
- [wikipedia: string searching algprithm](https://en.wikipedia.org/wiki/String-searching_algorithm)
