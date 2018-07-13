#![no_std]

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
    fn is_sorted(&self) -> bool where Self::Inner: Ord;
}

impl<A> SliceExtSealed for [A] {}
impl<A> SliceExt for [A] {
    type Inner = A;

    #[inline]
    fn is_sorted(&self) -> bool where A: Ord { (1..self.len()).all(|k| self[k] >= self[k-1]) }
}
