use naive_opt::{
    includes_bytes, string_rsearch_bytes, string_search_bytes, Search, SearchBytes, SearchIn,
    SearchInBytes,
};

// Test needle at the very beginning or end of the haystack
#[test]
fn test_needle_at_extremes() {
    let haystack = "abcdefg";
    let needle_start = "abc";
    let needle_end = "efg";
    let needle_middle = "cde";

    assert_eq!(haystack.search(needle_start), Some(0));
    assert_eq!(haystack.rsearch(needle_start), Some(0));
    assert!(haystack.includes(needle_start));

    assert_eq!(haystack.search(needle_end), Some(4));
    assert_eq!(haystack.rsearch(needle_end), Some(4));
    assert!(haystack.includes(needle_end));

    assert_eq!(haystack.search(needle_middle), Some(2));
    assert_eq!(haystack.rsearch(needle_middle), Some(2));
    assert!(haystack.includes(needle_middle));

    let haystack_bytes: &[u8] = b"abcdefg";
    let needle_start_bytes: &[u8] = b"abc";
    let needle_end_bytes: &[u8] = b"efg";
    let needle_middle_bytes: &[u8] = b"cde";

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_start_bytes),
        Some(0)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_start_bytes),
        Some(0)
    );
    assert!(includes_bytes(haystack_bytes, needle_start_bytes));

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_end_bytes),
        Some(4)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_end_bytes),
        Some(4)
    );
    assert!(includes_bytes(haystack_bytes, needle_end_bytes));

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_middle_bytes),
        Some(2)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_middle_bytes),
        Some(2)
    );
    assert!(includes_bytes(haystack_bytes, needle_middle_bytes));
}

// Test haystack and needle with special characters
#[test]
fn test_special_characters() {
    let haystack = "!@#$%^&*()_+";
    let needle = "#$%^";
    assert_eq!(haystack.search(needle), Some(2));
    assert_eq!(haystack.rsearch(needle), Some(2));
    assert!(haystack.includes(needle));

    let haystack_bytes: &[u8] = b"!@#$%^&*()_+";
    let needle_bytes: &[u8] = b"#$%^";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(2));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(2));
    assert!(includes_bytes(haystack_bytes, needle_bytes));
}

// Test repeated single-character needles
#[test]
fn test_repeated_single_char_needle() {
    let haystack = "aaaaa";
    let needle = "a";
    assert_eq!(haystack.search(needle), Some(0));
    assert_eq!(haystack.rsearch(needle), Some(4));
    assert!(haystack.includes(needle));

    let v: Vec<_> = haystack.search_indices(needle).collect();
    assert_eq!(v, [(0, "a"), (1, "a"), (2, "a"), (3, "a"), (4, "a")]);

    let v: Vec<_> = haystack.rsearch_indices(needle).collect();
    assert_eq!(v, [(4, "a"), (3, "a"), (2, "a"), (1, "a"), (0, "a")]);

    let haystack_bytes: &[u8] = b"aaaaa";
    let needle_bytes: &[u8] = b"a";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(0));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(4));
    assert!(includes_bytes(haystack_bytes, needle_bytes));

    let v: Vec<_> = haystack_bytes.search_indices_bytes(needle_bytes).collect();
    assert_eq!(
        v,
        [
            (0, &b"a"[..]),
            (1, &b"a"[..]),
            (2, &b"a"[..]),
            (3, &b"a"[..]),
            (4, &b"a"[..])
        ]
    );

    let v: Vec<_> = haystack_bytes.rsearch_indices_bytes(needle_bytes).collect();
    assert_eq!(
        v,
        [
            (4, &b"a"[..]),
            (3, &b"a"[..]),
            (2, &b"a"[..]),
            (1, &b"a"[..]),
            (0, &b"a"[..])
        ]
    );
}

// Test longer needles that are substrings of the haystack
#[test]
fn test_longer_substring_needle() {
    let haystack = "abcdefghijklmnopqrstuvwxyz";
    let needle = "fghijkl";
    assert_eq!(haystack.search(needle), Some(5));
    assert_eq!(haystack.rsearch(needle), Some(5));
    assert!(haystack.includes(needle));

    let haystack_bytes: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let needle_bytes: &[u8] = b"fghijkl";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(5));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(5));
    assert!(includes_bytes(haystack_bytes, needle_bytes));
}

// Test SearchIn and SearchInBytes traits with different types of needles
#[test]
fn test_search_in_with_string_and_char() {
    let haystack = "testing string and char needles";

    // Test with &String as needle
    let needle_string = "string".to_string();
    assert_eq!(
        <&String as SearchIn>::search_in(&&needle_string, haystack),
        Some(8)
    );
    assert!(<&String as SearchIn>::includes_in(
        &&needle_string,
        haystack
    ));

    // Test with char as needle
    let needle_char = 'c';
    assert_eq!(
        <char as SearchIn>::search_in(&needle_char, haystack),
        Some(19)
    );
    assert!(<char as SearchIn>::includes_in(&needle_char, haystack));

    // Test with &String as needle, ignore ascii case
    let needle_string_iac = "STRING".to_string();
    assert_eq!(
        <&String as SearchIn>::search_in_ignore_ascii_case(&&needle_string_iac, haystack),
        Some(8)
    );
    assert!(<&String as SearchIn>::includes_in_ignore_ascii_case(
        &&needle_string_iac,
        haystack
    ));

    // Test with char as needle, ignore ascii case
    let needle_char_iac = 'C';
    assert_eq!(
        <char as SearchIn>::search_in_ignore_ascii_case(&needle_char_iac, haystack),
        Some(19)
    );
    assert!(<char as SearchIn>::includes_in_ignore_ascii_case(
        &needle_char_iac,
        haystack
    ));
}

#[test]
fn test_search_in_bytes_with_string_and_char() {
    let haystack: &[u8] = b"testing string and char needles";

    // Test with &String as needle
    let needle_string = "string".to_string();
    assert_eq!(
        <&String as SearchInBytes>::search_in(&&needle_string, haystack),
        Some(8)
    );
    assert!(<&String as SearchInBytes>::includes_in(
        &&needle_string,
        haystack
    ));

    // Test with char as needle
    let needle_char = 'c';
    assert_eq!(
        <char as SearchInBytes>::search_in(&needle_char, haystack),
        Some(19)
    );
    assert!(<char as SearchInBytes>::includes_in(&needle_char, haystack));

    // Test with &String as needle, ignore ascii case
    let needle_string_iac = "STRING".to_string();
    assert_eq!(
        <&String as SearchInBytes>::search_in_ignore_ascii_case(&&needle_string_iac, haystack),
        Some(8)
    );
    assert!(<&String as SearchInBytes>::includes_in_ignore_ascii_case(
        &&needle_string_iac,
        haystack
    ));

    // Test with char as needle, ignore ascii case
    let needle_char_iac = 'C';
    assert_eq!(
        <char as SearchInBytes>::search_in_ignore_ascii_case(&needle_char_iac, haystack),
        Some(19)
    );
    assert!(<char as SearchInBytes>::includes_in_ignore_ascii_case(
        &needle_char_iac,
        haystack
    ));
}
