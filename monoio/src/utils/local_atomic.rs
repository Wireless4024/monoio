#![allow(dead_code)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[inline(always)]
fn usize_ptr(base: &AtomicUsize) -> *mut usize {
    base.as_ptr()
}

#[inline(always)]
pub(crate) fn load_local(base: &AtomicUsize) -> usize {
    unsafe { usize_ptr(base).read() }
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

#[inline(always)]
pub(crate) fn compare_exchange_local(
    base: &AtomicUsize,
    expected: usize,
    new: usize,
    _success: Ordering,
    _failure: Ordering,
) -> Result<usize, usize> {
    let current = load_local(base);
    if current == expected {
        unsafe {
            let raw_ptr = usize_ptr(base);
            raw_ptr.write(new);
        }
        Ok(current)
    } else {
        Err(current)
    }
}
