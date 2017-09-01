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

pub type Key = usize;

fn tls() -> &'static ext::ThreadLocal {
    ext::THREAD_LOCAL.try_get().expect("os::dynamic::THREAD_LOCAL is not implemented")
}

pub unsafe fn create(dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
    tls().create(dtor)
}

pub unsafe fn set(key: Key, value: *mut u8) {
    tls().set(key, value)
}

pub unsafe fn get(key: Key) -> *mut u8 {
    tls().get(key)
}

pub unsafe fn destroy(key: Key) {
    tls().destroy(key)
}

pub fn requires_synchronized_create() -> bool {
    false
}

