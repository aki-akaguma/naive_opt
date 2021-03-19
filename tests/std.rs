macro_rules! search_test {
    ($haystack:expr, $needle:expr, $result:expr) => {
        let r = $haystack.find($needle);
        assert_eq!(r, $result);
    };
}

#[cfg(test)]
mod std_str_str {
    #[test]
    fn test_empty_needle() {
        search_test!("", "", Some(0)); // ***
        search_test!("1", "", Some(0)); // ***
        let haystack = "111 a 111b";
        search_test!(haystack, "", Some(0)); // ***
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b";
        search_test!(haystack, "xxx", None);
        search_test!(haystack, "12b", None);
        search_test!(haystack, "a31", None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b";
        let needle = "111 a 111b";
        search_test!(haystack, needle, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b";
        let needle = "111 a 1 1b";
        search_test!(haystack, needle, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b";
        let needle = "111 a 111bZ";
        search_test!(haystack, needle, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b";
        search_test!(haystack, "b", Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b";
        search_test!(haystack, "a", Some(4));
        search_test!(haystack, "a 111", Some(4));
    }
}

#[cfg(test)]
mod std_str_string {
    #[test]
    fn test_empty_needle() {
        search_test!("", &"".to_string(), Some(0)); // ***
        search_test!("1", &"".to_string(), Some(0)); // ***
        let haystack = "111 a 111b";
        search_test!(haystack, &"".to_string(), Some(0)); // ***
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b";
        search_test!(haystack, &"xxx".to_string(), None);
        search_test!(haystack, &"12b".to_string(), None);
        search_test!(haystack, &"a31".to_string(), None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b";
        let needle = "111 a 111b".to_string();
        search_test!(haystack, &needle, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b";
        let needle = "111 a 1 1b".to_string();
        search_test!(haystack, &needle, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b";
        let needle = "111 a 111bZ".to_string();
        search_test!(haystack, &needle, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b";
        search_test!(haystack, &"b".to_string(), Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b";
        search_test!(haystack, &"a".to_string(), Some(4));
        search_test!(haystack, &"a 111".to_string(), Some(4));
    }
}

#[cfg(test)]
mod std_string_str {
    #[test]
    fn test_empty_needle() {
        search_test!(&"".to_string(), "", Some(0)); // ***
        search_test!(&"1".to_string(), "", Some(0)); // ***
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "", Some(0)); // ***
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "xxx", None);
        search_test!(&haystack, "12b", None);
        search_test!(&haystack, "a31", None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111b";
        search_test!(&haystack, needle, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 1 1b";
        search_test!(&haystack, needle, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111bZ";
        search_test!(&haystack, needle, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "b", Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, "a", Some(4));
        search_test!(&haystack, "a 111", Some(4));
    }
}

#[cfg(test)]
mod std_string_string {
    #[test]
    fn test_empty_needle() {
        search_test!(&"".to_string(), &"".to_string(), Some(0)); // ***
        search_test!(&"1".to_string(), &"".to_string(), Some(0)); // ***
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"".to_string(), Some(0)); // ***
    }
    #[test]
    fn test_not_found() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"xxx".to_string(), None);
        search_test!(&haystack, &"12b".to_string(), None);
        search_test!(&haystack, &"a31".to_string(), None);
    }
    #[test]
    fn test_perfect_matching() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111b".to_string();
        search_test!(&haystack, &needle, Some(0));
    }
    #[test]
    fn test_same_size() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 1 1b".to_string();
        search_test!(&haystack, &needle, None);
    }
    #[test]
    fn test_large_needle() {
        let haystack = "111 a 111b".to_string();
        let needle = "111 a 111bZ".to_string();
        search_test!(&haystack, &needle, None);
    }
    #[test]
    fn test_last_match() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"b".to_string(), Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b".to_string();
        search_test!(&haystack, &"a".to_string(), Some(4));
        search_test!(&haystack, &"a 111".to_string(), Some(4));
    }
}

#[cfg(test)]
mod std_match_indices {
    #[test]
    fn test_match_indices() {
        let haystack = "111 a 111b";
        let needle = "1";
        let mut m = haystack.match_indices(needle);
        assert_eq!(m.next(), Some((0, "1")));
        assert_eq!(m.next(), Some((1, "1")));
        assert_eq!(m.next(), Some((2, "1")));
        assert_eq!(m.next(), Some((6, "1")));
        assert_eq!(m.next(), Some((7, "1")));
        assert_eq!(m.next(), Some((8, "1")));
        assert_eq!(m.next(), None);
    }
}

//
// a copy from tests of the rust std::str
//
#[cfg(test)]
mod std_from_test_std {
    //
    #[test]
    fn test_find() {
        assert_eq!("hello".find('l'), Some(2));
        assert_eq!("hello".find(|c: char| c == 'o'), Some(4));
        assert!("hello".find('x').is_none());
        assert!("hello".find(|c: char| c == 'x').is_none());
        assert_eq!("ประเทศไทย中华Việt Nam".find('华'), Some(30));
        assert_eq!("ประเทศไทย中华Việt Nam".find(|c: char| c == '华'), Some(30));
    }
    //
    #[test]
    fn test_rfind() {
        assert_eq!("hello".rfind('l'), Some(3));
        assert_eq!("hello".rfind(|c: char| c == 'o'), Some(4));
        assert!("hello".rfind('x').is_none());
        assert!("hello".rfind(|c: char| c == 'x').is_none());
        assert_eq!("ประเทศไทย中华Việt Nam".rfind('华'), Some(30));
        assert_eq!("ประเทศไทย中华Việt Nam".rfind(|c: char| c == '华'), Some(30));
    }
    //
    #[test]
    fn test_find_str() {
        // byte positions
        assert_eq!("".find(""), Some(0));
        assert!("banana".find("apple pie").is_none());
        //
        let data = "abcabc";
        assert_eq!(data[0..6].find("ab"), Some(0));
        assert_eq!(data[2..6].find("ab"), Some(3 - 2));
        assert!(data[2..4].find("ab").is_none());
        //
        let string = "ประเทศไทย中华Việt Nam";
        let mut data = String::from(string);
        data.push_str(string);
        assert!(data.find("ไท华").is_none());
        assert_eq!(data[0..43].find(""), Some(0));
        assert_eq!(data[6..43].find(""), Some(6 - 6));
        //
        assert_eq!(data[0..43].find("ประ"), Some(0));
        assert_eq!(data[0..43].find("ทศไ"), Some(12));
        assert_eq!(data[0..43].find("ย中"), Some(24));
        assert_eq!(data[0..43].find("iệt"), Some(34));
        assert_eq!(data[0..43].find("Nam"), Some(40));
        //
        assert_eq!(data[43..86].find("ประ"), Some(43 - 43));
        assert_eq!(data[43..86].find("ทศไ"), Some(55 - 43));
        assert_eq!(data[43..86].find("ย中"), Some(67 - 43));
        assert_eq!(data[43..86].find("iệt"), Some(77 - 43));
        assert_eq!(data[43..86].find("Nam"), Some(83 - 43));
        //
        // find every substring -- assert that it finds it, or an earlier occurrence.
        let string = "Việt Namacbaabcaabaaba";
        for (i, ci) in string.char_indices() {
            let ip = i + ci.len_utf8();
            for j in string[ip..].char_indices().map(|(i, _)| i).chain(Some(string.len() - ip)) {
                let pat = &string[i..ip + j];
                assert!(match string.find(pat) {
                    None => false,
                    Some(x) => x <= i,
                });
                assert!(match string.rfind(pat) {
                    None => false,
                    Some(x) => x >= i,
                });
            }
        }
    }
    //
    #[test]
    fn test_contains() {
        assert!("abcde".contains("bcd"));
        assert!("abcde".contains("abcd"));
        assert!("abcde".contains("bcde"));
        assert!("abcde".contains(""));
        assert!("".contains(""));
        assert!(!"abcde".contains("def"));
        assert!(!"".contains("a"));
        //
        let data = "ประเทศไทย中华Việt Nam";
        assert!(data.contains("ประเ"));
        assert!(data.contains("ะเ"));
        assert!(data.contains("中华"));
        assert!(!data.contains("ไท华"));
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
}
