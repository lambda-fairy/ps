use std::marker::PhantomData;
use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct Transmute<T, U, X=False> {
    phantom: PhantomData<(fn(T) -> U, X)>,
}

impl<T, U, X> Transmute<T, U, X> {
    pub unsafe fn new_unchecked() -> Self {
        Transmute { phantom: PhantomData }
    }

    pub fn cast(self, value: T) -> U {
        unsafe {
            // Use `transmute_copy` to get around the size check
            let result: U = mem::transmute_copy(&value);
            mem::forget(value);
            result
        }
    }

    pub fn then<V, Y>(self, _next: Transmute<U, V, Y>)
        -> Transmute<T, V, <X as And<Y>>::Output> where
        X: And<Y>,
    {
        unsafe { Transmute::new_unchecked() }
    }

    pub fn as_ref<'a>(self) -> Transmute<&'a T, &'a U> {
        unsafe { Transmute::new_unchecked() }
    }
}

impl<T, U> Transmute<T, U, True> {
    pub fn as_mut<'a>(self) -> Transmute<&'a mut T, &'a mut U> {
        unsafe { Transmute::new_unchecked() }
    }

    pub fn rev(self) -> Transmute<U, T, True> {
        unsafe { Transmute::new_unchecked() }
    }
}

pub enum False {}
pub enum True {}

pub trait And<Y> {
    type Output;
}
impl And<True> for True { type Output = True; }
impl And<False> for True { type Output = False; }
impl And<True> for False { type Output = False; }
impl And<False> for False { type Output = False; }
