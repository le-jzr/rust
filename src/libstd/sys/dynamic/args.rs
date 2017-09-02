// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use vec;
use marker::PhantomData;
use ffi::OsString;
use iter::{Iterator, ExactSizeIterator, DoubleEndedIterator};

use sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

use sys::ext;
use ptr::null;
use mem;

pub struct Args {
    iter: vec::IntoIter<OsString>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl Args {
    pub fn inner_debug(&self) -> &[OsString] {
        self.iter.as_slice()
    }
}

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize { self.iter.len() }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> { self.iter.next_back() }
}

static ARGC: AtomicIsize = AtomicIsize::new(0);
static ARGV: AtomicUsize = AtomicUsize::new(0);

pub fn args() -> Args {
    let argc = ARGC.load(Ordering::Acquire);
    let argv = if argc == 0 {
        null()
    } else {
        ARGV.load(Ordering::Relaxed) as *const *const u8
    };

    let v: Vec<String> = unsafe { ext::ARGS.try_get().expect("os::dynamic::ARGS are not initialized").args(argc, argv) };
    let v: Vec<OsString> = unsafe { mem::transmute(v) };

    Args {
        iter: v.into_iter(),
        _dont_send_or_sync_me: PhantomData,
    }
}

pub unsafe fn init(argc: isize, argv: *const *const u8) {
    ARGV.store(argv as usize, Ordering::Relaxed);
    ARGC.store(argc, Ordering::Release);
}

pub unsafe fn cleanup() {
}
