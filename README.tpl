# {{crate}}

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

{{readme}}

# Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_str_str             |  757.800 μc |  521.920 μc |  728.020 μc |  520.580 μc |
| std_string_string       |  760.410 μc |  525.830 μc |  733.070 μc |  536.770 μc |
| func_str_str            |  102.100 μc |  121.790 μc |  122.460 μc |  122.300 μc |
| func_string_string      |  101.720 μc |  123.290 μc |  102.760 μc |  121.960 μc |
| trait_str_str           |   98.238 μc |  120.290 μc |  116.560 μc |  117.960 μc |
| trait_string_string     |   98.459 μc |  120.490 μc |   98.106 μc |  118.940 μc |
| std_indices             |  470.700 μc |  370.070 μc |  468.480 μc |  392.680 μc |
| func_indices            |  100.840 μc |  118.750 μc |  101.620 μc |  146.220 μc |
| trait_indices           |  100.920 μc |  118.810 μc |  101.070 μc |  145.120 μc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/{{crate}}/blob/main/CHANGELOG.md)

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

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/{{crate}}.svg
[crate-link]: https://crates.io/crates/{{crate}}
[docs-image]: https://docs.rs/{{crate}}/badge.svg
[docs-link]: https://docs.rs/{{crate}}/
[rustc-image]: https://img.shields.io/badge/rustc-1.65+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml
