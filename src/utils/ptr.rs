use std::{
    marker::PhantomData,
    sync::atomic::{AtomicPtr, Ordering},
};

/// A thread-safe pointer wrapper for shared mutable access.
///
/// Ptr provides a way to safely share mutable references across threads using atomic operations.
/// It wraps an `AtomicPtr` and provides methods to access the underlying data safely.
///
/// # Type Parameters
///
/// * `'a` - Lifetime parameter for the reference
/// * `T` - Type of the data being pointed to
///
/// # Safety
///
/// This type uses unsafe operations internally but provides a safe interface.
/// The caller must ensure that the pointer remains valid for the lifetime `'a`.
///
/// # Example
///
/// ```rust,no_run
/// use plux_rs::utils::Ptr;
///
/// let mut data = 42;
/// let ptr = Ptr::new(&mut data);
///
/// // Access as immutable reference
/// let value = ptr.as_ref();
/// assert_eq!(*value, 42);
///
/// // Access as mutable reference
/// let mut_ref = ptr.as_mut();
/// *mut_ref = 24;
/// assert_eq!(*ptr.as_ref(), 24);
/// ```
#[derive(Debug)]
pub struct Ptr<'a, T: 'a> {
    value: AtomicPtr<T>,
    marker: PhantomData<&'a T>,
}

impl<'a, T> Ptr<'a, T> {
    /// Creates a new Ptr from a mutable pointer.
    ///
    /// # Parameters
    ///
    /// * `value` - Raw mutable pointer to the data
    ///
    /// # Returns
    ///
    /// Returns a new Ptr instance.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer remains valid for the lifetime `'a`
    /// and that no other mutable references to the same data exist.
    pub const fn new(value: *mut T) -> Self {
        Self {
            value: AtomicPtr::new(value),
            marker: PhantomData,
        }
    }

    /// Returns the raw pointer.
    ///
    /// # Returns
    ///
    /// Returns the underlying `*mut T` pointer.
    pub fn as_ptr(&self) -> *mut T {
        self.value.load(Ordering::Relaxed)
    }

    /// Returns an immutable reference to the data.
    ///
    /// # Returns
    ///
    /// Returns `&T` to the underlying data.
    ///
    /// # Safety
    ///
    /// This method is safe as long as the original pointer was valid and no mutable
    /// references are being used concurrently.
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.value.load(Ordering::Relaxed) }
    }

    /// Returns a mutable reference to the data.
    ///
    /// # Returns
    ///
    /// Returns `&mut T` to the underlying data.
    ///
    /// # Safety
    ///
    /// This method is safe as long as the original pointer was valid and no other
    /// references (mutable or immutable) are being used concurrently.
    pub fn as_mut(&self) -> &mut T {
        unsafe { &mut *self.value.load(Ordering::Relaxed) }
    }
}
