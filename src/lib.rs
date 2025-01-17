/*!
The optimized naive string-search algorithm.

# Features

- The naive string-searching algorithm
- Enhanced with 1-byte search like the libc++ and the libstd++ string::find
- Specializing in UTF-8 strings, which is a feature of rust
- The ASCII Stochastics search
- Support the zero overhead trait.
- Support ignore ascii case match.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

# Compatibility

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

# Ignore ascii case match

This crate supports an ASCII case-insensitive match with each function.

| this crate                                               |
|:---------------------------------------------------------|
| `naive_opt::Search::search_ignore_ascii_case()`          |
| `naive_opt::Search::rsearch_ignore_ascii_case()`         |
| `naive_opt::Search::includes_ignore_ascii_case()`        |
| `naive_opt::Search::search_indices_ignore_ascii_case()`  |
| `naive_opt::Search::rsearch_indices_ignore_ascii_case()` |

# Examples

## Example function:

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

## Example trait: Search

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

## Example trait: SearchIn

```rust
use naive_opt::SearchIn;

let haystack = "111 a 111b";
let needle = "a";
let r = needle.search_in(haystack);
assert_eq!(r, Some(4));

assert_eq!("1".search_in(haystack), Some(0));
assert_eq!("1".rsearch_in(haystack), Some(8));
```

## Example Ignore ascii case match

```rust
use naive_opt::Search;

let haystack = "111 a 111b";
let needle = "A";
let r = haystack.search_ignore_ascii_case(needle);
assert_eq!(r, Some(4));
assert_eq!(haystack.rsearch_ignore_ascii_case("A"), Some(4));

let v: Vec<_> = "abc345aBc901abc".search_indices_ignore_ascii_case("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "aBc"), (12, "abc")]);
let v: Vec<_> = "abc345aBc901abc".rsearch_indices_ignore_ascii_case("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "aBc"), (0, "abc")]);

assert_eq!("<A HREF=http://".includes_ignore_ascii_case("href"), true);
```
*/

