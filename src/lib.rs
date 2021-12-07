#![no_std]

use core::{cmp::*, mem, ops::*, slice};

mod private {
    pub trait OptionExtSealed {}
    pub trait SliceExtSealed {}
}
use private::*;

pub trait OptionExt: OptionExtSealed {
    type Inner;
    fn try_get_or_insert_with<E, F: FnOnce() -> Result<Self::Inner, E>>(&mut self, f: F) -> Result<&mut Self::Inner, E>;
}

impl<A> OptionExtSealed for Option<A> {}
impl<A> OptionExt for Option<A> {
    type Inner = A;

    #[inline]
    fn try_get_or_insert_with<E, F: FnOnce() -> Result<A, E>>(&mut self, f: F) -> Result<&mut A, E> { unsafe {
        if let None = self { *self = Some(f()?) }
        match self {
            Some(a) => Ok(a),
            _ => ::core::hint::unreachable_unchecked(),
        }
    } }
}

pub trait SliceExt: SliceExtSealed {
    type Inner;
    fn is_sorted(&self) -> bool where Self::Inner: Ord { self.is_sorted_by(Ord::cmp) }
    fn is_sorted_by<F: FnMut(&Self::Inner, &Self::Inner) -> Ordering>(&self, f: F) -> bool;
    fn is_sorted_by_key<F: FnMut(&Self::Inner) -> K, K: Ord>(&self, mut f: F) -> bool {
        self.is_sorted_by(|a, b| Ord::cmp(&f(a), &f(b)))
    }
    fn try_split_at(&self, k: usize) -> Option<(&Self, &Self)>;
    fn try_split_at_mut(&mut self, k: usize) -> Option<(&mut Self, &mut Self)>;
    unsafe fn split_at_unchecked(&self, k: usize) -> (&Self, &Self);
    unsafe fn split_at_unchecked_mut(&mut self, k: usize) -> (&mut Self, &mut Self);
    unsafe fn copy_from_ptr(&mut self, _: *const Self::Inner);

    fn copy_x(&mut self, s: usize, t: usize, n: usize) where Self::Inner: Copy;
    fn copy_from_x(&mut self, other: &Self) where Self::Inner: Copy;
}

impl<A> SliceExtSealed for [A] {}
impl<A> SliceExt for [A] {
    type Inner = A;

    #[inline]
    fn is_sorted_by<F: FnMut(&A, &A) -> Ordering>(&self, mut f: F) -> bool {
        (1..self.len()).all(|k| Ordering::Less != f(&self[k], &self[k-1]))
    }

    #[inline]
    fn try_split_at(&self, k: usize) -> Option<(&Self, &Self)> {
        if k > self.len() { None } else { Some(unsafe { self.split_at_unchecked(k) }) }
    }

    #[inline]
    fn try_split_at_mut(&mut self, k: usize) -> Option<(&mut Self, &mut Self)> {
        if k > self.len() { None } else { Some(unsafe { self.split_at_unchecked_mut(k) }) }
    }

    #[inline(always)]
    unsafe fn split_at_unchecked(&self, k: usize) -> (&Self, &Self) {
        (slice::from_raw_parts(self.as_ptr(), k),
         slice::from_raw_parts(self.as_ptr().add(k), self.len() - k))
    }

    #[inline(always)]
    unsafe fn split_at_unchecked_mut(&mut self, k: usize) -> (&mut Self, &mut Self) {
        (slice::from_raw_parts_mut(self.as_mut_ptr(), k),
         slice::from_raw_parts_mut(self.as_mut_ptr().add(k), self.len() - k))
    }

    #[inline(always)]
    unsafe fn copy_from_ptr(&mut self, ptr: *const Self::Inner) {
        ptr.copy_to_nonoverlapping(self.as_mut_ptr(), self.len())
    }

    #[inline]
    fn copy_x(&mut self, s: usize, t: usize, n: usize) where Self::Inner: Copy { self.copy_within(s..s + n, t) }

    #[inline]
    fn copy_from_x(&mut self, other: &Self) where Self::Inner: Copy { unsafe {
        let l = core::cmp::min(self.len(), other.len());
        core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr(), l)
    } }
}

#[inline]
pub fn checked_sub<A: PartialOrd<B> + Sub<B>, B>(a: A, b: B) -> Option<<A as Sub<B>>::Output> {
    if a >= b { Some(a - b) } else { None }
}

#[inline]
pub fn zip_opt<A, B>(x: Option<A>, y: Option<B>) -> Option<(A, B)> {
    match (x, y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

#[inline(always)]
pub fn ptr_diff<A>(q: *mut A, p: *mut A) -> usize { (q as usize - p as usize)/mem::size_of::<A>() }
