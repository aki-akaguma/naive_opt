macro_rules! search_test {
    ($haystack:expr, $needle:expr, $result:expr) => {
        let r = $haystack.search($needle);
        assert_eq!(r, $result);
    };
}

#[cfg(test)]
mod trait_str_str {
    use naive_opt::Search;
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
mod trait_str_string {
    use naive_opt::Search;
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
mod trait_string_str {
    use naive_opt::Search;
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
mod trait_string_string {
    use naive_opt::Search;
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
mod trai_search_indices {
    use naive_opt::Search;
    #[test]
    fn test_search_indices_0() {
        let haystack = "111 a 111b";
        let needle = "1";
        let mut m = haystack.search_indices(needle);
        assert_eq!(m.next(), Some((0, "1")));
        assert_eq!(m.next(), Some((1, "1")));
        assert_eq!(m.next(), Some((2, "1")));
        assert_eq!(m.next(), Some((6, "1")));
        assert_eq!(m.next(), Some((7, "1")));
        assert_eq!(m.next(), Some((8, "1")));
        assert_eq!(m.next(), None);
    }
}
