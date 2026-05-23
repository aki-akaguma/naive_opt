use crate::mc_generic::{generic_rsearch, generic_rsearch_iac, generic_search, generic_search_iac, LastByte};

#[inline]
pub(crate) fn naive_opt_mc_last_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_search::<LastByte>(hay_bytes, nee_bytes)
}

#[inline]
pub(crate) fn naive_opt_mc_last_rev_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_rsearch::<LastByte>(hay_bytes, nee_bytes)
}

#[inline]
pub(crate) fn naive_opt_mc_last_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_search_iac::<LastByte>(hay_bytes, nee_bytes)
}

#[inline]
pub(crate) fn naive_opt_mc_last_rev_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_rsearch_iac::<LastByte>(hay_bytes, nee_bytes)
}
