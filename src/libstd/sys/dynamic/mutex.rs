// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys::ext;
use sys::ext::LazyConst;

pub struct Mutex(LazyConst<ext::MutexHandle, MutexInitializer>);
pub struct ReentrantMutex(LazyConst<ext::MutexHandle, ReMutexInitializer>);

#[inline]
fn mutex() -> &'static ext::Mutex {
    ext::MUTEX.try_get().expect("os::dynamic::MUTEX is not initialized")
}

#[inline]
fn remutex() -> &'static ext::Mutex {
    ext::REENTRANT_MUTEX.try_get().expect("os::dynamic::REENTRANT_MUTEX is not initialized")
}

struct MutexInitializer;
impl ext::LazyConstInitializer<ext::MutexHandle> for MutexInitializer {
    fn initialize(&self) -> ext::MutexHandle {
        mutex().new()
    }
}

struct ReMutexInitializer;
impl ext::LazyConstInitializer<ext::MutexHandle> for ReMutexInitializer {
    fn initialize(&self) -> ext::MutexHandle {
        remutex().new()
    }
}

impl Mutex {
    pub const fn new() -> Self {
        Mutex(LazyConst::new(ext::MutexHandle::uninitialized(), MutexInitializer))
    }

    #[inline]
    pub unsafe fn init(&mut self) {
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        mutex().try_lock(self.handle())
    }

    #[inline]
    pub unsafe fn lock(&self) {
        mutex().lock(self.handle())
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        mutex().unlock(self.handle())
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        mutex().destroy(self.handle())
    }

    #[inline]
    pub fn handle(&self) -> ext::MutexHandle {
        *self.0.get()
    }
}



impl ReentrantMutex {
    pub const fn uninitialized() -> Self {
        ReentrantMutex(LazyConst::new(ext::MutexHandle::uninitialized(), ReMutexInitializer))
    }

    #[inline]
    pub unsafe fn init(&mut self) {
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        remutex().try_lock(self.handle())
    }

    #[inline]
    pub unsafe fn lock(&self) {
        remutex().lock(self.handle())
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        remutex().unlock(self.handle())
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        remutex().destroy(self.handle())
    }

    #[inline]
    pub fn handle(&self) -> ext::MutexHandle {
        *self.0.get()
    }
}
