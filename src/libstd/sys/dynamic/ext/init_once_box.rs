// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use core::cell::UnsafeCell;

pub struct InitializedError;

pub struct InitOnceBox<T: ?Sized + Sync> {
    inner: UnsafeCell<Option<Box<T>>>,
    initializing: AtomicBool,
    initialized: AtomicBool,
}

unsafe impl<T: ?Sized + Sync> Sync for InitOnceBox<T> {}

impl<T: ?Sized + Sync> InitOnceBox<T> {
    pub const fn empty() -> Self {
        InitOnceBox{
            inner: UnsafeCell::new(None),
            initializing: ATOMIC_BOOL_INIT,
            initialized: ATOMIC_BOOL_INIT,
        }
    }

    pub fn initialize(&self, b: Box<T>) -> Result<(), InitializedError> {
        // We use two atomic bools to ensure integrity of the box.
        // The `initializing` bool ensures that only one initializer can
        // modify the UnsafeCell. The `initialized` bool ensures that a
        // reader cannot access the cell before the initializer is finished.

        // Note that we cannot use AtomicPtr<T> (or any other scheme where
        // the pointer is atomically set) because the representation of
        // Box<T> on unsized types is two machine words, which is more than
        // most processors can atomically handle. Requiring sized T is not
        // an option either, since this type was specifically made to hold
        // trait objects.

        // test-and-set
        let initializing = self.initializing.swap(true, Ordering::Relaxed);
        if initializing {
            // Spin until initialized.
            while !self.initialized.load(Ordering::Acquire) {}
            return Err(InitializedError);
        }

        unsafe {
            (*self.inner.get()) = Some(b);
        }

        self.initialized.store(true, Ordering::Release);
        Ok(())
    }

    /* We don't actually want to use this anywhere...

    /// Get the contents.
    /// If necessary, spin to wait until the box is initialized.
    #[inline]
    pub fn get(&self) -> &T {
        while !self.initialized.load(Ordering::Acquire) {}
        unsafe { &*self.inner.get() }.as_ref().unwrap()
    }
    */

    #[inline]
    pub fn try_get(&self) -> Option<&T> {
        if self.initialized.load(Ordering::Acquire) {
            use core::ops::Deref;
            unsafe { &*self.inner.get() }.as_ref().map(Box::deref)
        } else {
            None
        }
    }
}

