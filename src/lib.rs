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
//! use naive_opt::{string_search, string_rsearch};
//! use naive_opt::{string_search_indices, string_rsearch_indices};
//!
//! let haystack = "111 a 111b";
//! let needle = "a";
//! let r = string_search(haystack, needle);
//! assert_eq!(r, Some(4));
//!
//! assert_eq!(string_search(haystack, "1"), Some(0));
//! assert_eq!(string_rsearch(haystack, "1"), Some(8));
//!
//! let v: Vec<_> = string_search_indices("abc345abc901abc", "abc").collect();
//! assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
//! let v: Vec<_> = string_rsearch_indices("abc345abc901abc", "abc").collect();
//! assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
//! ```
//!
//! ## Example trait: Search
//!
//! ```rust
//! use naive_opt::Search;
//!
//! let haystack = "111 a 111b";
//! let needle = "a";
//! let r = haystack.search(needle);
//! assert_eq!(r, Some(4));
//!
//! assert_eq!(haystack.search("1"), Some(0));
//! assert_eq!(haystack.rsearch("1"), Some(8));
//!
//! let v: Vec<_> = "abc345abc901abc".search_indices("abc").collect();
//! assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
//! let v: Vec<_> = "abc345abc901abc".rsearch_indices("abc").collect();
//! assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
//! ```
//!
//! ## Example trait: SearchIn
//!
//! ```rust
//! use naive_opt::SearchIn;
//!
//! let haystack = "111 a 111b";
//! let needle = "a";
//! let r = needle.search_in(haystack);
//! assert_eq!(r, Some(4));
//!
//! assert_eq!("1".search_in(haystack), Some(0));
//! assert_eq!("1".rsearch_in(haystack), Some(8));
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
}
impl<'c> Search for &'c str {
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
}
impl<'c> Search for String {
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
}
impl<'a, 'b> SearchIn<'a> for &'b str {
    #[inline]
    fn search_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_last(haystack, self)
    }
    #[inline]
    fn rsearch_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_last_rev(haystack, self)
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
    fn rsearch_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_last_rev(haystack, self.as_str())
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
    fn rsearch_in(&self, haystack: &'a str) -> Option<usize> {
        naive_opt_mc_last_rev(haystack, self.to_string().as_str())
    }
    #[inline]
    fn len(&self) -> usize {
        self.to_string().as_bytes().len()
    }
}

///
/// search the needle in the haystack
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_search(haystack: &str, needle: &str) -> Option<usize> {
    naive_opt_mc_last(haystack, needle)
}

///
/// reverse search the needle in the haystack
///
/// return index of the haystack, if it found the needle. Otherwise return None.
///
pub fn string_rsearch(haystack: &str, needle: &str) -> Option<usize> {
    naive_opt_mc_last_rev(haystack, needle)
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
    for m in my_memchr_iter(last_byte, &hay_bytes[last_idx..]) {
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

#[inline(always)]
fn naive_opt_mc_last_rev(haystack: &str, needle: &str) -> Option<usize> {
    let hay_bytes = haystack.as_bytes();
    let nee_bytes = needle.as_bytes();
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(hay_len);
    }
    let last_idx = nee_len - 1;
    let last_byte = nee_bytes[last_idx];
    if hay_len <= last_idx {
        return None;
    }
    for m in my_memrchr_iter(last_byte, &hay_bytes[last_idx..]) {
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

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn my_memchr_iter(needle: u8, haystack: &[u8]) -> memchr::Memchr {
    memchr::memchr_iter(needle, haystack)
}
#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn my_memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<memchr::Memchr> {
    memchr::memrchr_iter(needle, haystack)
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
fn my_memchr_iter(needle: u8, haystack: &[u8]) -> iter::Memchr {
    iter::memchr_iter(needle, haystack)
}
#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
fn my_memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<iter::Memchr> {
    iter::memrchr_iter(needle, haystack)
}

#[cfg(not(target_arch = "x86_64"))]
mod iter {
    #[inline]
    pub fn memchr_iter(needle: u8, haystack: &[u8]) -> Memchr {
        Memchr::new(needle, haystack)
    }
    #[inline]
    pub fn memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<Memchr> {
        Memchr::new(needle, haystack).rev()
    }
    pub struct Memchr<'a> {
        needle: u8,
        haystack: &'a [u8],
        position: usize,
    }
    impl<'a> Memchr<'a> {
        #[inline]
        pub fn new(needle: u8, haystack: &[u8]) -> Memchr {
            Memchr { needle: needle, haystack: haystack, position: 0 }
        }
    }
    impl<'a> Iterator for Memchr<'a> {
        type Item = usize;
        #[inline]
        fn next(&mut self) -> Option<usize> {
            memchr(self.needle, self.haystack).map(move |idx|{
                self.haystack = self.haystack.split_at(idx + 1).1;
                let found = self.position + idx;
                self.position = found + 1;
                found
            })
        }
        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(self.haystack.len()))
        }
    }
    impl<'a> DoubleEndedIterator for Memchr<'a> {
        #[inline]
        fn next_back(&mut self) -> Option<Self::Item> {
            memrchr(self.needle, self.haystack).map(move |idx| {
                self.haystack = self.haystack.split_at(idx).0;
                self.position + idx
            })
        }
    }
    // naive algorithm
    /*
    fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
        haystack.iter().position(|&b| b == n1)
    }
    fn memrchr(n1: u8, haystack: &[u8]) -> Option<usize> {
        haystack.iter().rposition(|&b| b == n1)
    }
    */
    fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
        let haystack_len = haystack.len();
        let haystack = haystack.as_ptr() as *const libc::c_void;
        let n1 = n1 as i32;
        unsafe {
            let p = libc::memchr(haystack, n1, haystack_len);
            if p.is_null() {
                None
            } else {
                Some(p as usize - haystack as usize)
            }
        }
    }
    fn memrchr(n1: u8, haystack: &[u8]) -> Option<usize> {
        let haystack_len = haystack.len();
        let haystack = haystack.as_ptr() as *const libc::c_void;
        let n1 = n1 as i32;
        unsafe {
            let p = libc::memrchr(haystack, n1, haystack_len);
            if p.is_null() {
                None
            } else {
                Some(p as usize - haystack as usize)
            }
        }
    }
}
