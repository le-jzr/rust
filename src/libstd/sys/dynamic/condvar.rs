// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys::mutex::Mutex;
use time::Duration;

use sys::ext;
use sys::ext::LazyConst;

struct CondvarInitializer;
impl ext::LazyConstInitializer<ext::CondvarHandle> for CondvarInitializer {
    fn initialize(&self) -> ext::CondvarHandle {
        cvar().new()
    }
}

#[inline]
fn cvar() -> &'static ext::Condvar {
    ext::CONDVAR.try_get().expect("os::dynamic::Condvar is not initialized")
}

pub struct Condvar(LazyConst<ext::CondvarHandle, CondvarInitializer>);

impl Condvar {
    #[inline]
    pub const fn new() -> Condvar {
        Condvar(LazyConst::new(ext::CondvarHandle::uninitialized(), CondvarInitializer))
    }

    #[inline]
    pub unsafe fn init(&self) {
    }

    #[inline]
    pub fn notify_one(&self) {
        cvar().notify_one(self.handle())
    }

    #[inline]
    pub fn notify_all(&self) {
        cvar().notify_all(self.handle())
    }

    #[inline]
    pub fn wait(&self, mutex: &Mutex) {
        cvar().wait(self.handle(), mutex.handle())
    }

    #[inline]
    pub fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        cvar().wait_timeout(self.handle(), mutex.handle(), dur)
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        cvar().destroy(self.handle())
    }

    #[inline]
    pub fn handle(&self) -> ext::CondvarHandle {
        *self.0.get()
    }
}