///
/// search the needle
///
pub trait Search {
    ///
    /// search the needle in self.
    ///
    /// return index of self, if it found the needle. Otherwise return None.
    ///
    fn search<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize>;
    ///
    /// reverse search the needle in self.
    ///
    /// return index of self, if it found the needle. Otherwise return None.
    ///
    fn rsearch<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize>;
    ///
    /// An iterator over the matches of needle in self.
    ///
    /// Examples
    ///
    /// ```rust
    /// use naive_opt::Search;
    ///
    /// let v: Vec<_> = "abc345abc901abc".search_indices("abc").collect();
    /// assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
    ///
    /// let v: Vec<_> = "0abcabc7".search_indices("abc").collect();
    /// assert_eq!(v, [(1, "abc"), (4, "abc")]);
    ///
    /// let v: Vec<_> = "ababa".search_indices("aba").collect();
    /// assert_eq!(v, [(0, "aba")]); // only the first `aba`
    /// ```
    ///
    fn search_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> SearchIndices<'a, P>;
    ///
    /// An reverse search iterator over the matches of needle in self.
    ///
    /// Examples
    ///
    /// ```rust
    /// use naive_opt::Search;
    ///
    /// let v: Vec<_> = "abc345abc901abc".rsearch_indices("abc").collect();
    /// assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
    ///
    /// let v: Vec<_> = "0abcabc7".rsearch_indices("abc").collect();
    /// assert_eq!(v, [(4, "abc"), (1, "abc")]);
    ///
    /// let v: Vec<_> = "ababa".rsearch_indices("aba").collect();
    /// assert_eq!(v, [(2, "aba")]); // only the last `aba`
    /// ```
    ///
    fn rsearch_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> RevSearchIndices<'a, P>;
    ///
    /// includes the needle in self.
    ///
    /// returns true if the given pattern matches a sub-slice of this string slice.
    /// returns false if it does not.
    ///
    fn includes<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool;
    ///
    /// search the needle in self, ignore ascii case.
    ///
    /// return index of self, if it found the needle. Otherwise return None.
    ///
    fn search_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize>;
    ///
    /// reverse search the needle in self, ignore ascii case.
    ///
    /// return index of self, if it found the needle. Otherwise return None.
    ///
    fn rsearch_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize>;
    ///
    /// An iterator over the matches of needle in self, ignore ascii case.
    ///
    fn search_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
        &'a self,
        needle: P,
    ) -> SearchIndicesIgnoreAsciiCase<'a, P>;
    ///
    /// An reverse search iterator over the matches of needle in self, ignore ascii case.
    ///
    fn rsearch_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
        &'a self,
        needle: P,
    ) -> RevSearchIndicesIgnoreAsciiCase<'a, P>;
    ///
    /// includes the needle in self, ignore ascii case.
    ///
    fn includes_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool;
}
impl Search for &str {
    #[inline]
    fn search<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in(self)
    }
    #[inline]
    fn rsearch<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.rsearch_in(self)
    }
    #[inline]
    fn search_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> SearchIndices<'a, P> {
        SearchIndices::new(self, needle)
    }
    #[inline]
    fn rsearch_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> RevSearchIndices<'a, P> {
        RevSearchIndices::new(self, needle)
    }
    #[inline]
    fn includes<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in(self)
    }
    //
    #[inline]
    fn search_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in_ignore_ascii_case(self)
    }
    #[inline]
    fn rsearch_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.rsearch_in_ignore_ascii_case(self)
    }
    #[inline]
    fn search_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
        &'a self,
        needle: P,
    ) -> SearchIndicesIgnoreAsciiCase<'a, P> {
        SearchIndicesIgnoreAsciiCase::new(self, needle)
    }
    #[inline]
    fn rsearch_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
        &'a self,
        needle: P,
    ) -> RevSearchIndicesIgnoreAsciiCase<'a, P> {
        RevSearchIndicesIgnoreAsciiCase::new(self, needle)
    }
    #[inline]
    fn includes_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in_ignore_ascii_case(self)
    }
}
impl Search for String {
    #[inline]
    fn search<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in(self.as_str())
    }
    #[inline]
    fn rsearch<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.rsearch_in(self.as_str())
    }
    #[inline]
    fn search_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> SearchIndices<'a, P> {
        SearchIndices::new(self.as_str(), needle)
    }
    #[inline]
    fn rsearch_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> RevSearchIndices<'a, P> {
        RevSearchIndices::new(self.as_str(), needle)
    }
    #[inline]
    fn includes<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in(self)
    }
    //
    #[inline]
    fn search_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in_ignore_ascii_case(self.as_str())
    }
    #[inline]
    fn rsearch_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.rsearch_in_ignore_ascii_case(self.as_str())
    }
    #[inline]
    fn search_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
        &'a self,
        needle: P,
    ) -> SearchIndicesIgnoreAsciiCase<'a, P> {
        SearchIndicesIgnoreAsciiCase::new(self.as_str(), needle)
    }
    #[inline]
    fn rsearch_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
        &'a self,
        needle: P,
    ) -> RevSearchIndicesIgnoreAsciiCase<'a, P> {
        RevSearchIndicesIgnoreAsciiCase::new(self.as_str(), needle)
    }
    #[inline]
    fn includes_ignore_ascii_case<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in_ignore_ascii_case(self)
    }
}

