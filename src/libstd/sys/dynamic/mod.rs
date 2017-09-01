// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// TODO: remove when possible
#![allow(unused_variables)]
#![allow(missing_docs)]

pub mod ext;

pub mod stdio;
pub mod thread;
pub mod os;
pub mod env;
pub mod fs;
pub mod net;
pub mod path;
pub mod os_str;
pub mod pipe;
pub mod process;
pub mod time;
pub mod condvar;
pub mod mutex;
pub mod rwlock;
pub mod stack_overflow;
pub mod thread_local;
pub mod args;
pub mod rand;

pub mod memchr {
    pub use ::sys_common::memchr::fallback::{memchr, memrchr};
}

use io;

pub fn init() {
    // Nothing to do at init-time.
}

pub fn decode_error_kind(errno: i32) -> io::ErrorKind {
    io::ErrorKind::Other
}

pub unsafe fn abort_internal() -> ! {
    // TODO
    ::core::intrinsics::abort();
}
