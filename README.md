# naive_opt
The optimized naive string-search algorithm.

* Enhanced with 1-byte search like the libc++ and the libstd++ string::find
* Specializing in UTF-8 strings, which is a feature of rust
* Support the zero overhead trait.

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
| std_str_str             |  444.140 uc |  350.680 uc |  452.730 uc |  351.780 uc |
| std_string_string       |  442.580 uc |  356.640 uc |  450.770 uc |  355.350 uc |
| func_str_str            |   57.334 uc |   56.436 uc |   58.972 uc |   59.021 uc |
| func_string_string      |   56.362 uc |   56.381 uc |   58.813 uc |   58.735 uc |
| trait_str_str           |   57.250 uc |   56.488 uc |   59.303 uc |   59.414 uc |
| trait_string_string     |   56.369 uc |   56.472 uc |   58.844 uc |   58.882 uc |
| std_indices             |  348.210 uc |  268.740 uc |  347.280 uc |  265.080 uc |
| func_indices            |   66.154 uc |   66.333 uc |   68.518 uc |   68.825 uc |
| trait_indices           |   66.030 uc |   66.011 uc |   68.792 uc |   68.714 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- compile by rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz
