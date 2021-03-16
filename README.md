# naive_opt
The optimized naive string-search algorithm.

* You can write in 3 ways.

```rust
let haystack = "apple orange strawberry"
let needle = "ge"
assert_eq!(haystack.search(needle), Some(10));
```

```rust
let haystack = "apple orange strawberry"
let needle = "ge"
assert_eq!(needle.search_in(haystack), Some(10));
```

```rust
let haystack = "apple orange strawberry"
let needle = "ge"
assert_eq!(string_searching(haystack, needle), Some(10));
```

* support the zero overhead trait.

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| func_str_str            |   42.812 uc |   42.656 uc |   47.694 uc |   47.690 uc |
| func_string_string      |   42.802 uc |   42.794 uc |   47.217 uc |   46.972 uc |
| trait_str_str           |   42.744 uc |   42.720 uc |   47.671 uc |   47.677 uc |
| trait_string_string     |   42.798 uc |   42.824 uc |   47.165 uc |   46.946 uc |
