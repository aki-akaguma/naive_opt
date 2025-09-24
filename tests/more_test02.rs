use naive_opt::{
    includes_bytes, includes_bytes_ignore_ascii_case, string_rsearch_bytes,
    string_rsearch_bytes_ignore_ascii_case, string_search_bytes,
    string_search_bytes_ignore_ascii_case, Search, SearchBytes,
};

// Test cases for empty haystack and non-empty needle
#[test]
fn test_empty_haystack_non_empty_needle() {
    let haystack_str = "";
    let needle_str = "a";
    assert_eq!(haystack_str.search(needle_str), None);
    assert_eq!(haystack_str.rsearch(needle_str), None);
    assert!(!haystack_str.includes(needle_str));
    assert_eq!(haystack_str.search_ignore_ascii_case(needle_str), None);
    assert_eq!(haystack_str.rsearch_ignore_ascii_case(needle_str), None);
    assert!(!haystack_str.includes_ignore_ascii_case(needle_str));

    let haystack_bytes: &[u8] = b"";
    let needle_bytes: &[u8] = b"a";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), None);
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), None);
    assert!(!includes_bytes(haystack_bytes, needle_bytes));
    assert_eq!(
        string_search_bytes_ignore_ascii_case(haystack_bytes, needle_bytes),
        None
    );
    assert_eq!(
        string_rsearch_bytes_ignore_ascii_case(haystack_bytes, needle_bytes),
        None
    );
    assert!(!includes_bytes_ignore_ascii_case(
        haystack_bytes,
        needle_bytes
    ));
}

// Test cases for non-empty haystack and empty needle
#[test]
fn test_non_empty_haystack_empty_needle() {
    let haystack_str = "abc";
    let needle_str = "";
    assert_eq!(haystack_str.search(needle_str), Some(0));
    assert_eq!(haystack_str.rsearch(needle_str), Some(3));
    assert!(haystack_str.includes(needle_str));
    assert_eq!(haystack_str.search_ignore_ascii_case(needle_str), Some(0));
    assert_eq!(haystack_str.rsearch_ignore_ascii_case(needle_str), Some(3));
    assert!(haystack_str.includes_ignore_ascii_case(needle_str));

    let haystack_bytes: &[u8] = b"abc";
    let needle_bytes: &[u8] = b"";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(0));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(3));
    assert!(includes_bytes(haystack_bytes, needle_bytes));
    assert_eq!(
        string_search_bytes_ignore_ascii_case(haystack_bytes, needle_bytes),
        Some(0)
    );
    assert_eq!(
        string_rsearch_bytes_ignore_ascii_case(haystack_bytes, needle_bytes),
        Some(3)
    );
    assert!(includes_bytes_ignore_ascii_case(
        haystack_bytes,
        needle_bytes
    ));
}

// Test cases for needle longer than haystack
#[test]
fn test_needle_longer_than_haystack() {
    let haystack_str = "abc";
    let needle_str = "abcd";
    assert_eq!(haystack_str.search(needle_str), None);
    assert_eq!(haystack_str.rsearch(needle_str), None);
    assert!(!haystack_str.includes(needle_str));
    assert_eq!(haystack_str.search_ignore_ascii_case(needle_str), None);
    assert_eq!(haystack_str.rsearch_ignore_ascii_case(needle_str), None);
    assert!(!haystack_str.includes_ignore_ascii_case(needle_str));

    let haystack_bytes: &[u8] = b"abc";
    let needle_bytes: &[u8] = b"abcd";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), None);
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), None);
    assert!(!includes_bytes(haystack_bytes, needle_bytes));
    assert_eq!(
        string_search_bytes_ignore_ascii_case(haystack_bytes, needle_bytes),
        None
    );
    assert_eq!(
        string_rsearch_bytes_ignore_ascii_case(haystack_bytes, needle_bytes),
        None
    );
    assert!(!includes_bytes_ignore_ascii_case(
        haystack_bytes,
        needle_bytes
    ));
}

