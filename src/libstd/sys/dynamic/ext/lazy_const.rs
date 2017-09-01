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

pub trait LazyConstInitializer<T> {
    fn initialize(&self) -> T;
}

unsafe impl<T, I> Sync for LazyConst<T, I>
    where I: LazyConstInitializer<T> {}

pub struct LazyConst<T, I>
    where I: LazyConstInitializer<T>
{
    inner: UnsafeCell<T>,
    initializer: I,
    initializing: AtomicBool,
    initialized: AtomicBool,
}

impl<T, I> LazyConst<T, I>
    where I: LazyConstInitializer<T>
{
    #[inline]
    pub const fn new(uninitialized: T, init: I) -> Self {
        LazyConst{
            inner: UnsafeCell::new(uninitialized),
            initializer: init,
            initializing: ATOMIC_BOOL_INIT,
            initialized: ATOMIC_BOOL_INIT,
        }
    }

    pub fn get(&self) -> &T {
        if self.initialized.load(Ordering::Acquire) {
            return unsafe {&*self.inner.get()};
        }

        let initializing = self.initializing.swap(true, Ordering::Relaxed);
        if initializing {
            // Spin until initialized.
            while !self.initialized.load(Ordering::Acquire) {}
            return unsafe {&*self.inner.get()};
        }

        unsafe {
            *self.inner.get() = self.initializer.initialize();
            self.initialized.store(true, Ordering::Release);
            &*self.inner.get()
        }
    }
}

