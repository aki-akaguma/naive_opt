# naive_opt
The optimized naive string-search algorithm.

* Enhanced with 1-byte search like the libc++ and the libstd++ string::find
* Specializing in UTF-8 strings, which is a feature of rust
* Support the zero overhead trait.

## Compatibility

This crate is implemented to replace the rust std library.
However, the method names are different, so please rewrite your code.
It shouldn't be too difficult.

compatibility:
|:---------------------|:------------------------------|
| rust std::str        | this crate                    |
|:---------------------|:------------------------------|
| std::str::find()     | naive_opt::Search::search()   |
| std::str::contains() | naive_opt::Search::includes() |

## Examples

### Example function:

```rust
use naive_opt::string_search;
let haystack = "111 a 111b";
let needle = "a";
let r = string_search(haystack, needle);
assert_eq!(r, Some(4));
```

### Example trait 1:

```rust
use naive_opt::Search;
let haystack = "111 a 111b";
let needle = "a";
let r = haystack.search(needle);
assert_eq!(r, Some(4));
```

### Example trait 2:

```rust
use naive_opt::SearchIn;
let haystack = "111 a 111b";
let needle = "a";
let r = needle.search_in(haystack);
assert_eq!(r, Some(4));
```

## Benchmark

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  445.560 uc |  355.440 uc |  458.540 uc |  359.400 uc |
| std_string_string       |  450.900 uc |  355.110 uc |  453.940 uc |  354.490 uc |
| func_str_str            |   57.928 uc |   56.888 uc |   59.602 uc |   59.365 uc |
| func_string_string      |   56.754 uc |   56.721 uc |   59.276 uc |   59.016 uc |
| trait_str_str           |   50.825 uc |   49.719 uc |   52.659 uc |   52.919 uc |
| trait_string_string     |   49.898 uc |   49.951 uc |   52.263 uc |   52.721 uc |
| std_indices             |  349.200 uc |  269.560 uc |  373.310 uc |  267.570 uc |
| func_indices            |   51.955 uc |   52.269 uc |   55.074 uc |   54.534 uc |
| trait_indices           |   52.235 uc |   52.201 uc |   54.549 uc |   54.589 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- compile by rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz

## Todos

- [ ] rsearch, reverse search

## Changelogs

https://github.com/aki-akaguma/naive_opt/blob/main/CHANGELOG.md
