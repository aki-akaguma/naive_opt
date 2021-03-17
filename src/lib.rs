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
    fn search<'a, P>(&'a self, needle: P) -> Option<usize>
    where
        P: SearchIn<'a>;
}
impl<'c> Search for &'c str {
    #[inline]
    fn search<'a, P>(&'a self, needle: P) -> Option<usize>
    where
        P: SearchIn<'a>,
    {
        needle.search_in(self)
    }
}
impl<'c> Search for String {
    #[inline]
    fn search<'a, P>(&'a self, needle: P) -> Option<usize>
    where
        P: SearchIn<'a>,
    {
        needle.search_in(self.as_str())
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
    fn search_in(self, haystack: &'a str) -> Option<usize>;
}
impl<'a, 'b> SearchIn<'a> for &'b str {
    #[inline]
    fn search_in(self, haystack: &'a str) -> Option<usize> {
        string_search(haystack, self)
    }
}
impl<'a, 'b> SearchIn<'a> for &'b String {
    #[inline]
    fn search_in(self, haystack: &'a str) -> Option<usize> {
        string_search(haystack, self.as_str())
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
