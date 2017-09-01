// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use alloc::boxed::FnBox;
use ffi::CStr;
use io;
use time::Duration;

pub struct Thread {
    _todo: ()
}

impl Thread {
    pub unsafe fn new<'a>(stack: usize, p: Box<FnBox() + 'a>) -> io::Result<Thread> {
        unimplemented!()
    }

    pub fn yield_now() {
        unimplemented!()
    }

    pub fn set_name(_name: &CStr) {
        unimplemented!()
    }

    pub fn join(self) {
        unimplemented!()
    }

    pub fn sleep(dur: Duration) {
        unimplemented!()
    }
}


pub mod guard {
    pub unsafe fn current() -> Option<usize> { None }
    pub unsafe fn init() -> Option<usize> { None }
}