pub trait SearchBytes {
    fn search_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize>;
    fn rsearch_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize>;
    fn search_indices_bytes<'a, P>(&'a self, needle: P) -> SearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>;
    fn rsearch_indices_bytes<'a, P>(&'a self, needle: P) -> RevSearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>;
    fn includes_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool;
    //
    fn search_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize>;
    fn rsearch_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize>;
    fn search_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> SearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>;
    fn rsearch_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> RevSearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>;
    fn includes_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool;
}
impl SearchBytes for &[u8] {
    #[inline]
    fn search_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in(self)
    }
    #[inline]
    fn rsearch_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.rsearch_in(self)
    }
    #[inline]
    fn search_indices_bytes<'a, P>(&'a self, needle: P) -> SearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        SearchIndicesBytes::new(self, needle)
    }
    #[inline]
    fn rsearch_indices_bytes<'a, P>(&'a self, needle: P) -> RevSearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        RevSearchIndicesBytes::new(self, needle)
    }
    #[inline]
    fn includes_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in(self)
    }
    //
    #[inline]
    fn search_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize> {
        needle.search_in_ignore_ascii_case(self)
    }
    #[inline]
    fn rsearch_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize> {
        needle.rsearch_in_ignore_ascii_case(self)
    }
    #[inline]
    fn search_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> SearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        SearchIndicesBytesIgnoreAsciiCase::new(self, needle)
    }
    #[inline]
    fn rsearch_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> RevSearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        RevSearchIndicesBytesIgnoreAsciiCase::new(self, needle)
    }
    #[inline]
    fn includes_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in_ignore_ascii_case(self)
    }
}
impl SearchBytes for &str {
    #[inline]
    fn search_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in(self.as_bytes())
    }
    #[inline]
    fn rsearch_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.rsearch_in(self.as_bytes())
    }
    #[inline]
    fn search_indices_bytes<'a, P>(&'a self, needle: P) -> SearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        SearchIndicesBytes::new(self.as_bytes(), needle)
    }
    #[inline]
    fn rsearch_indices_bytes<'a, P>(&'a self, needle: P) -> RevSearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        RevSearchIndicesBytes::new(self.as_bytes(), needle)
    }
    #[inline]
    fn includes_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in(self.as_bytes())
    }
    //
    #[inline]
    fn search_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize> {
        needle.search_in_ignore_ascii_case(self.as_bytes())
    }
    #[inline]
    fn rsearch_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize> {
        needle.rsearch_in_ignore_ascii_case(self.as_bytes())
    }
    #[inline]
    fn search_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> SearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        SearchIndicesBytesIgnoreAsciiCase::new(self.as_bytes(), needle)
    }
    #[inline]
    fn rsearch_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> RevSearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        RevSearchIndicesBytesIgnoreAsciiCase::new(self.as_bytes(), needle)
    }
    #[inline]
    fn includes_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in_ignore_ascii_case(self.as_bytes())
    }
}
impl SearchBytes for String {
    #[inline]
    fn search_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in(self.as_bytes())
    }
    #[inline]
    fn rsearch_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.rsearch_in(self.as_bytes())
    }
    #[inline]
    fn search_indices_bytes<'a, P>(&'a self, needle: P) -> SearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        SearchIndicesBytes::new(self.as_bytes(), needle)
    }
    #[inline]
    fn rsearch_indices_bytes<'a, P>(&'a self, needle: P) -> RevSearchIndicesBytes<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        RevSearchIndicesBytes::new(self.as_bytes(), needle)
    }
    #[inline]
    fn includes_bytes<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in(self.as_bytes())
    }
    //
    #[inline]
    fn search_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize> {
        needle.search_in_ignore_ascii_case(self.as_bytes())
    }
    #[inline]
    fn rsearch_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
        &'a self,
        needle: P,
    ) -> Option<usize> {
        needle.rsearch_in_ignore_ascii_case(self.as_bytes())
    }
    #[inline]
    fn search_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> SearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        SearchIndicesBytesIgnoreAsciiCase::new(self.as_bytes(), needle)
    }
    #[inline]
    fn rsearch_indices_bytes_ignore_ascii_case<'a, P>(
        &'a self,
        needle: P,
    ) -> RevSearchIndicesBytesIgnoreAsciiCase<'a, P>
    where
        P: SearchInBytes<'a>,
    {
        RevSearchIndicesBytesIgnoreAsciiCase::new(self.as_bytes(), needle)
    }
    #[inline]
    fn includes_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in_ignore_ascii_case(self.as_bytes())
    }
}

