use std::{marker::PhantomData, sync::atomic::{AtomicPtr, Ordering}};

#[derive(Debug)]
pub struct Ptr<'a, T: 'a> {
    value: AtomicPtr<T>,
    marker: PhantomData<&'a T>,
}

impl<'a, T> Ptr<'a, T> {
    pub const fn new(value: *mut T) -> Self {
        Self {
            value: AtomicPtr::new(value),
            marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.value.load(Ordering::Relaxed)
    }

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.value.load(Ordering::Relaxed) }
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe { &mut *self.value.load(Ordering::Relaxed) }
    }
}