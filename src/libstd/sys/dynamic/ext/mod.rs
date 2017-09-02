// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![unstable(issue = "0000", feature = "dynamic_sys")]

mod init_once_box;
pub use self::init_once_box::*;

mod lazy_const;
pub use self::lazy_const::*;

mod sync;
pub use self::sync::*;

mod stdio;
pub use self::stdio::*;

pub static THREAD_LOCAL: InitOnceBox<ThreadLocal> = InitOnceBox::empty();
pub static MUTEX: InitOnceBox<Mutex> = InitOnceBox::empty();
pub static REENTRANT_MUTEX: InitOnceBox<Mutex> = InitOnceBox::empty();
pub static RWLOCK: InitOnceBox<RwLock> = InitOnceBox::empty();
pub static CONDVAR: InitOnceBox<Condvar> = InitOnceBox::empty();

pub static STDIN: InitOnceBox<Stdin> = InitOnceBox::empty();
pub static STDOUT: InitOnceBox<Stdout> = InitOnceBox::empty();
pub static STDERR: InitOnceBox<Stdout> = InitOnceBox::empty();

pub static ARGS: InitOnceBox<Args> = InitOnceBox::empty();

pub trait Args: Sync {
    unsafe fn args(&self, argc: isize, argv: *const *const u8) -> Vec<String>;
}
