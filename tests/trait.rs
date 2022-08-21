macro_rules! search_test {
    ($haystack:expr, $needle:expr, $result:expr) => {
        let r = $haystack.search($needle);
        assert_eq!(r, $result);
        let r = $haystack.as_bytes().search_bytes($needle.as_bytes());
        assert_eq!(r, $result);
    };
    ($haystack:expr, $needle:expr, $result:expr, $result_ignore_ascii_case:expr) => {
        search_test!($haystack, $needle, $result);
        //
        let r = $haystack.search_ignore_ascii_case($needle);
        assert_eq!(r, $result_ignore_ascii_case);
        let r = $haystack
            .as_bytes()
            .search_bytes_ignore_ascii_case($needle.as_bytes());
        assert_eq!(r, $result_ignore_ascii_case);
    };
}
macro_rules! rsearch_test {
    ($haystack:expr, $needle:expr, $result:expr) => {
        let r = $haystack.rsearch($needle);
        assert_eq!(r, $result);
        let r = $haystack.as_bytes().rsearch_bytes($needle.as_bytes());
        assert_eq!(r, $result);
    };
    ($haystack:expr, $needle:expr, $result:expr, $result_ignore_ascii_case:expr) => {
        rsearch_test!($haystack, $needle, $result);
        //
        let r = $haystack.rsearch_ignore_ascii_case($needle);
        assert_eq!(r, $result_ignore_ascii_case);
        let r = $haystack
            .as_bytes()
            .rsearch_bytes_ignore_ascii_case($needle.as_bytes());
        assert_eq!(r, $result_ignore_ascii_case);
    };
}

