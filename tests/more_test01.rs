use naive_opt::{
    includes_bytes, includes_bytes_ignore_ascii_case, string_rsearch_bytes, string_search_bytes,
    Search, SearchBytes, SearchIn, SearchInBytes,
};

#[test]
fn test_includes_bytes() {
    let haystack: &[u8] = b"abcde";
    let needle: &[u8] = b"bcd";
    assert!(includes_bytes(haystack, needle));
    assert!(!includes_bytes(haystack, &b"ace"[..]));
    assert!(includes_bytes(haystack, &b"abcde"[..]));
    assert!(includes_bytes(haystack, &b""[..]));
    assert!(includes_bytes(&b""[..], &b""[..]));
    assert!(!includes_bytes(&b""[..], &b"a"[..]));
}

#[test]
fn test_includes_bytes_ignore_ascii_case() {
    let haystack: &[u8] = b"abcDe";
    let needle: &[u8] = b"BCD";
    assert!(includes_bytes_ignore_ascii_case(haystack, needle));
    assert!(!includes_bytes_ignore_ascii_case(haystack, &b"ace"[..]));
    assert!(includes_bytes_ignore_ascii_case(haystack, &b"ABCDE"[..]));
    assert!(includes_bytes_ignore_ascii_case(haystack, &b""[..]));
    assert!(includes_bytes_ignore_ascii_case(&b""[..], &b""[..]));
    assert!(!includes_bytes_ignore_ascii_case(&b""[..], &b"a"[..]));
}

#[test]
fn test_string_search_bytes() {
    let haystack: &[u8] = b"ababa";
    let needle: &[u8] = b"aba";
    assert_eq!(string_search_bytes(haystack, needle), Some(0));
    assert_eq!(string_search_bytes(haystack, &b"bab"[..]), Some(1));
    assert_eq!(string_search_bytes(haystack, &b"c"[..]), None);
}

#[test]
fn test_string_rsearch_bytes() {
    let haystack: &[u8] = b"ababa";
    let needle: &[u8] = b"aba";
    assert_eq!(string_rsearch_bytes(haystack, needle), Some(2));
    assert_eq!(string_rsearch_bytes(haystack, &b"bab"[..]), Some(1));
    assert_eq!(string_rsearch_bytes(haystack, &b"c"[..]), None);
}

#[test]
fn test_search_trait_for_str() {
    let haystack = "abcde";
    assert_eq!(haystack.search("bcd"), Some(1));
    assert!(haystack.includes("bcd"));
    assert_eq!(haystack.search_ignore_ascii_case("BCD"), Some(1));
    assert!(haystack.includes_ignore_ascii_case("BCD"));
}

#[test]
fn test_search_bytes_trait_for_slice_u8() {
    let haystack: &[u8] = b"abcde";
    let needle: &[u8] = b"bcd";
    assert_eq!(haystack.search_bytes(needle), Some(1));
    assert!(haystack.includes_bytes(needle));
    assert_eq!(
        haystack.search_bytes_ignore_ascii_case(&b"BCD"[..]),
        Some(1)
    );
    assert!(haystack.includes_bytes_ignore_ascii_case(&b"BCD"[..]));
}

#[test]
fn test_search_in_trait_for_str() {
    let haystack = "abcde";
    let needle = "bcd";
    assert_eq!(<&str as SearchIn>::search_in(&needle, haystack), Some(1));
    assert!(<&str as SearchIn>::includes_in(&needle, haystack));
    assert_eq!(
        <&str as SearchIn>::search_in_ignore_ascii_case(&needle, "ABCDE",),
        Some(1)
    );
    assert!(<&str as SearchIn>::includes_in_ignore_ascii_case(
        &needle, "ABCDE",
    ));
}

#[test]
fn test_search_in_bytes_trait_for_slice_u8() {
    let haystack: &[u8] = b"abcde";
    let needle: &[u8] = b"bcd";
    assert_eq!(
        <&[u8] as SearchInBytes>::search_in(&needle, haystack),
        Some(1)
    );
    assert!(<&[u8] as SearchInBytes>::includes_in(&needle, haystack));
    assert_eq!(
        <&[u8] as SearchInBytes>::search_in_ignore_ascii_case(&needle, &b"ABCDE"[..],),
        Some(1)
    );
    assert!(<&[u8] as SearchInBytes>::includes_in_ignore_ascii_case(
        &needle,
        &b"ABCDE"[..],
    ));
}

#[test]
fn test_search_indices() {
    let haystack = "ababa";
    let v: Vec<_> = haystack.search_indices("aba").collect();
    assert_eq!(v, [(0, "aba")]);
}

#[test]
fn test_rsearch_indices() {
    let haystack = "ababa";
    let v: Vec<_> = haystack.rsearch_indices("aba").collect();
    assert_eq!(v, [(2, "aba")]);
}

#[test]
fn test_search_indices_bytes() {
    let haystack: &[u8] = b"ababa";
    let v: Vec<_> = haystack.search_indices_bytes(&b"aba"[..]).collect();
    assert_eq!(v, [(0, &b"aba"[..])]);
}

#[test]
fn test_rsearch_indices_bytes() {
    let haystack: &[u8] = b"ababa";
    let v: Vec<_> = haystack.rsearch_indices_bytes(&b"aba"[..]).collect();
    assert_eq!(v, [(2, &b"aba"[..])]);
}

#[test]
fn test_search_indices_ignore_ascii_case() {
    let haystack = "abAbA";
    let v: Vec<_> = haystack.search_indices_ignore_ascii_case("aba").collect();
    assert_eq!(v, [(0, "abA")]);
}

#[test]
fn test_rsearch_indices_ignore_ascii_case() {
    let haystack = "abAbA";
    let v: Vec<_> = haystack.rsearch_indices_ignore_ascii_case("aba").collect();
    assert_eq!(v, [(2, "AbA")]);
}

#[test]
fn test_search_indices_bytes_ignore_ascii_case() {
    let haystack: &[u8] = b"abAbA";
    let v: Vec<_> = haystack
        .search_indices_bytes_ignore_ascii_case(&b"aba"[..])
        .collect();
    assert_eq!(v, [(0, &b"abA"[..])]);
}

#[test]
fn test_rsearch_indices_bytes_ignore_ascii_case() {
    let haystack: &[u8] = b"abAbA";
    let v: Vec<_> = haystack
        .rsearch_indices_bytes_ignore_ascii_case(&b"aba"[..])
        .collect();
    assert_eq!(v, [(2, &b"AbA"[..])]);
}
