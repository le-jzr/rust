// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub type ThreadLocalKey = usize;

pub trait ThreadLocal: Sync {
    unsafe fn create(&self, dtor: Option<unsafe extern fn(*mut u8)>) -> ThreadLocalKey;
    unsafe fn set(&self, key: ThreadLocalKey, value: *mut u8);
    unsafe fn get(&self, key: ThreadLocalKey) -> *mut u8;
    unsafe fn destroy(&self, key: ThreadLocalKey);
}
