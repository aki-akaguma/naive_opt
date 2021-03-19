//!
//! The optimized naive string-search algorithm.
//!
//! * Enhanced with 1-byte search like the libc++ and the libstd++ string::find
//! * Specializing in UTF-8 strings, which is a feature of rust
//! * Support the zero overhead trait.
//!
//! # Examples
//!
//! ## Example function:
//!
//! ```rust
//! use naive_opt::string_search;
//! let haystack = "111 a 111b";
//! let needle = "a";
//! let r = string_search(haystack, needle);
//! assert_eq!(r, Some(4));
//! ```
//!
//! ## Example trait 1:
//!
//! ```rust
//! use naive_opt::Search;
//! let haystack = "111 a 111b";
//! let needle = "a";
//! let r = haystack.search(needle);
//! assert_eq!(r, Some(4));
//! ```
//!
//! ## Example trait 2:
//!
//! ```rust
//! use naive_opt::SearchIn;
//! let haystack = "111 a 111b";
//! let needle = "a";
//! let r = needle.search_in(haystack);
//! assert_eq!(r, Some(4));
//! ```
//!

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
    /// includes the needle in self.
    ///
    /// returns true if the given pattern matches a sub-slice of this string slice.
    /// returns false if it does not.
    ///
    fn includes<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool;
}
impl<'c> Search for &'c str {
    #[inline]
    fn search<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in(self)
    }
    #[inline]
    fn search_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> SearchIndices<'a, P> {
        SearchIndices::new(self, needle)
    }
    #[inline]
    fn includes<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in(self)
    }
}
impl<'c> Search for String {
    #[inline]
    fn search<'a, P: SearchIn<'a>>(&'a self, needle: P) -> Option<usize> {
        needle.search_in(self.as_str())
    }
    #[inline]
    fn search_indices<'a, P: SearchIn<'a>>(&'a self, needle: P) -> SearchIndices<'a, P> {
        SearchIndices::new(self.as_str(), needle)
    }
    #[inline]
    fn includes<'a, P: SearchIn<'a>>(&'a self, needle: P) -> bool {
        needle.includes_in(self)
    }
}

///
/// Created with the method [search_indices].
///
pub struct SearchIndices<'a, P: SearchIn<'a>> {
    curr_idx: usize,
    haystack: &'a str,
    needle: P,
}
impl<'a, P: SearchIn<'a>> SearchIndices<'a, P> {
    fn new (a_haystack: &'a str, a_needle: P) -> SearchIndices<'a, P> {
        SearchIndices {
            curr_idx: 0,
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, P: SearchIn<'a>> Iterator for SearchIndices<'a, P> {
    type Item = (usize, &'a str);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.needle.search_in(&self.haystack[self.curr_idx..]) {
            Some(idx) => {
                self.curr_idx += idx;
                let st = self.curr_idx;
                let ed = st + self.needle.len();
                self.curr_idx = ed;
                Some((st, &self.haystack[st..ed]))
            },
            None => None
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
    /// return the length of self
    ///
    fn len(&self) -> usize;
    ///
    /// includes self in the haystack.
    ///
    /// returns true if the given pattern matches a sub-slice of this string slice.
    /// returns false if it does not.
    ///
    #[inline]
    fn includes_in(&self, haystack: &'a str) -> bool {
        !self.search_in(haystack).is_none()
    }
}
impl<'a, 'b> SearchIn<'a> for &'b str {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_last(haystack, self)
    }
    #[inline]
    fn len(&self) -> usize {
        self.as_bytes().len()
    }
}
impl<'a, 'b> SearchIn<'a> for &'b String {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_last(haystack, self.as_str())
    }
    #[inline]
    fn len(&self) -> usize {
        self.as_str().len()
    }
}
impl<'a, 'b> SearchIn<'a> for char {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_last(haystack, self.to_string().as_str())
    }
    #[inline]
    fn len(&self) -> usize {
        self.to_string().as_bytes().len()
    }
}

///
/// seach the needle in the haystack
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_search(haystack: &str, needle: &str) -> Option<usize> {
    naive_opt_mc_last(haystack, needle)
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
///
pub fn string_search_indices<'a, 'b>(haystack: &'a str, needle: &'b str) -> SearchIndices2<'a, 'b> {
    SearchIndices2::new(haystack, needle)
}

///
/// Created with the function [string_search_indices].
///
pub struct SearchIndices2<'a, 'b> {
    curr_idx: usize,
    haystack: &'a str,
    needle: &'b str,
}
impl<'a, 'b> SearchIndices2<'a, 'b> {
    fn new (a_haystack: &'a str, a_needle: &'b str) -> SearchIndices2<'a, 'b> {
        SearchIndices2 {
            curr_idx: 0,
            haystack: a_haystack,
            needle: a_needle,
        }
    }
}
impl<'a, 'b> Iterator for SearchIndices2<'a, 'b> {
    type Item = (usize, &'a str);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.needle.search_in(&self.haystack[self.curr_idx..]) {
            Some(idx) => {
                self.curr_idx += idx;
                let st = self.curr_idx;
                let ed = st + self.needle.len();
                self.curr_idx = ed;
                Some((st, &self.haystack[st..ed]))
            },
            None => None
        }
    }
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
#[inline(always)]
fn naive_opt_mc_last(haystack: &str, needle: &str) -> Option<usize> {
    let hay_bytes = haystack.as_bytes();
    let nee_bytes = needle.as_bytes();
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(0);
    }
    let last_idx = nee_len - 1;
    let last_byte = nee_bytes[last_idx];
    if hay_len <= last_idx {
        return None;
    }
    for m in memchr::memchr_iter(last_byte, &hay_bytes[last_idx..]) {
        let st = m;
        let ed = st + nee_len;
        if ed > hay_len {
            break;
        }
        if nee_bytes == &hay_bytes[st..ed] {
            return Some(st);
        }
    }
    None
}
