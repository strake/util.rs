#![no_std]

use core::{cmp::Ordering, slice};

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
}
