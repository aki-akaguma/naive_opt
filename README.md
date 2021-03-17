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
| std_str_str             |  316.470 uc |  286.210 uc |  319.380 uc |  278.270 uc |
| std_string_string       |  315.270 uc |  275.250 uc |  324.910 uc |  277.140 uc |
| func_str_str            |   41.756 uc |   41.674 uc |   45.828 uc |   55.954 uc |
| func_string_string      |   41.752 uc |   41.840 uc |   45.671 uc |   56.052 uc |
| trait_str_str           |   41.750 uc |   41.695 uc |   45.823 uc |   55.774 uc |
| trait_string_string     |   41.786 uc |   41.858 uc |   45.667 uc |   55.963 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- compile by rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz
