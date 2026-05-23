# Code Review: naive_opt

## Overview
`naive_opt` is a Rust crate providing an optimized version of the naive string-searching algorithm. It achieves high performance by leveraging SIMD-accelerated memory operations (via the `memx` crate) and a probabilistic approach ("ASCII Stochastics") to identify the best starting point for the search.

## Strengths
- **Performance-Oriented**: The use of `memx` for `memchr` and `memeq` ensures that the core operations are highly efficient.
- **Probabilistic Optimization**: The "ASCII Stochastics" table is a clever way to minimize false positives during the initial byte search phase by choosing the least frequent byte of the needle as the search target.
- **API Ergonomics**: The implementation of `Search` and `SearchIn` traits provides a familiar and powerful API that mirrors `std::str::find` while offering additional features like byte-based search and case-insensitive matching.
- **Comprehensive Testing**: The project includes extensive tests that verify compatibility with standard library expectations and cover various edge cases, including empty needles and large haystacks.
- **Well-documented**: The crate includes clear examples and a compatibility table in the documentation.

## Areas for Improvement

### 1. Code Duplication
The implementation in `src/mc_1st.rs` and `src/mc_last.rs` is largely redundant. Most functions differ only in whether they look at `nee_bytes[0]` or `nee_bytes[nee_bytes.len() - 1]`.
*   **Recommendation**: Consider refactoring these into a single parameterized implementation or using a macro to generate both versions. This would reduce the maintenance burden and the risk of logic drifting between the two strategies.

### 2. Inlining Strategy
Extensive use of `#[inline(always)]` is present across the codebase. While beneficial for small, hot functions, it can lead to unnecessary binary bloat if overused on larger functions.
*   **Recommendation**: Evaluate if `#[inline]` (letting the compiler decide) is sufficient for some of the larger functions, keeping `#[inline(always)]` only for the most critical, small wrappers.

### 3. ASCII Stochastics Table
The `_ASCII_STOCHAS` table is hardcoded with specific weights.
*   **Inquiry**: It would be beneficial to document how these weights were derived (e.g., from a specific corpus of text).
*   **Observation**: Currently, non-ASCII characters default to `mc_last`. While reasonable, a more generalized approach could further improve performance for non-English text.

### 4. Specialization for `char`
The `SearchIn` implementation for `char` converts the character to a `String` (and then to bytes) on every call, causing a heap allocation.
```rust
impl<'a> SearchIn<'a> for char {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_bytes(haystack.as_bytes(), self.to_string().as_bytes())
    }
    // ...
}
```
*   **Recommendation**: Use `char::encode_utf8` with a local stack-allocated buffer to avoid heap allocation.
```rust
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        let mut buf = [0u8; 4];
        let nee_str = self.encode_utf8(&mut buf);
        naive_opt_mc_bytes(haystack.as_bytes(), nee_str.as_bytes())
    }
```

### 5. Dependency on `memx`
The crate relies heavily on `memx`.
*   **Observation**: While `memx` is efficient, ensure its versioning and maintenance align with the stability goals of `naive_opt`. The current version `0.2` suggests it is still evolving.

## Conclusion
`naive_opt` is a high-quality implementation of an optimized string search algorithm. It is well-structured, thoroughly tested, and provides significant performance benefits over a standard naive search. Addressing the minor code duplication and heap allocation in the `char` implementation would further refine the crate's excellence.

---
Review Date: 2026-05-23
Reviewer: Gemini CLI Agent
