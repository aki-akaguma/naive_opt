#[inline(always)]
pub(crate) fn naive_opt_mc_1st_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(0);
    }
    if hay_len < nee_len {
        return None;
    }
    let byte_1st = nee_bytes[0];
    for m in ::memx::iter::memchr_iter(hay_bytes, byte_1st) {
        let st = m;
        let ed = st + nee_len;
        if ed > hay_len {
            break;
        }
        // if nee_bytes == &hay_bytes[st..ed] { ... }
        if ::memx::memeq(nee_bytes, &hay_bytes[st..ed]) {
            return Some(st);
        }
    }
    None
}

#[inline(always)]
pub(crate) fn naive_opt_mc_1st_rev_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(hay_len);
    }
    if hay_len < nee_len {
        return None;
    }
    let byte_1st = nee_bytes[0];
    for m in ::memx::iter::memrchr_iter(&hay_bytes[..(hay_len - nee_len + 1)], byte_1st) {
        let st = m;
        let ed = st + nee_len;
        if ed > hay_len {
            break;
        }
        // if nee_bytes == &hay_bytes[st..ed] { ... }
        if ::memx::memeq(nee_bytes, &hay_bytes[st..ed]) {
            return Some(st);
        }
    }
    None
}

#[inline(always)]
pub(crate) fn naive_opt_mc_1st_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(0);
    }
    if hay_len < nee_len {
        return None;
    }
    let byte_1st = nee_bytes[0];
    let byte_1st_uc = byte_1st.to_ascii_uppercase();
    let byte_1st_lc = byte_1st.to_ascii_lowercase();
    for m in ::memx::iter::memchr_dbl_iter(hay_bytes, byte_1st_uc, byte_1st_lc) {
        let st = m;
        let ed = st + nee_len;
        if ed > hay_len {
            break;
        }
        // if nee_bytes == &hay_bytes[st..ed] { ... }
        //if ::memx::memeq(nee_bytes, &hay_bytes[st..ed]) {
        if nee_bytes.eq_ignore_ascii_case(&hay_bytes[st..ed]) {
            return Some(st);
        }
    }
    None
}

#[inline(always)]
pub(crate) fn naive_opt_mc_1st_rev_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(hay_len);
    }
    if hay_len < nee_len {
        return None;
    }
    let byte_1st = nee_bytes[0];
    let byte_1st_uc = byte_1st.to_ascii_uppercase();
    let byte_1st_lc = byte_1st.to_ascii_lowercase();
    for m in ::memx::iter::memrchr_dbl_iter(
        &hay_bytes[..(hay_len - nee_len + 1)],
        byte_1st_uc,
        byte_1st_lc,
    ) {
        let st = m;
        let ed = st + nee_len;
        if ed > hay_len {
            break;
        }
        // if nee_bytes == &hay_bytes[st..ed] { ... }
        //if ::memx::memeq(nee_bytes, &hay_bytes[st..ed]) {
        if nee_bytes.eq_ignore_ascii_case(&hay_bytes[st..ed]) {
            return Some(st);
        }
    }
    None
}
