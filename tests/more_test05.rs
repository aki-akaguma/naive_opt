use naive_opt::{
    includes_bytes, string_rsearch_bytes, string_search_bytes, Search, SearchBytes, SearchIn,
    SearchInBytes,
};

// Test Unicode characters with varying lengths and positions
#[test]
fn test_unicode_variations() {
    let haystack = "你好世界 hello world 你好世界";
    let needle_short = "好";
    let needle_long = "世界 hello";
    let needle_not_found = "Rust";

    assert_eq!(haystack.search(needle_short), Some(3));
    assert_eq!(haystack.rsearch(needle_short), Some(28));
    assert!(haystack.includes(needle_short));

    assert_eq!(haystack.search(needle_long), Some(6));
    assert_eq!(haystack.rsearch(needle_long), Some(6));
    assert!(haystack.includes(needle_long));

    assert_eq!(haystack.search(needle_not_found), None);
    assert!(!haystack.includes(needle_not_found));

    let haystack_bytes = haystack.as_bytes();
    let needle_short_bytes = needle_short.as_bytes();
    let needle_long_bytes = needle_long.as_bytes();
    let needle_not_found_bytes = needle_not_found.as_bytes();

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_short_bytes),
        Some(3)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_short_bytes),
        Some(28)
    );
    assert!(includes_bytes(haystack_bytes, needle_short_bytes));

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_long_bytes),
        Some(6)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_long_bytes),
        Some(6)
    );
    assert!(includes_bytes(haystack_bytes, needle_long_bytes));

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_not_found_bytes),
        None
    );
    assert!(!includes_bytes(haystack_bytes, needle_not_found_bytes));
}

// Test haystack and needle with only whitespace characters
#[test]
fn test_whitespace_only() {
    let haystack = "   \t  ";
    let needle = "\t";
    assert_eq!(haystack.search(needle), Some(3));
    assert_eq!(haystack.rsearch(needle), Some(3));
    assert!(haystack.includes(needle));

    let haystack_bytes: &[u8] = b"   \t  ";
    let needle_bytes: &[u8] = b"\t";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(3));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(3));
    assert!(includes_bytes(haystack_bytes, needle_bytes));
}

// Test haystack and needle with mixed ASCII and Unicode characters
#[test]
fn test_mixed_ascii_unicode() {
    let haystack = "Rust你好World";
    let needle_ascii = "World";
    let needle_unicode = "你好";
    let needle_mixed = "t你好W";

    assert_eq!(haystack.search(needle_ascii), Some(10));
    assert_eq!(haystack.rsearch(needle_ascii), Some(10));
    assert!(haystack.includes(needle_ascii));

    assert_eq!(haystack.search(needle_unicode), Some(4));
    assert_eq!(haystack.rsearch(needle_unicode), Some(4));
    assert!(haystack.includes(needle_unicode));

    assert_eq!(haystack.search(needle_mixed), Some(3));
    assert_eq!(haystack.rsearch(needle_mixed), Some(3));
    assert!(haystack.includes(needle_mixed));

    let haystack_bytes = haystack.as_bytes();
    let needle_ascii_bytes = needle_ascii.as_bytes();
    let needle_unicode_bytes = needle_unicode.as_bytes();
    let needle_mixed_bytes = needle_mixed.as_bytes();

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_ascii_bytes),
        Some(10)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_ascii_bytes),
        Some(10)
    );
    assert!(includes_bytes(haystack_bytes, needle_ascii_bytes));

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_unicode_bytes),
        Some(4)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_unicode_bytes),
        Some(4)
    );
    assert!(includes_bytes(haystack_bytes, needle_unicode_bytes));

    assert_eq!(
        string_search_bytes(haystack_bytes, needle_mixed_bytes),
        Some(3)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_mixed_bytes),
        Some(3)
    );
    assert!(includes_bytes(haystack_bytes, needle_mixed_bytes));
}

// Test SearchIn and SearchInBytes traits with empty patterns
#[test]
fn test_search_in_empty_patterns() {
    let haystack = "test";
    let empty_str = "";
    let empty_string = "".to_string();
    let empty_bytes: &[u8] = b"";

    assert_eq!(<&str as SearchIn>::search_in(&empty_str, haystack), Some(0));
    assert!(<&str as SearchIn>::includes_in(&empty_str, haystack));
    assert_eq!(
        <&String as SearchIn>::search_in(&&empty_string, haystack),
        Some(0)
    );
    assert!(<&String as SearchIn>::includes_in(&&empty_string, haystack));

    let haystack_bytes: &[u8] = b"test";
    assert_eq!(
        <&str as SearchInBytes>::search_in(&empty_str, haystack_bytes),
        Some(0)
    );
    assert!(<&str as SearchInBytes>::includes_in(
        &empty_str,
        haystack_bytes
    ));
    assert_eq!(
        <&String as SearchInBytes>::search_in(&&empty_string, haystack_bytes),
        Some(0)
    );
    assert!(<&String as SearchInBytes>::includes_in(
        &&empty_string,
        haystack_bytes
    ));
    assert_eq!(
        <&[u8] as SearchInBytes>::search_in(&empty_bytes, haystack_bytes),
        Some(0)
    );
    assert!(<&[u8] as SearchInBytes>::includes_in(
        &empty_bytes,
        haystack_bytes
    ));
}

#[test]
fn test_empty_pattern_indices() {
    let haystack = "abc";
    let empty_needle = "";

    let v: Vec<_> = haystack.search_indices(empty_needle).collect();
    assert_eq!(v, []);

    let v: Vec<_> = haystack.rsearch_indices(empty_needle).collect();
    assert_eq!(v, []);

    let haystack_bytes: &[u8] = b"abc";
    let empty_needle_bytes: &[u8] = b"";

    let v: Vec<_> = haystack_bytes
        .search_indices_bytes(empty_needle_bytes)
        .collect();
    assert_eq!(v, []);

    let v: Vec<_> = haystack_bytes
        .rsearch_indices_bytes(empty_needle_bytes)
        .collect();
    assert_eq!(v, []);
}