///
/// Created with the method [Search::search_indices()].
///
pub struct SearchIndices<'a, P: SearchIn<'a>> {
    curr_idx: usize,
    haystack: &'a str,
    needle: P,
}
impl<'a, P: SearchIn<'a>> SearchIndices<'a, P> {
    fn new(a_haystack: &'a str, a_needle: P) -> SearchIndices<'a, P> {
        SearchIndices {
            curr_idx: 0,
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchIn<'a>> Iterator for SearchIndices<'a, P> {
    type Item = (usize, &'a str);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.needle.search_in(&self.haystack[self.curr_idx..]) {
            Some(idx) => {
                self.curr_idx += idx;
                let st = self.curr_idx;
                let ed = st + self.needle.len();
                self.curr_idx = ed;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

pub struct SearchIndicesBytes<'a, P: SearchInBytes<'a>> {
    curr_idx: usize,
    haystack: &'a [u8],
    needle: P,
}
impl<'a, P: SearchInBytes<'a>> SearchIndicesBytes<'a, P> {
    fn new(a_haystack: &'a [u8], a_needle: P) -> SearchIndicesBytes<'a, P> {
        SearchIndicesBytes {
            curr_idx: 0,
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchInBytes<'a>> Iterator for SearchIndicesBytes<'a, P> {
    type Item = (usize, &'a [u8]);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.needle.search_in(&self.haystack[self.curr_idx..]) {
            Some(idx) => {
                self.curr_idx += idx;
                let st = self.curr_idx;
                let ed = st + self.needle.len();
                self.curr_idx = ed;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

///
/// Created with the method [Search::rsearch_indices()].
///
pub struct RevSearchIndices<'a, P: SearchIn<'a>> {
    curr_ed: usize,
    haystack: &'a str,
    needle: P,
}
impl<'a, P: SearchIn<'a>> RevSearchIndices<'a, P> {
    fn new(a_haystack: &'a str, a_needle: P) -> RevSearchIndices<'a, P> {
        RevSearchIndices {
            curr_ed: a_haystack.len(),
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchIn<'a>> Iterator for RevSearchIndices<'a, P> {
    type Item = (usize, &'a str);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.needle.rsearch_in(&self.haystack[0..self.curr_ed]) {
            Some(idx) => {
                let st = idx;
                let ed = st + self.needle.len();
                self.curr_ed = st;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

pub struct RevSearchIndicesBytes<'a, P: SearchInBytes<'a>> {
    curr_ed: usize,
    haystack: &'a [u8],
    needle: P,
}
impl<'a, P: SearchInBytes<'a>> RevSearchIndicesBytes<'a, P> {
    fn new(a_haystack: &'a [u8], a_needle: P) -> RevSearchIndicesBytes<'a, P> {
        RevSearchIndicesBytes {
            curr_ed: a_haystack.len(),
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchInBytes<'a>> Iterator for RevSearchIndicesBytes<'a, P> {
    type Item = (usize, &'a [u8]);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.needle.rsearch_in(&self.haystack[0..self.curr_ed]) {
            Some(idx) => {
                let st = idx;
                let ed = st + self.needle.len();
                self.curr_ed = st;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

///
/// Created with the method [Search::search_indices_ignore_ascii_case()].
///
pub struct SearchIndicesIgnoreAsciiCase<'a, P: SearchIn<'a>> {
    curr_idx: usize,
    haystack: &'a str,
    needle: P,
}
impl<'a, P: SearchIn<'a>> SearchIndicesIgnoreAsciiCase<'a, P> {
    fn new(a_haystack: &'a str, a_needle: P) -> SearchIndicesIgnoreAsciiCase<'a, P> {
        SearchIndicesIgnoreAsciiCase {
            curr_idx: 0,
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchIn<'a>> Iterator for SearchIndicesIgnoreAsciiCase<'a, P> {
    type Item = (usize, &'a str);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self
            .needle
            .search_in_ignore_ascii_case(&self.haystack[self.curr_idx..])
        {
            Some(idx) => {
                self.curr_idx += idx;
                let st = self.curr_idx;
                let ed = st + self.needle.len();
                self.curr_idx = ed;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

pub struct SearchIndicesBytesIgnoreAsciiCase<'a, P: SearchInBytes<'a>> {
    curr_idx: usize,
    haystack: &'a [u8],
    needle: P,
}
impl<'a, P: SearchInBytes<'a>> SearchIndicesBytesIgnoreAsciiCase<'a, P> {
    fn new(a_haystack: &'a [u8], a_needle: P) -> SearchIndicesBytesIgnoreAsciiCase<'a, P> {
        SearchIndicesBytesIgnoreAsciiCase {
            curr_idx: 0,
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchInBytes<'a>> Iterator for SearchIndicesBytesIgnoreAsciiCase<'a, P> {
    type Item = (usize, &'a [u8]);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self
            .needle
            .search_in_ignore_ascii_case(&self.haystack[self.curr_idx..])
        {
            Some(idx) => {
                self.curr_idx += idx;
                let st = self.curr_idx;
                let ed = st + self.needle.len();
                self.curr_idx = ed;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

///
/// Created with the method [Search::rsearch_indices_ignore_ascii_case()].
///
pub struct RevSearchIndicesIgnoreAsciiCase<'a, P: SearchIn<'a>> {
    curr_ed: usize,
    haystack: &'a str,
    needle: P,
}
impl<'a, P: SearchIn<'a>> RevSearchIndicesIgnoreAsciiCase<'a, P> {
    fn new(a_haystack: &'a str, a_needle: P) -> RevSearchIndicesIgnoreAsciiCase<'a, P> {
        RevSearchIndicesIgnoreAsciiCase {
            curr_ed: a_haystack.len(),
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchIn<'a>> Iterator for RevSearchIndicesIgnoreAsciiCase<'a, P> {
    type Item = (usize, &'a str);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self
            .needle
            .rsearch_in_ignore_ascii_case(&self.haystack[0..self.curr_ed])
        {
            Some(idx) => {
                let st = idx;
                let ed = st + self.needle.len();
                self.curr_ed = st;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

pub struct RevSearchIndicesBytesIgnoreAsciiCase<'a, P: SearchInBytes<'a>> {
    curr_ed: usize,
    haystack: &'a [u8],
    needle: P,
}
impl<'a, P: SearchInBytes<'a>> RevSearchIndicesBytesIgnoreAsciiCase<'a, P> {
    fn new(a_haystack: &'a [u8], a_needle: P) -> RevSearchIndicesBytesIgnoreAsciiCase<'a, P> {
        RevSearchIndicesBytesIgnoreAsciiCase {
            curr_ed: a_haystack.len(),
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchInBytes<'a>> Iterator for RevSearchIndicesBytesIgnoreAsciiCase<'a, P> {
    type Item = (usize, &'a [u8]);
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self
            .needle
            .rsearch_in_ignore_ascii_case(&self.haystack[0..self.curr_ed])
        {
            Some(idx) => {
                let st = idx;
                let ed = st + self.needle.len();
                self.curr_ed = st;
                Some((st, &self.haystack[st..ed]))
            }
            None => None,
        }
    }
}

///
/// search in the haystack
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub trait SearchIn<'a>: Sized {
    ///
    /// search self in the haystack.
    ///
    /// return index of the haystack, if it found self. Otherwise return None.
    ///
    fn search_in(&self, haystack: &'a str) -> Option<usize>;
    ///
    /// reverse search self in the haystack.
    ///
    /// return index of the haystack, if it found self. Otherwise return None.
    ///
    fn rsearch_in(&self, haystack: &'a str) -> Option<usize>;
    ///
    /// search self in the haystack, ignore ascii case.
    ///
    /// return index of the haystack, if it found self. Otherwise return None.
    ///
    fn search_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize>;
    ///
    /// reverse search self in the haystack, ignore ascii case.
    ///
    /// return index of the haystack, if it found self. Otherwise return None.
    ///
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize>;
    ///
    /// return the length of self
    ///
    fn len(&self) -> usize;
    ///
    /// true if a length of 0.
    ///
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    ///
    /// includes self in the haystack.
    ///
    /// returns true if the given pattern matches a sub-slice of this string slice.
    /// returns false if it does not.
    ///
    #[inline]
    fn includes_in(&self, haystack: &'a str) -> bool {
        self.search_in(haystack).is_some()
    }
    ///
    /// includes self in the haystack, ignore ascii case.
    ///
    /// returns true if the given pattern matches a sub-slice of this string slice.
    /// returns false if it does not.
    ///
    #[inline]
    fn includes_in_ignore_ascii_case(&self, haystack: &'a str) -> bool {
        self.search_in_ignore_ascii_case(haystack).is_some()
    }
}
impl<'a> SearchIn<'a> for &str {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_bytes(haystack.as_bytes(), self.as_bytes())
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_rev_bytes(haystack.as_bytes(), self.as_bytes())
    }
    #[inline]
    fn search_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_bytes_iac(haystack.as_bytes(), self.as_bytes())
    }
    #[inline]
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_rev_bytes_iac(haystack.as_bytes(), self.as_bytes())
    }
    #[inline]
    fn len(&self) -> usize {
        #[allow(clippy::needless_as_bytes)]
        self.as_bytes().len()
    }
}
impl<'a> SearchIn<'a> for &String {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_bytes(haystack.as_bytes(), self.as_str().as_bytes())
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_rev_bytes(haystack.as_bytes(), self.as_str().as_bytes())
    }
    #[inline]
    fn search_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_bytes_iac(haystack.as_bytes(), self.as_str().as_bytes())
    }
    #[inline]
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_rev_bytes_iac(haystack.as_bytes(), self.as_str().as_bytes())
    }
    #[inline]
    fn len(&self) -> usize {
        self.as_str().len()
    }
}
impl<'a> SearchIn<'a> for char {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_bytes(haystack.as_bytes(), self.to_string().as_bytes())
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_rev_bytes(haystack.as_bytes(), self.to_string().as_bytes())
    }
    #[inline]
    fn search_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_bytes_iac(haystack.as_bytes(), self.to_string().as_bytes())
    }
    #[inline]
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_rev_bytes_iac(haystack.as_bytes(), self.to_string().as_bytes())
    }
    #[inline]
    fn len(&self) -> usize {
        self.to_string().len()
    }
}

pub trait SearchInBytes<'a>: Sized {
    fn search_in(&self, haystack: &'a [u8]) -> Option<usize>;
    fn rsearch_in(&self, haystack: &'a [u8]) -> Option<usize>;
    fn search_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize>;
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    fn includes_in(&self, haystack: &'a [u8]) -> bool {
        self.search_in(haystack).is_some()
    }
    #[inline]
    fn includes_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> bool {
        self.search_in_ignore_ascii_case(haystack).is_some()
    }
}
impl<'a> SearchInBytes<'a> for &[u8] {
    #[inline]
    fn search_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes(haystack, self)
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes(haystack, self)
    }
    #[inline]
    fn search_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes_iac(haystack, self)
    }
    #[inline]
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes_iac(haystack, self)
    }
    #[inline]
    fn len(&self) -> usize {
        u8_len(self)
    }
}
#[inline(always)]
fn u8_len(a: &[u8]) -> usize {
    a.len()
}
impl<'a> SearchInBytes<'a> for &str {
    #[inline]
    fn search_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes(haystack, self.as_bytes())
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes(haystack, self.as_bytes())
    }
    #[inline]
    fn search_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes_iac(haystack, self.as_bytes())
    }
    #[inline]
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes_iac(haystack, self.as_bytes())
    }
    #[inline]
    fn len(&self) -> usize {
        #[allow(clippy::needless_as_bytes)]
        self.as_bytes().len()
    }
}
impl<'a> SearchInBytes<'a> for &String {
    #[inline]
    fn search_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes(haystack, self.as_bytes())
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes(haystack, self.as_bytes())
    }
    #[inline]
    fn search_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes_iac(haystack, self.as_bytes())
    }
    #[inline]
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes_iac(haystack, self.as_bytes())
    }
    #[inline]
    fn len(&self) -> usize {
        self.as_str().len()
    }
}
impl<'a> SearchInBytes<'a> for char {
    #[inline]
    fn search_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes(haystack, self.to_string().as_bytes())
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes(haystack, self.to_string().as_bytes())
    }
    #[inline]
    fn search_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_bytes_iac(haystack, self.to_string().as_bytes())
    }
    #[inline]
    fn rsearch_in_ignore_ascii_case(&self, haystack: &'a [u8]) -> Option<usize> {
        naive_opt_mc_rev_bytes_iac(haystack, self.to_string().as_bytes())
    }
    #[inline]
    fn len(&self) -> usize {
        self.to_string().len()
    }
}

///
/// search the needle in the haystack
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_search(haystack: &str, needle: &str) -> Option<usize> {
    naive_opt_mc_bytes(haystack.as_bytes(), needle.as_bytes())
}

///
/// reverse search the needle in the haystack
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_rsearch(haystack: &str, needle: &str) -> Option<usize> {
    naive_opt_mc_rev_bytes(haystack.as_bytes(), needle.as_bytes())
}

///
/// search the needle in the haystack bytes
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_search_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    naive_opt_mc_bytes(haystack, needle)
}

///
/// reverse search the needle in the haystack bytes
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_rsearch_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    naive_opt_mc_rev_bytes(haystack, needle)
}

///
/// search the needle in the haystack, ignore ascii case.
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_search_ignore_ascii_case(haystack: &str, needle: &str) -> Option<usize> {
    naive_opt_mc_bytes_iac(haystack.as_bytes(), needle.as_bytes())
}

///
/// search the needle in the haystack bytes, ignore ascii case.
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_search_bytes_ignore_ascii_case(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    naive_opt_mc_bytes_iac(haystack, needle)
}

///
/// reverse search the needle in the haystack, ignore ascii case.
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_rsearch_ignore_ascii_case(haystack: &str, needle: &str) -> Option<usize> {
    naive_opt_mc_rev_bytes_iac(haystack.as_bytes(), needle.as_bytes())
}

///
/// reverse search the needle in the haystack bytes, ignore ascii case.
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_rsearch_bytes_ignore_ascii_case(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    naive_opt_mc_rev_bytes_iac(haystack, needle)
}

///
/// An iterator over the matches of the needle in the haystack.
///
/// Examples
///
/// ```rust
/// use naive_opt::string_search_indices;
///
/// let v: Vec<_> = string_search_indices("abc345abc901abc", "abc").collect();
/// assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
///
/// let v: Vec<_> = string_search_indices("0abcabc7", "abc").collect();
/// assert_eq!(v, [(1, "abc"), (4, "abc")]);
///
/// let v: Vec<_> = string_search_indices("ababa", "aba").collect();
/// assert_eq!(v, [(0, "aba")]); // only the first `aba`
/// ```
///
pub fn string_search_indices<'a, P: SearchIn<'a>>(
    haystack: &'a str,
    needle: P,
) -> SearchIndices<'a, P> {
    SearchIndices::new(haystack, needle)
}

pub fn string_search_indices_bytes<'a, P: SearchInBytes<'a>>(
    haystack: &'a [u8],
    needle: P,
) -> SearchIndicesBytes<'a, P> {
    SearchIndicesBytes::new(haystack, needle)
}

///
/// An reverse search iterator over the matches of the needle in the haystack.
///
/// Examples
///
/// ```rust
/// use naive_opt::string_rsearch_indices;
///
/// let v: Vec<_> = string_rsearch_indices("abc345abc901abc", "abc").collect();
/// assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
///
/// let v: Vec<_> = string_rsearch_indices("0abcabc7", "abc").collect();
/// assert_eq!(v, [(4, "abc"), (1, "abc")]);
///
/// let v: Vec<_> = string_rsearch_indices("ababa", "aba").collect();
/// assert_eq!(v, [(2, "aba")]); // only the last `aba`
/// ```
///
pub fn string_rsearch_indices<'a, P: SearchIn<'a>>(
    haystack: &'a str,
    needle: P,
) -> RevSearchIndices<'a, P> {
    RevSearchIndices::new(haystack, needle)
}

pub fn string_rsearch_indices_bytes<'a, P: SearchInBytes<'a>>(
    haystack: &'a [u8],
    needle: P,
) -> RevSearchIndicesBytes<'a, P> {
    RevSearchIndicesBytes::new(haystack, needle)
}

pub fn string_search_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
    haystack: &'a str,
    needle: P,
) -> SearchIndicesIgnoreAsciiCase<'a, P> {
    SearchIndicesIgnoreAsciiCase::new(haystack, needle)
}

pub fn string_search_indices_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
    haystack: &'a [u8],
    needle: P,
) -> SearchIndicesBytesIgnoreAsciiCase<'a, P> {
    SearchIndicesBytesIgnoreAsciiCase::new(haystack, needle)
}

pub fn string_rsearch_indices_ignore_ascii_case<'a, P: SearchIn<'a>>(
    haystack: &'a str,
    needle: P,
) -> RevSearchIndicesIgnoreAsciiCase<'a, P> {
    RevSearchIndicesIgnoreAsciiCase::new(haystack, needle)
}

pub fn string_rsearch_indices_bytes_ignore_ascii_case<'a, P: SearchInBytes<'a>>(
    haystack: &'a [u8],
    needle: P,
) -> RevSearchIndicesBytesIgnoreAsciiCase<'a, P> {
    RevSearchIndicesBytesIgnoreAsciiCase::new(haystack, needle)
}
//
// Only UTF-8 character sequence are used in the rust.
//
// utf8 byte sequence:
//     1 bytes: 7F
//     2 bytes: DF BF
//     3 bytes: EF BF BF
//     4 bytes: F4 8F BF BF
// 1st byte (of 2..4 bytes seq) is likely to be repeated.
//
// I think it is stochastically effective to use the last byte for seaching.
//
// Otherwise the ASCII character are using many space code: 0x20.
// This code do stochastics using by the 1st byte and last byte.
//

#[cfg(not(feature = "only_mc_last"))]
mod mc_1st;

#[cfg(not(feature = "only_mc_1st"))]
mod mc_last;

#[inline(always)]
fn naive_opt_mc_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    #[cfg(feature = "only_mc_1st")]
    {
        mc_1st::naive_opt_mc_1st_bytes(hay_bytes, nee_bytes)
    }
    #[cfg(feature = "only_mc_last")]
    {
        mc_last::naive_opt_mc_last_bytes(hay_bytes, nee_bytes)
    }
    #[cfg(all(not(feature = "only_mc_1st"), not(feature = "only_mc_last")))]
    {
        if nee_bytes.is_empty() {
            return Some(0);
        }
        let byte_1st = nee_bytes[0];
        let byte_last = nee_bytes[nee_bytes.len() - 1];
        if byte_1st.is_ascii() && byte_last.is_ascii() {
            let weight_1st = _ASCII_STOCHAS[byte_1st as usize];
            let weight_last = _ASCII_STOCHAS[byte_last as usize];
            if weight_1st <= weight_last {
                mc_1st::naive_opt_mc_1st_bytes(hay_bytes, nee_bytes)
            } else {
                mc_last::naive_opt_mc_last_bytes(hay_bytes, nee_bytes)
            }
        } else {
            mc_last::naive_opt_mc_last_bytes(hay_bytes, nee_bytes)
        }
    }
}

#[inline(always)]
fn naive_opt_mc_rev_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    #[cfg(feature = "only_mc_1st")]
    {
        mc_1st::naive_opt_mc_1st_rev_bytes(hay_bytes, nee_bytes)
    }
    #[cfg(feature = "only_mc_last")]
    {
        mc_last::naive_opt_mc_last_rev_bytes(hay_bytes, nee_bytes)
    }
    #[cfg(all(not(feature = "only_mc_1st"), not(feature = "only_mc_last")))]
    {
        if nee_bytes.is_empty() {
            return Some(hay_bytes.len());
        }
        let byte_1st = nee_bytes[0];
        let byte_last = nee_bytes[nee_bytes.len() - 1];
        if byte_1st.is_ascii() && byte_last.is_ascii() {
            let weight_1st = _ASCII_STOCHAS[byte_1st as usize];
            let weight_last = _ASCII_STOCHAS[byte_last as usize];
            if weight_1st <= weight_last {
                mc_1st::naive_opt_mc_1st_rev_bytes(hay_bytes, nee_bytes)
            } else {
                mc_last::naive_opt_mc_last_rev_bytes(hay_bytes, nee_bytes)
            }
        } else {
            mc_last::naive_opt_mc_last_rev_bytes(hay_bytes, nee_bytes)
        }
    }
}

// ascii stochastics
const _ASCII_STOCHAS: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 0, 2, 0, 0, 0, 0, 0, 1, 1, 0, 3, 6, 14, 19, 1, 3, 4, 3, 2, 2, 1, 1, 1, 1, 1, 2, 0, 0, 1,
    0, 0, 0, 4, 1, 5, 2, 4, 3, 0, 1, 5, 0, 0, 2, 3, 3, 2, 5, 0, 4, 6, 6, 1, 0, 0, 0, 0, 0, 1, 0, 1,
    0, 1, 0, 39, 7, 20, 19, 69, 11, 9, 18, 39, 0, 2, 18, 12, 38, 38, 12, 1, 34, 35, 50, 13, 5, 5,
    2, 7, 0, 0, 2, 0, 0, 0,
];

#[inline(always)]
fn naive_opt_mc_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    #[cfg(feature = "only_mc_1st")]
    {
        mc_1st::naive_opt_mc_1st_bytes_iac(hay_bytes, nee_bytes)
    }
    #[cfg(feature = "only_mc_last")]
    {
        mc_last::naive_opt_mc_last_bytes_iac(hay_bytes, nee_bytes)
    }
    #[cfg(all(not(feature = "only_mc_1st"), not(feature = "only_mc_last")))]
    {
        if nee_bytes.is_empty() {
            return Some(0);
        }
        let byte_1st = nee_bytes[0];
        let byte_last = nee_bytes[nee_bytes.len() - 1];
        if byte_1st.is_ascii() && byte_last.is_ascii() {
            let weight_1st = _ASCII_STOCHAS[byte_1st as usize];
            let weight_last = _ASCII_STOCHAS[byte_last as usize];
            if weight_1st <= weight_last {
                mc_1st::naive_opt_mc_1st_bytes_iac(hay_bytes, nee_bytes)
            } else {
                mc_last::naive_opt_mc_last_bytes_iac(hay_bytes, nee_bytes)
            }
        } else {
            mc_last::naive_opt_mc_last_bytes_iac(hay_bytes, nee_bytes)
        }
    }
}

#[inline(always)]
fn naive_opt_mc_rev_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    #[cfg(feature = "only_mc_1st")]
    {
        mc_1st::naive_opt_mc_1st_rev_bytes_iac(hay_bytes, nee_bytes)
    }
    #[cfg(feature = "only_mc_last")]
    {
        mc_last::naive_opt_mc_last_rev_bytes_iac(hay_bytes, nee_bytes)
    }
    #[cfg(all(not(feature = "only_mc_1st"), not(feature = "only_mc_last")))]
    {
        if nee_bytes.is_empty() {
            return Some(hay_bytes.len());
        }
        let byte_1st = nee_bytes[0];
        let byte_last = nee_bytes[nee_bytes.len() - 1];
        if byte_1st.is_ascii() && byte_last.is_ascii() {
            let weight_1st = _ASCII_STOCHAS[byte_1st as usize];
            let weight_last = _ASCII_STOCHAS[byte_last as usize];
            if weight_1st <= weight_last {
                mc_1st::naive_opt_mc_1st_rev_bytes_iac(hay_bytes, nee_bytes)
            } else {
                mc_last::naive_opt_mc_last_rev_bytes_iac(hay_bytes, nee_bytes)
            }
        } else {
            mc_last::naive_opt_mc_last_rev_bytes_iac(hay_bytes, nee_bytes)
        }
    }
}
