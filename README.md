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
| std_str_str             |  541.680 uc |  520.740 uc |  555.300 uc |  511.630 uc |
| std_string_string       |  540.130 uc |  517.280 uc |  556.200 uc |  508.900 uc |
| func_str_str            |  121.840 uc |  130.300 uc |  113.910 uc |  113.900 uc |
| func_string_string      |  121.000 uc |  129.540 uc |  113.740 uc |  109.830 uc |
| trait_str_str           |  118.850 uc |  123.410 uc |  111.720 uc |  105.280 uc |
| trait_string_string     |  120.660 uc |  118.760 uc |  112.090 uc |  108.290 uc |
| std_indices             |  466.430 uc |  397.850 uc |  475.890 uc |  398.430 uc |
| func_indices            |  115.710 uc |  119.230 uc |  113.610 uc |  109.420 uc |
| trait_indices           |  118.760 uc |  118.760 uc |  114.140 uc |  109.550 uc |

- compile by rustc 1.41.1 (f3e1a954d 2020-02-24)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  513.550 uc |  475.900 uc |  493.000 uc |  474.140 uc |
| std_string_string       |  509.330 uc |  472.820 uc |  493.370 uc |  479.790 uc |
| func_str_str            |  132.100 uc |  131.870 uc |  106.010 uc |  109.170 uc |
| func_string_string      |  128.180 uc |  131.710 uc |  111.030 uc |  105.800 uc |
| trait_str_str           |  128.020 uc |  124.220 uc |  104.610 uc |  105.830 uc |
| trait_string_string     |  123.580 uc |  125.900 uc |  103.350 uc |  110.630 uc |
| std_indices             |  448.560 uc |  407.380 uc |  427.750 uc |  415.310 uc |
| func_indices            |  121.280 uc |  121.210 uc |  112.230 uc |  112.480 uc |
| trait_indices           |  123.160 uc |  121.350 uc |  113.720 uc |  111.870 uc |

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
