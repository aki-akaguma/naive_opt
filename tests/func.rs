macro_rules! search_test {
    ($haystack:expr, $needle:expr, $result:expr) => {
        let r = string_search($haystack, $needle);
        assert_eq!(r, $result);
        let r = string_search_bytes($haystack.as_bytes(), $needle.as_bytes());
        assert_eq!(r, $result);
    };
    ($haystack:expr, $needle:expr, $result:expr, $result_ignore_ascii_case:expr) => {
        search_test!($haystack, $needle, $result);
        //
        let r = string_search_ignore_ascii_case($haystack, $needle);
        assert_eq!(r, $result_ignore_ascii_case);
        let r = string_search_bytes_ignore_ascii_case($haystack.as_bytes(), $needle.as_bytes());
        assert_eq!(r, $result_ignore_ascii_case);
    };
}
macro_rules! rsearch_test {
    ($haystack:expr, $needle:expr, $result:expr) => {
        let r = string_rsearch($haystack, $needle);
        assert_eq!(r, $result);
        let r = string_rsearch_bytes($haystack.as_bytes(), $needle.as_bytes());
        assert_eq!(r, $result);
    };
    ($haystack:expr, $needle:expr, $result:expr, $result_ignore_ascii_case:expr) => {
        rsearch_test!($haystack, $needle, $result);
        //
        let r = string_rsearch_ignore_ascii_case($haystack, $needle);
        assert_eq!(r, $result_ignore_ascii_case);
        let r = string_rsearch_bytes_ignore_ascii_case($haystack.as_bytes(), $needle.as_bytes());
        assert_eq!(r, $result_ignore_ascii_case);
    };
}

#[cfg(test)]
mod func_str_str {
    use naive_opt::{string_search, string_search_bytes};
    use naive_opt::{string_search_bytes_ignore_ascii_case, string_search_ignore_ascii_case};
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
mod func_str_string {
    use naive_opt::{string_search, string_search_bytes};
    use naive_opt::{string_search_bytes_ignore_ascii_case, string_search_ignore_ascii_case};
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
mod func_string_str {
    use naive_opt::{string_search, string_search_bytes};
    use naive_opt::{string_search_bytes_ignore_ascii_case, string_search_ignore_ascii_case};
    #[test]
    fn test_empty_needle() {
        search_test!("", "", Some(0), Some(0));
        search_test!("1", "", Some(0), Some(0));
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
mod func_string_string {
    use naive_opt::{string_search, string_search_bytes};
    use naive_opt::{string_search_bytes_ignore_ascii_case, string_search_ignore_ascii_case};
    #[test]
    fn test_empty_needle() {
        search_test!("", "", Some(0), Some(0));
        search_test!("1", "", Some(0), Some(0));
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
mod func_str_str_rev {
    use naive_opt::{string_rsearch, string_rsearch_bytes};
    use naive_opt::{string_rsearch_bytes_ignore_ascii_case, string_rsearch_ignore_ascii_case};
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
        rsearch_test!(haystack, "B", None, Some(9));
    }
    #[test]
    fn test_small_needle() {
        let haystack = "111 a 111b";
        rsearch_test!(haystack, "a", Some(4));
        rsearch_test!(haystack, "a 111", Some(4));
        rsearch_test!(haystack, "A", None, Some(4));
        rsearch_test!(haystack, "A 111", None, Some(4));
    }
}

#[cfg(test)]
mod func_search_indices {
    use naive_opt::{string_search_indices, string_search_indices_bytes};
    use naive_opt::{
        string_search_indices_bytes_ignore_ascii_case, string_search_indices_ignore_ascii_case,
    };
    #[test]
    fn test_search_indices_0() {
        let haystack = "aaa 1 aaab";
        let needle = "a";
        let mut m = string_search_indices(haystack, needle);
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
        let mut m = string_search_indices_bytes(haystack, needle);
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
        let mut m = string_search_indices_ignore_ascii_case(haystack, needle);
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
        let mut m = string_search_indices_bytes_ignore_ascii_case(haystack, needle);
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
mod func_rsearch_indices {
    use naive_opt::{string_rsearch_indices, string_rsearch_indices_bytes};
    use naive_opt::{
        string_rsearch_indices_bytes_ignore_ascii_case, string_rsearch_indices_ignore_ascii_case,
    };
    #[test]
    fn test_rsearch_indices_0() {
        let haystack = "aaa 1 aaab";
        let needle = "a";
        let mut m = string_rsearch_indices(haystack, needle);
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
        let mut m = string_rsearch_indices_bytes(haystack, needle);
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
        let mut m = string_rsearch_indices_ignore_ascii_case(haystack, needle);
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
        let mut m = string_rsearch_indices_bytes_ignore_ascii_case(haystack, needle);
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
mod func_str_str_large {
    use naive_opt::{string_rsearch, string_rsearch_bytes};
    use naive_opt::{string_rsearch_bytes_ignore_ascii_case, string_rsearch_ignore_ascii_case};
    use naive_opt::{string_search, string_search_bytes};
    use naive_opt::{string_search_bytes_ignore_ascii_case, string_search_ignore_ascii_case};
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
