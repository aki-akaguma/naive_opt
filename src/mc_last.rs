#[inline(always)]
pub(crate) fn naive_opt_mc_last_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    let hay_len = hay_bytes.len();
    let nee_len = nee_bytes.len();
    //
    if nee_len == 0 {
        return Some(0);
    }
    let last_idx = nee_len - 1;
    let last_byte = nee_bytes[last_idx];
    if hay_len <= last_idx {
        return None;
    }
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
    let last_idx = nee_len - 1;
    let last_byte = nee_bytes[last_idx];
    if hay_len <= last_idx {
        return None;
    }
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
