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

pub struct RWLock(LazyConst<ext::RwLockHandle, RWLockInitializer>);

struct RWLockInitializer;
impl ext::LazyConstInitializer<ext::RwLockHandle> for RWLockInitializer {
    fn initialize(&self) -> ext::RwLockHandle {
        rwlock().new()
    }
}

#[inline]
fn rwlock() -> &'static ext::RwLock {
    ext::RWLOCK.try_get().expect("os::dynamic::RwLock is not initialized")
}

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock(LazyConst::new(ext::RwLockHandle::uninitialized(), RWLockInitializer))
    }

    #[inline]
    pub unsafe fn read(&self) {
        rwlock().read(self.handle())
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        rwlock().try_read(self.handle())
    }

    #[inline]
    pub unsafe fn write(&self) {
        rwlock().write(self.handle())
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        rwlock().try_write(self.handle())
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        rwlock().read_unlock(self.handle())
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        rwlock().write_unlock(self.handle())
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        rwlock().destroy(self.handle())
    }

    #[inline]
    pub fn handle(&self) -> ext::RwLockHandle {
        *self.0.get()
    }
}
