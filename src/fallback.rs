#[inline]
pub fn memchr_iter(needle: u8, haystack: &[u8]) -> Memchr {
    Memchr::new(needle, haystack)
}
#[inline]
pub fn memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<Memchr> {
    Memchr::new(needle, haystack).rev()
}

pub struct Memchr<'a> {
    needle: u8,
    haystack: &'a [u8],
    position: usize,
}
impl<'a> Memchr<'a> {
    #[inline]
    pub fn new(needle: u8, haystack: &[u8]) -> Memchr {
        Memchr {
            needle: needle,
            haystack: haystack,
            position: 0,
        }
    }
}
impl<'a> Iterator for Memchr<'a> {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<usize> {
        memchr(self.needle, self.haystack).map(move |idx| {
            self.haystack = self.haystack.split_at(idx + 1).1;
            let found = self.position + idx;
            self.position = found + 1;
            found
        })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.haystack.len()))
    }
}
impl<'a> DoubleEndedIterator for Memchr<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        memrchr(self.needle, self.haystack).map(move |idx| {
            self.haystack = self.haystack.split_at(idx).0;
            self.position + idx
        })
    }
}
/*
// naive algorithm
fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1)
}
fn memrchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().rposition(|&b| b == n1)
}
*/
fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    let haystack_len = haystack.len();
    let haystack = haystack.as_ptr() as *const libc::c_void;
    let n1 = n1 as i32;
    unsafe {
        let p = libc::memchr(haystack, n1, haystack_len);
        if p.is_null() {
            None
        } else {
            Some(p as usize - haystack as usize)
        }
    }
}

fn memrchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    let haystack_len = haystack.len();
    let haystack = haystack.as_ptr() as *const libc::c_void;
    let n1 = n1 as i32;
    unsafe {
        let p = libc::memrchr(haystack, n1, haystack_len);
        if p.is_null() {
            None
        } else {
            Some(p as usize - haystack as usize)
        }
    }
}
