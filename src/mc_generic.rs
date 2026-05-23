pub(crate) trait SearchStrategy {
    fn pivot(nee_len: usize) -> usize;
}

pub(crate) struct FirstByte;
impl SearchStrategy for FirstByte {
    #[inline(always)]
    fn pivot(_: usize) -> usize {
        0
    }
}

pub(crate) struct LastByte;
impl SearchStrategy for LastByte {
    #[inline(always)]
    fn pivot(nee_len: usize) -> usize {
        nee_len - 1
    }
}

#[inline]
pub(crate) fn generic_search<S: SearchStrategy>(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    if nee_len == 0 {
        return Some(0);
    }
    if hay_len < nee_len {
        return None;
    }

    let pivot = S::pivot(nee_len);
    let pick_byte = nee_bytes[pivot];

    let search_slice = &hay_bytes[pivot..hay_len - nee_len + pivot + 1];

    for m in ::memx::iter::memchr_iter(search_slice, pick_byte) {
        let st = m;
        if ::memx::memeq(nee_bytes, &hay_bytes[st..st + nee_len]) {
            return Some(st);
        }
    }
    None
}

#[inline]
pub(crate) fn generic_rsearch<S: SearchStrategy>(
    hay_bytes: &[u8],
    nee_bytes: &[u8],
) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    if nee_len == 0 {
        return Some(hay_len);
    }
    if hay_len < nee_len {
        return None;
    }

    let pivot = S::pivot(nee_len);
    let pick_byte = nee_bytes[pivot];

    let search_slice = &hay_bytes[pivot..hay_len - nee_len + pivot + 1];

    for m in ::memx::iter::memrchr_iter(search_slice, pick_byte) {
        let st = m;
        if ::memx::memeq(nee_bytes, &hay_bytes[st..st + nee_len]) {
            return Some(st);
        }
    }
    None
}

#[inline]
pub(crate) fn generic_search_iac<S: SearchStrategy>(
    hay_bytes: &[u8],
    nee_bytes: &[u8],
) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    if nee_len == 0 {
        return Some(0);
    }
    if hay_len < nee_len {
        return None;
    }

    let pivot = S::pivot(nee_len);
    let pick_byte = nee_bytes[pivot];
    let pick_byte_uc = pick_byte.to_ascii_uppercase();
    let pick_byte_lc = pick_byte.to_ascii_lowercase();

    let search_slice = &hay_bytes[pivot..hay_len - nee_len + pivot + 1];

    for m in ::memx::iter::memchr_dbl_iter(search_slice, pick_byte_uc, pick_byte_lc) {
        let st = m;
        if nee_bytes.eq_ignore_ascii_case(&hay_bytes[st..st + nee_len]) {
            return Some(st);
        }
    }
    None
}

#[inline]
pub(crate) fn generic_rsearch_iac<S: SearchStrategy>(
    hay_bytes: &[u8],
    nee_bytes: &[u8],
) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    if nee_len == 0 {
        return Some(hay_len);
    }
    if hay_len < nee_len {
        return None;
    }

    let pivot = S::pivot(nee_len);
    let pick_byte = nee_bytes[pivot];
    let pick_byte_uc = pick_byte.to_ascii_uppercase();
    let pick_byte_lc = pick_byte.to_ascii_lowercase();

    let search_slice = &hay_bytes[pivot..hay_len - nee_len + pivot + 1];

    for m in ::memx::iter::memrchr_dbl_iter(search_slice, pick_byte_uc, pick_byte_lc) {
        let st = m;
        if nee_bytes.eq_ignore_ascii_case(&hay_bytes[st..st + nee_len]) {
            return Some(st);
        }
    }
    None
}
