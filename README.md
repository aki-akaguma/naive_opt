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
| std_str_str             |  314.190 uc |  280.340 uc |  319.940 uc |  276.370 uc |
| std_string_string       |  315.330 uc |  282.610 uc |  326.080 uc |  275.660 uc |
| func_str_str            |   41.707 uc |   41.724 uc |   45.775 uc |   45.949 uc |
| func_string_string      |   41.828 uc |   41.826 uc |   45.820 uc |   45.748 uc |
| trait_str_str           |   41.709 uc |   41.737 uc |   45.795 uc |   45.934 uc |
| trait_string_string     |   41.779 uc |   41.860 uc |   45.790 uc |   45.776 uc |
| std_indices             |  341.490 uc |  258.910 uc |  340.680 uc |  256.040 uc |
| func_indices            |   65.796 uc |   65.853 uc |   68.599 uc |   68.416 uc |
| trait_indices           |   66.602 uc |   66.702 uc |   68.598 uc |   68.667 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- compile by rustc 1.50.0 (cb75ad5db 2021-02-10)
- bench on intel Q6600 @ 2.40GHz