#[cfg(test)]
mod trait_str_str {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_empty_needle() {
        search_test!("", "", Some(0), Some(0));
        search_test!("1", "", Some(0), Some(0));
        let haystack = "111 a 111b";
        search_test!(haystack, "", Some(0), Some(0));
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b";
        search_test!(haystack, "xxx", None, None);
        search_test!(haystack, "12b", None, None);
        search_test!(haystack, "a31", None, None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b";
        let needle = "111 a 111b";
        search_test!(haystack, needle, Some(0), Some(0));
        let needle = "111 A 111B";
        search_test!(haystack, needle, None, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b";
        let needle = "111 a 1 1b";
        search_test!(haystack, needle, None, None);
        let needle = "111 A 1 1A";
        search_test!(haystack, needle, None, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b";
        let needle = "111 a 111bZ";
        search_test!(haystack, needle, None, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b";
        search_test!(haystack, "b", Some(9));
        search_test!(haystack, "B", None, Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b";
        search_test!(haystack, "a", Some(4));
        search_test!(haystack, "a 111", Some(4));
        search_test!(haystack, "A", None, Some(4));
        search_test!(haystack, "A 111", None, Some(4));
    }
}

#[cfg(test)]
mod trait_str_string {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_empty_needle() {
        search_test!("", &"".to_string(), Some(0), Some(0));
        search_test!("1", &"".to_string(), Some(0), Some(0));
        let haystack = "111 a 111b";
        search_test!(haystack, &"".to_string(), Some(0), Some(0));
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b";
        search_test!(haystack, &"xxx".to_string(), None, None);
        search_test!(haystack, &"12b".to_string(), None, None);
        search_test!(haystack, &"a31".to_string(), None, None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b";
        let needle = "111 a 111b".to_string();
        search_test!(haystack, &needle, Some(0), Some(0));
        let needle = "111 A 111B".to_string();
        search_test!(haystack, &needle, None, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b";
        let needle = "111 a 1 1b".to_string();
        search_test!(haystack, &needle, None, None);
        let needle = "111 A 1 1B".to_string();
        search_test!(haystack, &needle, None, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b";
        let needle = "111 a 111bZ".to_string();
        search_test!(haystack, &needle, None, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b";
        search_test!(haystack, &"b".to_string(), Some(9));
        search_test!(haystack, &"B".to_string(), None, Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b";
        search_test!(haystack, &"a".to_string(), Some(4));
        search_test!(haystack, &"a 111".to_string(), Some(4));
        search_test!(haystack, &"A".to_string(), None, Some(4));
        search_test!(haystack, &"A 111".to_string(), None, Some(4));
    }
}

#[cfg(test)]
mod trait_string_str {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_empty_needle() {
        search_test!(&"".to_string(), "", Some(0), Some(0));
        search_test!(&"1".to_string(), "", Some(0), Some(0));
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "", Some(0), Some(0));
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "xxx", None, None);
        search_test!(&haystack, "12b", None, None);
        search_test!(&haystack, "a31", None, None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111b";
        search_test!(&haystack, needle, Some(0), Some(0));
        let needle = "111 A 111B";
        search_test!(&haystack, needle, None, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 1 1b";
        search_test!(&haystack, needle, None, None);
        let needle = "111 A 1 1B";
        search_test!(&haystack, needle, None, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111bZ";
        search_test!(&haystack, needle, None, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "b", Some(9));
        search_test!(&haystack, "B", None, Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "a", Some(4));
        search_test!(&haystack, "a 111", Some(4));
        search_test!(&haystack, "A", None, Some(4));
        search_test!(&haystack, "A 111", None, Some(4));
    }
}

#[cfg(test)]
mod trait_string_string {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_empty_needle() {
        search_test!(&"".to_string(), &"".to_string(), Some(0), Some(0));
        search_test!(&"1".to_string(), &"".to_string(), Some(0), Some(0));
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"".to_string(), Some(0), Some(0));
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"xxx".to_string(), None, None);
        search_test!(&haystack, &"12b".to_string(), None, None);
        search_test!(&haystack, &"a31".to_string(), None, None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111b".to_string();
        search_test!(&haystack, &needle, Some(0), Some(0));
        let needle = "111 A 111B".to_string();
        search_test!(&haystack, &needle, None, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 1 1b".to_string();
        search_test!(&haystack, &needle, None, None);
        let needle = "111 A 1 1B".to_string();
        search_test!(&haystack, &needle, None, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111bZ".to_string();
        search_test!(&haystack, &needle, None, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"b".to_string(), Some(9));
        search_test!(&haystack, &"B".to_string(), None, Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"a".to_string(), Some(4));
        search_test!(&haystack, &"a 111".to_string(), Some(4));
        search_test!(&haystack, &"A".to_string(), None, Some(4));
        search_test!(&haystack, &"A 111".to_string(), None, Some(4));
    }
}

#[cfg(test)]
mod trait_str_str_rev {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_empty_needle() {
        rsearch_test!("", "", Some(0), Some(0));
        rsearch_test!("1", "", Some(1), Some(1));
        let haystack = "111 a 111b";
        rsearch_test!(haystack, "", Some(10), Some(10));
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b";
        rsearch_test!(haystack, "xxx", None, None);
        rsearch_test!(haystack, "12b", None, None);
        rsearch_test!(haystack, "a31", None, None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b";
        let needle = "111 a 111b";
        rsearch_test!(haystack, needle, Some(0), Some(0));
        let needle = "111 A 111B";
        rsearch_test!(haystack, needle, None, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b";
        let needle = "111 a 1 1b";
        rsearch_test!(haystack, needle, None, None);
        let needle = "111 A 1 1B";
        rsearch_test!(haystack, needle, None, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b";
        let needle = "111 a 111bZ";
        rsearch_test!(haystack, needle, None, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b";
        rsearch_test!(haystack, "b", Some(9));
        search_test!(haystack, "B", None, Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b";
        rsearch_test!(haystack, "a", Some(4));
        rsearch_test!(haystack, "a 111", Some(4));
        search_test!(haystack, "A", None, Some(4));
        search_test!(haystack, "A 111", None, Some(4));
    }
}

#[cfg(test)]
mod trai_search_indices {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_search_indices_0() {
        let haystack = "aaa 1 aaab";
        let needle = "a";
        let mut m = haystack.search_indices(needle);
        assert_eq!(m.next(), Some((0, "a")));
        assert_eq!(m.next(), Some((1, "a")));
        assert_eq!(m.next(), Some((2, "a")));
        assert_eq!(m.next(), Some((6, "a")));
        assert_eq!(m.next(), Some((7, "a")));
        assert_eq!(m.next(), Some((8, "a")));
        assert_eq!(m.next(), None);
    }
    #[test]
    fn test_search_indices_bytes_0() {
        let haystack = "aaa 1 aaab".as_bytes();
        let needle = "a".as_bytes();
        let mut m = haystack.search_indices_bytes(needle);
        assert_eq!(m.next(), Some((0, "a".as_bytes())));
        assert_eq!(m.next(), Some((1, "a".as_bytes())));
        assert_eq!(m.next(), Some((2, "a".as_bytes())));
        assert_eq!(m.next(), Some((6, "a".as_bytes())));
        assert_eq!(m.next(), Some((7, "a".as_bytes())));
        assert_eq!(m.next(), Some((8, "a".as_bytes())));
        assert_eq!(m.next(), None);
    }
    #[test]
    fn test_search_indices_ignore_ascii_case_0() {
        let haystack = "aAa 1 aAab";
        let needle = "A";
        let mut m = haystack.search_indices_ignore_ascii_case(needle);
        assert_eq!(m.next(), Some((0, "a")));
        assert_eq!(m.next(), Some((1, "A")));
        assert_eq!(m.next(), Some((2, "a")));
        assert_eq!(m.next(), Some((6, "a")));
        assert_eq!(m.next(), Some((7, "A")));
        assert_eq!(m.next(), Some((8, "a")));
        assert_eq!(m.next(), None);
    }
    #[test]
    fn test_search_indices_bytes_ignore_ascii_case_0() {
        let haystack = "aAa 1 aAab".as_bytes();
        let needle = "A".as_bytes();
        let mut m = haystack.search_indices_bytes_ignore_ascii_case(needle);
        assert_eq!(m.next(), Some((0, "a".as_bytes())));
        assert_eq!(m.next(), Some((1, "A".as_bytes())));
        assert_eq!(m.next(), Some((2, "a".as_bytes())));
        assert_eq!(m.next(), Some((6, "a".as_bytes())));
        assert_eq!(m.next(), Some((7, "A".as_bytes())));
        assert_eq!(m.next(), Some((8, "a".as_bytes())));
        assert_eq!(m.next(), None);
    }
}

#[cfg(test)]
mod trai_rsearch_indices {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_rsearch_indices_0() {
        let haystack = "aaa 1 aaab";
        let needle = "a";
        let mut m = haystack.rsearch_indices(needle);
        assert_eq!(m.next(), Some((8, "a")));
        assert_eq!(m.next(), Some((7, "a")));
        assert_eq!(m.next(), Some((6, "a")));
        assert_eq!(m.next(), Some((2, "a")));
        assert_eq!(m.next(), Some((1, "a")));
        assert_eq!(m.next(), Some((0, "a")));
        assert_eq!(m.next(), None);
    }
    #[test]
    fn test_rsearch_indices_bytes_0() {
        let haystack = "aaa 1 aaab".as_bytes();
        let needle = "a".as_bytes();
        let mut m = haystack.rsearch_indices_bytes(needle);
        assert_eq!(m.next(), Some((8, "a".as_bytes())));
        assert_eq!(m.next(), Some((7, "a".as_bytes())));
        assert_eq!(m.next(), Some((6, "a".as_bytes())));
        assert_eq!(m.next(), Some((2, "a".as_bytes())));
        assert_eq!(m.next(), Some((1, "a".as_bytes())));
        assert_eq!(m.next(), Some((0, "a".as_bytes())));
        assert_eq!(m.next(), None);
    }
    #[test]
    fn test_rsearch_indices_ignore_ascii_case_0() {
        let haystack = "aAa 1 aAab";
        let needle = "A";
        let mut m = haystack.rsearch_indices_ignore_ascii_case(needle);
        assert_eq!(m.next(), Some((8, "a")));
        assert_eq!(m.next(), Some((7, "A")));
        assert_eq!(m.next(), Some((6, "a")));
        assert_eq!(m.next(), Some((2, "a")));
        assert_eq!(m.next(), Some((1, "A")));
        assert_eq!(m.next(), Some((0, "a")));
        assert_eq!(m.next(), None);
    }
    #[test]
    fn test_rsearch_indices_bytes_ignore_ascii_case_0() {
        let haystack = "aAa 1 aAab".as_bytes();
        let needle = "A".as_bytes();
        let mut m = haystack.rsearch_indices_bytes_ignore_ascii_case(needle);
        assert_eq!(m.next(), Some((8, "a".as_bytes())));
        assert_eq!(m.next(), Some((7, "A".as_bytes())));
        assert_eq!(m.next(), Some((6, "a".as_bytes())));
        assert_eq!(m.next(), Some((2, "a".as_bytes())));
        assert_eq!(m.next(), Some((1, "A".as_bytes())));
        assert_eq!(m.next(), Some((0, "a".as_bytes())));
        assert_eq!(m.next(), None);
    }
}

#[cfg(test)]
mod trai_str_str_large {
    use naive_opt::{Search, SearchBytes};
    #[test]
    fn test_large_needle_found() {
        let haystack = "c11 a 111b1234567890AbcDe67890";
        //
        let needle = "1234567890AbcDe67890";
        search_test!(haystack, needle, Some(10), Some(10));
        rsearch_test!(haystack, needle, Some(10), Some(10));
        let needle = "1234567890abcde67890";
        search_test!(haystack, needle, None, Some(10));
        rsearch_test!(haystack, needle, None, Some(10));
        let needle = "1234567890aBCdE67890";
        search_test!(haystack, needle, None, Some(10));
        rsearch_test!(haystack, needle, None, Some(10));
        //
        let needle = "AbcDe67890";
        search_test!(haystack, needle, Some(20), Some(20));
        rsearch_test!(haystack, needle, Some(20), Some(20));
        let needle = "abcde67890";
        search_test!(haystack, needle, None, Some(20));
        rsearch_test!(haystack, needle, None, Some(20));
        let needle = "aBCdE67890";
        search_test!(haystack, needle, None, Some(20));
        rsearch_test!(haystack, needle, None, Some(20));
    }
}

//
// a copy from tests of the rust std::str
//
#[cfg(test)]
mod std_from_test_std {
    use naive_opt::Search;
    //
    #[test]
    fn test_find() {
        assert_eq!("hello".search('l'), Some(2));
        //assert_eq!("hello".search(|c: char| c == 'o'), Some(4));
        assert!("hello".search('x').is_none());
        //assert!("hello".search(|c: char| c == 'x').is_none());
        assert_eq!("ประเทศไทย中华Việt Nam".find('华'), Some(30));
        //assert_eq!("ประเทศไทย中华Việt Nam".find(|c: char| c == '华'), Some(30));
    }
    //
    #[test]
    fn test_rfind() {
        assert_eq!("hello".rsearch('l'), Some(3));
        //assert_eq!("hello".rsearch(|c: char| c == 'o'), Some(4));
        assert!("hello".rsearch('x').is_none());
        //assert!("hello".rsearch(|c: char| c == 'x').is_none());
        assert_eq!("ประเทศไทย中华Việt Nam".rsearch('华'), Some(30));
        //assert_eq!("ประเทศไทย中华Việt Nam".rsearch(|c: char| c == '华'), Some(30));
    }
    //
    #[test]
    fn test_find_str() {
        // byte positions
        assert_eq!("".search(""), Some(0));
        assert!("banana".search("apple pie").is_none());
        //
        let data = "abcabc";
        assert_eq!((&data[0..6]).search("ab"), Some(0));
        assert_eq!((&data[2..6]).search("ab"), Some(3 - 2));
        assert!((&data[2..4]).search("ab").is_none());
        //
        let string = "ประเทศไทย中华Việt Nam";
        let mut data = String::from(string);
        data.push_str(string);
        assert!(data.search("ไท华").is_none());
        assert_eq!((&data[0..43]).search(""), Some(0));
        assert_eq!((&data[6..43]).search(""), Some(6 - 6));
        //
        assert_eq!((&data[0..43]).search("ประ"), Some(0));
        assert_eq!((&data[0..43]).search("ทศไ"), Some(12));
        assert_eq!((&data[0..43]).search("ย中"), Some(24));
        assert_eq!((&data[0..43]).search("iệt"), Some(34));
        assert_eq!((&data[0..43]).search("Nam"), Some(40));
        //
        assert_eq!((&data[43..86]).search("ประ"), Some(43 - 43));
        assert_eq!((&data[43..86]).search("ทศไ"), Some(55 - 43));
        assert_eq!((&data[43..86]).search("ย中"), Some(67 - 43));
        assert_eq!((&data[43..86]).search("iệt"), Some(77 - 43));
        assert_eq!((&data[43..86]).search("Nam"), Some(83 - 43));
        //
        // find every substring -- assert that it finds it, or an earlier occurrence.
        let string = "Việt Namacbaabcaabaaba";
        for (i, ci) in string.char_indices() {
            let ip = i + ci.len_utf8();
            for j in string[ip..]
                .char_indices()
                .map(|(i, _)| i)
                .chain(Some(string.len() - ip))
            {
                let pat = &string[i..ip + j];
                assert!(match string.search(pat) {
                    None => false,
                    Some(x) => x <= i,
                });
                assert!(match string.rsearch(pat) {
                    None => false,
                    Some(x) => x >= i,
                });
            }
        }
    }
    //
    #[test]
    fn test_contains() {
        assert!("abcde".includes("bcd"));
        assert!("abcde".includes("abcd"));
        assert!("abcde".includes("bcde"));
        assert!("abcde".includes(""));
        assert!("".includes(""));
        assert!(!"abcde".includes("def"));
        assert!(!"".includes("a"));
        //
        let data = "ประเทศไทย中华Việt Nam";
        assert!(data.includes("ประเ"));
        assert!(data.includes("ะเ"));
        assert!(data.includes("中华"));
        assert!(!data.includes("ไท华"));
    }
    //
    #[test]
    fn test_contains_char() {
        assert!("abc".contains('b'));
        assert!("a".contains('a'));
        assert!(!"abc".contains('d'));
        assert!(!"".contains('a'));
    }
    //
    #[test]
    fn test_pattern_deref_forward() {
        let data = "aabcdaa";
        assert!(data.contains("bcd"));
        assert!(data.contains(&"bcd"));
        assert!(data.contains(&"bcd".to_string()));
    }
    //
    #[test]
    fn test_empty_match_indices() {
        let data = "aä中!";
        let vec: Vec<_> = data.match_indices("").collect();
        assert_eq!(vec, [(0, ""), (1, ""), (3, ""), (6, ""), (7, "")]);
    }
    //
    fn helper_check_contains_all_substrings(s: &str) {
        assert!(s.contains(""));
        for i in 0..s.len() {
            for j in i + 1..=s.len() {
                assert!(s.contains(&s[i..j]));
            }
        }
    }
    //
    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn strslice_issue_16589() {
        assert!("bananas".contains("nana"));
        // prior to the fix for #16589, x.contains("abcdabcd") returned false
        // test all substrings for good measure
        helper_check_contains_all_substrings("012345678901234567890123456789bcdabcdabcd");
    }
    //
    #[test]
    fn strslice_issue_16878() {
        assert!(!"1234567ah012345678901ah".contains("hah"));
        assert!(!"00abc01234567890123456789abc".contains("bcabc"));
    }
    //
    #[test]
    #[cfg_attr(miri, ignore)] // Miri is too slow
    fn test_strslice_contains() {
        let x = "There are moments, Jeeves, when one asks oneself, 'Do trousers matter?'";
        helper_check_contains_all_substrings(x);
    }
    //
    #[test]
    fn contains_weird_cases() {
        assert!("* \t".includes(' '));
        assert!(!"* \t".includes('?'));
        assert!(!"* \t".includes('\u{1F4A9}'));
    }
    /*
    //
    #[test]
    fn different_str_pattern_forwarding_lifetimes() {
        use std::str::pattern::Pattern;
        //
        fn foo<'a, P>(p: P)
        where
            for<'b> &'b P: Pattern<'a>,
        {
            for _ in 0..3 {
                "asdf".find(&p);
            }
        }
        foo::<&str>("x");
    }
    */
}
