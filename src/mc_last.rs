#[inline(always)]
pub(crate) fn naive_opt_mc_last_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(0);
    }
    if hay_len < nee_len {
        return None;
    }
    let last_idx = nee_len - 1;
    let last_byte = nee_bytes[last_idx];
    for m in ::memx::iter::memchr_iter(&hay_bytes[last_idx..], last_byte) {
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
pub(crate) fn naive_opt_mc_last_rev_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(hay_len);
    }
    if hay_len < nee_len {
        return None;
    }
    let last_idx = nee_len - 1;
    let last_byte = nee_bytes[last_idx];
    for m in ::memx::iter::memrchr_iter(&hay_bytes[last_idx..], last_byte) {
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
pub(crate) fn naive_opt_mc_last_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(0);
    }
    if hay_len < nee_len {
        return None;
    }
    let last_idx = nee_len - 1;
    let byte_last = nee_bytes[last_idx];
    let byte_last_uc = byte_last.to_ascii_uppercase();
    let byte_last_lc = byte_last.to_ascii_lowercase();
    for m in ::memx::iter::memchr_double_iter(&hay_bytes[last_idx..], byte_last_uc, byte_last_lc) {
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
pub(crate) fn naive_opt_mc_last_rev_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(hay_len);
    }
    if hay_len < nee_len {
        return None;
    }
    let last_idx = nee_len - 1;
    let byte_last = nee_bytes[last_idx];
    let byte_last_uc = byte_last.to_ascii_uppercase();
    let byte_last_lc = byte_last.to_ascii_lowercase();
    for m in ::memx::iter::memrchr_double_iter(&hay_bytes[last_idx..], byte_last_uc, byte_last_lc) {
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
