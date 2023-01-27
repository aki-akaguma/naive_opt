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
    #[test]
    fn test_diskstats() {
        let haystack = "   7       0 loop0 514 0 3326 388 0 0 0 0 0 237 350 0 0 0 0
   7       1 loop1 1177257 0 2356814 473613 0 0 0 0 0 27515 178091 0 0 0 0
   7       2 loop2 752 0 3770 608 0 0 0 0 0 221 566 0 0 0 0
   7       3 loop3 2026 0 6314 1244 0 0 0 0 0 490 674 0 0 0 0
   7       4 loop4 57222 0 115290 25131 0 0 0 0 0 2783 6502 0 0 0 0
   7       5 loop5 1212 0 4696 641 0 0 0 0 0 418 561 0 0 0 0
   7       6 loop6 24 0 102 37 0 0 0 0 0 25 30 0 0 0 0
   7       7 loop7 3074 0 6994 429 0 0 0 0 0 847 105 0 0 0 0
   7       8 loop8 240 0 2774 360 0 0 0 0 0 112 326 0 0 0 0
   7       9 loop9 208 0 2708 282 0 0 0 0 0 85 256 0 0 0 0
   7      10 loop10 1 0 2 0 0 0 0 0 0 1 0 0 0 0 0
   7      11 loop11 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7      12 loop12 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7      13 loop13 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7      14 loop14 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7      15 loop15 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   8       0 sda 1216630 2162899 47845388 377918 117916 568658 11948368 78877 0 644078 105581 24 0 70336512 1290
   8       1 sda1 28 0 224 10 0 0 0 0 0 15 0 0 0 0 0
   8       2 sda2 1216548 2162899 47842100 377882 107816 568658 11948368 72056 0 643921 104339 24 0 70336512 1290
   8      16 sdb 11460059 3144618 236280768 24092509 974696 4791235 128815456 4756499 0 5857710 25793982 0 0 0 0
   8      17 sdb1 11086187 2334404 196841618 21934339 541757 4344642 68116064 2106205 0 5252221 21296637 0 0 0 0
   8      20 sdb4 3 0 12 38 0 0 0 0 0 6 37 0 0 0 0
   8      21 sdb5 373830 810214 39436290 2148419 420954 446593 60699392 2367123 0 640920 4210397 0 0 0 0
  11       0 sr0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   8      32 sdc 237422 269250 8040160 84541 281916 819040 23564712 156439 0 283390 81961 0 0 0 0
   8      33 sdc1 28 0 224 7 0 0 0 0 0 21 1 0 0 0 0
   8      34 sdc2 63 0 4960 31 0 0 0 0 0 39 9 0 0 0 0
   8      35 sdc3 237283 269250 8032032 84488 262771 819040 23564712 143869 0 283049 78503 0 0 0 0
   2       0 fd0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 253       0 dm-0 1263557 0 30782434 806632 683430 0 12014672 1184232 0 440664 1990864 0 0 0 0
 253       1 dm-1 508176 0 8027698 264428 1096966 0 23713128 1440559 0 282957 1704987 0 0 0 0
 253       2 dm-2 13468842 0 196839394 40823058 4885492 0 68142752 39819429 0 5273386 80642487 0 0 0 0
   8      80 sdf 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   8      96 sdg 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   8      64 sde 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   8      48 sdd 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
".to_string();
        search_test!(&haystack, "\n", Some(59));
        search_test!(&haystack[60..], "\n", Some(74));
        search_test!(&haystack[(60+75)..], "\n", Some(59));
        search_test!(&haystack[(60+75+60)..], "\n", Some(61));
        search_test!(&haystack, "sda2", Some(1100));
        search_test!(&haystack, "sdc3", Some(1847));
        search_test!(&haystack, "sdd", Some(2426));
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