// Test cases for single character/byte needle
#[test]
fn test_single_char_needle() {
    let haystack_str = "banana";
    let needle_str = "a";
    assert_eq!(haystack_str.search(needle_str), Some(1));
    assert_eq!(haystack_str.rsearch(needle_str), Some(5));
    assert!(haystack_str.includes(needle_str));
    assert_eq!(haystack_str.search_ignore_ascii_case("A"), Some(1));
    assert_eq!(haystack_str.rsearch_ignore_ascii_case("A"), Some(5));
    assert!(haystack_str.includes_ignore_ascii_case("A"));

    let haystack_bytes: &[u8] = b"banana";
    let needle_bytes: &[u8] = b"a";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(1));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(5));
    assert!(includes_bytes(haystack_bytes, needle_bytes));
    assert_eq!(
        string_search_bytes_ignore_ascii_case(haystack_bytes, &b"A"[..]),
        Some(1)
    );
    assert_eq!(
        string_rsearch_bytes_ignore_ascii_case(haystack_bytes, &b"A"[..]),
        Some(5)
    );
    assert!(includes_bytes_ignore_ascii_case(haystack_bytes, &b"A"[..]));
}

// Test cases for multiple occurrences of the needle
#[test]
fn test_multiple_occurrences() {
    let haystack_str = "ababab";
    let needle_str = "aba";
    assert_eq!(haystack_str.search(needle_str), Some(0));
    assert_eq!(haystack_str.rsearch(needle_str), Some(2));
    assert!(haystack_str.includes(needle_str));

    let v: Vec<_> = haystack_str.search_indices(needle_str).collect();
    assert_eq!(v, [(0, "aba")]);

    let v: Vec<_> = haystack_str.rsearch_indices(needle_str).collect();
    assert_eq!(v, [(2, "aba")]);

    let haystack_bytes: &[u8] = b"ababab";
    let needle_bytes: &[u8] = b"aba";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(0));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(2));
    assert!(includes_bytes(haystack_bytes, needle_bytes));

    let v: Vec<_> = haystack_bytes.search_indices_bytes(needle_bytes).collect();
    assert_eq!(v, [(0, &b"aba"[..])]);

    let v: Vec<_> = haystack_bytes.rsearch_indices_bytes(needle_bytes).collect();
    assert_eq!(v, [(2, &b"aba"[..])]);
}

// Test cases for overlapping matches
#[test]
fn test_overlapping_matches() {
    let haystack_str = "aaaaa";
    let needle_str = "aaa";
    assert_eq!(haystack_str.search(needle_str), Some(0));
    assert_eq!(haystack_str.rsearch(needle_str), Some(2));
    assert!(haystack_str.includes(needle_str));

    let v: Vec<_> = haystack_str.search_indices(needle_str).collect();
    assert_eq!(v, [(0, "aaa")]); // Only the first non-overlapping match

    let v: Vec<_> = haystack_str.rsearch_indices(needle_str).collect();
    assert_eq!(v, [(2, "aaa")]); // Only the last non-overlapping match

    let haystack_bytes: &[u8] = b"aaaaa";
    let needle_bytes: &[u8] = b"aaa";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), Some(0));
    assert_eq!(string_rsearch_bytes(haystack_bytes, needle_bytes), Some(2));
    assert!(includes_bytes(haystack_bytes, needle_bytes));

    let v: Vec<_> = haystack_bytes.search_indices_bytes(needle_bytes).collect();
    assert_eq!(v, [(0, &b"aaa"[..])]);

    let v: Vec<_> = haystack_bytes.rsearch_indices_bytes(needle_bytes).collect();
    assert_eq!(v, [(2, &b"aaa"[..])]);
}

// Test cases for mixed case and ignore ascii case
#[test]
fn test_mixed_case_ignore_ascii_case() {
    let haystack_str = "AbcDEfG";
    let needle_str = "bCd";
    assert_eq!(haystack_str.search(needle_str), None);
    assert_eq!(haystack_str.search_ignore_ascii_case(needle_str), Some(1));
    assert!(haystack_str.includes_ignore_ascii_case(needle_str));

    let v: Vec<_> = haystack_str
        .search_indices_ignore_ascii_case(needle_str)
        .collect();
    assert_eq!(v, [(1, "bcD")]);

    let haystack_bytes: &[u8] = b"AbcDEfG";
    let needle_bytes: &[u8] = b"bCd";
    assert_eq!(string_search_bytes(haystack_bytes, needle_bytes), None);
    assert_eq!(
        string_search_bytes_ignore_ascii_case(haystack_bytes, needle_bytes),
        Some(1)
    );
    assert!(includes_bytes_ignore_ascii_case(
        haystack_bytes,
        needle_bytes
    ));

    let v: Vec<_> = haystack_bytes
        .search_indices_bytes_ignore_ascii_case(needle_bytes)
        .collect();
    assert_eq!(v, [(1, &b"bcD"[..])]);
}
