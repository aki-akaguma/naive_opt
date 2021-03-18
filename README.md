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
| std_str_str             |  445.100 uc |  351.770 uc |  453.920 uc |  374.950 uc |
| std_string_string       |  438.650 uc |  353.290 uc |  456.160 uc |  376.580 uc |
| func_str_str            |   57.182 uc |   56.293 uc |   59.289 uc |   59.012 uc |
| func_string_string      |   56.079 uc |   56.269 uc |   58.694 uc |   58.678 uc |
| trait_str_str           |   50.418 uc |   49.367 uc |   52.200 uc |   52.112 uc |
| trait_string_string     |   49.448 uc |   49.501 uc |   52.080 uc |   52.057 uc |
| std_indices             |  338.080 uc |  260.590 uc |  357.770 uc |  266.960 uc |
| func_indices            |   55.877 uc |   55.891 uc |   58.164 uc |   58.097 uc |
| trait_indices           |   51.534 uc |   51.547 uc |   54.863 uc |   54.808 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- compile by rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz
