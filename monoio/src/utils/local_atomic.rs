use std::sync::atomic::AtomicUsize;

#[inline(always)]
fn usize_ptr(base: &AtomicUsize) -> *mut usize {
    base.as_ptr()
}

#[inline(always)]
pub(crate) fn fetch_add_local(base: &AtomicUsize, amount: usize) -> usize {
    unsafe {
        let raw_ptr = usize_ptr(base);
        let old = raw_ptr.read();
        let new = old.wrapping_add(amount);
        raw_ptr.write(new);
        new
    }
}

#[inline(always)]
pub(crate) fn fetch_sub_local(base: &AtomicUsize, amount: usize) -> usize {
    unsafe {
        let raw_ptr = usize_ptr(base);
        let old = raw_ptr.read();
        let new = old.wrapping_sub(amount);
        raw_ptr.write(new);
        new
    }
}
