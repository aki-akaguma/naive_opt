use crate::mc_generic::{generic_rsearch, generic_rsearch_iac, generic_search, generic_search_iac, FirstByte};

#[inline]
pub(crate) fn naive_opt_mc_1st_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_search::<FirstByte>(hay_bytes, nee_bytes)
}

#[inline]
pub(crate) fn naive_opt_mc_1st_rev_bytes(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_rsearch::<FirstByte>(hay_bytes, nee_bytes)
}

#[inline]
pub(crate) fn naive_opt_mc_1st_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_search_iac::<FirstByte>(hay_bytes, nee_bytes)
}

#[inline]
pub(crate) fn naive_opt_mc_1st_rev_bytes_iac(hay_bytes: &[u8], nee_bytes: &[u8]) -> Option<usize> {
    generic_rsearch_iac::<FirstByte>(hay_bytes, nee_bytes)
}
