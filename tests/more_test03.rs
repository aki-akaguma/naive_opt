use naive_opt::{includes_bytes, string_rsearch_bytes, string_search_bytes, Search, SearchBytes};

// Test with Unicode characters
#[test]
fn test_unicode_characters() {
    let haystack = "你好世界你好";
    let needle = "世界";
    assert_eq!(haystack.search(needle), Some(6));
    assert_eq!(haystack.rsearch(needle), Some(6));
    assert!(haystack.includes(needle));

    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(6));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(6));
    assert!(includes_bytes(haystack_bytes, needle_bytes));

    // Case-insensitive search with Unicode (should behave as case-sensitive for non-ASCII)
    assert_eq!(haystack.search_ignore_ascii_case(needle), Some(6));
    assert!(haystack.includes_ignore_ascii_case(needle));
}

// Test with longer strings
#[test]
fn test_longer_strings() {
    let haystack = "a".repeat(1000) + "b" + &"a".repeat(1000);
    let needle = "b";
    assert_eq!(haystack.search(needle), Some(1000));
    assert_eq!(haystack.rsearch(needle), Some(1000));
    assert!(haystack.includes(needle));

    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    assert_eq!(
        string_search_bytes(haystack_bytes, needle_bytes),
        Some(1000)
    );
    assert_eq!(
        string_rsearch_bytes(haystack_bytes, needle_bytes),
        Some(1000)
    );
    assert!(includes_bytes(haystack_bytes, needle_bytes));

    let needle_long = "a".repeat(500) + "b" + &"a".repeat(500);
    assert_eq!(haystack.search(&needle_long), Some(500));
    assert!(haystack.includes(&needle_long));
}

// Test with char as needle
#[test]
fn test_char_needle() {
    let haystack = "abacaba";
    let needle = 'c';
    assert_eq!(haystack.search(needle), Some(3));
    assert_eq!(haystack.rsearch(needle), Some(3));
    assert!(haystack.includes(needle));

    let needle_upper = 'C';
    assert_eq!(haystack.search_ignore_ascii_case(needle_upper), Some(3));
    assert!(haystack.includes_ignore_ascii_case(needle_upper));
}

// Test search_indices with multiple overlapping matches
#[test]
fn test_search_indices_overlapping() {
    let haystack = "aaaaa";
    let needle = "aa";
    let v: Vec<_> = haystack.search_indices(needle).collect();
    assert_eq!(v, [(0, "aa"), (2, "aa")]); // Non-overlapping matches

    let v: Vec<_> = haystack.rsearch_indices(needle).collect();
    assert_eq!(v, [(3, "aa"), (1, "aa")]); // Non-overlapping matches in reverse
}

// Test search_indices_ignore_ascii_case with multiple overlapping matches
#[test]
fn test_search_indices_ignore_ascii_case_overlapping() {
    let haystack = "aAaAa";
    let needle = "aa";
    let v: Vec<_> = haystack.search_indices_ignore_ascii_case(needle).collect();
    assert_eq!(v, [(0, "aA"), (2, "aA")]);

    let v: Vec<_> = haystack.rsearch_indices_ignore_ascii_case(needle).collect();
    assert_eq!(v, [(3, "Aa"), (1, "Aa")]);
}

// Test search_indices_bytes with multiple overlapping matches
#[test]
fn test_search_indices_bytes_overlapping() {
    let haystack: &[u8] = b"aaaaa";
    let needle: &[u8] = b"aa";
    let v: Vec<_> = haystack.search_indices_bytes(needle).collect();
    assert_eq!(v, [(0, &b"aa"[..]), (2, &b"aa"[..])]);

    let v: Vec<_> = haystack.rsearch_indices_bytes(needle).collect();
    assert_eq!(v, [(3, &b"aa"[..]), (1, &b"aa"[..])]);
}

// Test search_indices_bytes_ignore_ascii_case with multiple overlapping matches
#[test]
fn test_search_indices_bytes_ignore_ascii_case_overlapping() {
    let haystack: &[u8] = b"aAaAa";
    let needle: &[u8] = b"aa";
    let v: Vec<_> = haystack
        .search_indices_bytes_ignore_ascii_case(needle)
        .collect();
    assert_eq!(v, [(0, &b"aA"[..]), (2, &b"aA"[..])]);

    let v: Vec<_> = haystack
        .rsearch_indices_bytes_ignore_ascii_case(needle)
        .collect();
    assert_eq!(v, [(3, &b"Aa"[..]), (1, &b"Aa"[..])]);
}
