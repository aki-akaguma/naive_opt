# {{crate}}

{{readme}}

# Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  748.440 μc |  520.660 μc |  753.480 μc |  508.910 μc |
| std_string_string       |  756.080 μc |  520.690 μc |  757.250 μc |  508.680 μc |
| func_str_str            |   92.168 μc |  111.290 μc |  110.980 μc |  110.710 μc |
| func_string_string      |   92.432 μc |  112.260 μc |   93.275 μc |  110.790 μc |
| trait_str_str           |   87.074 μc |  107.730 μc |  107.540 μc |  106.330 μc |
| trait_string_string     |   87.373 μc |  107.170 μc |   88.310 μc |  106.250 μc |
| std_indices             |  470.500 μc |  371.010 μc |  468.910 μc |  374.780 μc |
| func_indices            |   92.939 μc |  108.660 μc |   92.794 μc |  124.560 μc |
| trait_indices           |   93.142 μc |  108.450 μc |   92.940 μc |  122.630 μc |

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
