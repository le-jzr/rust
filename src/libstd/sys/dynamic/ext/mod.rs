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

use io;
use time::Duration;

mod ffi;
pub use self::ffi::*;

mod init_once_box;
pub use self::init_once_box::*;

mod lazy_const;
pub use self::lazy_const::*;

mod thread_local;
pub use self::thread_local::*;

pub static MUTEX: InitOnceBox<Mutex> = InitOnceBox::empty();
pub static REENTRANT_MUTEX: InitOnceBox<Mutex> = InitOnceBox::empty();
pub static RWLOCK: InitOnceBox<RwLock> = InitOnceBox::empty();
pub static CONDVAR: InitOnceBox<Condvar> = InitOnceBox::empty();

pub static STDIN: InitOnceBox<Stdin> = InitOnceBox::empty();
pub static STDOUT: InitOnceBox<Stdout> = InitOnceBox::empty();
pub static STDERR: InitOnceBox<Stdout> = InitOnceBox::empty();

pub static THREAD_LOCAL: InitOnceBox<ThreadLocal> = InitOnceBox::empty();

#[derive(Copy, Clone)]
pub struct MutexHandle(usize);

impl MutexHandle {
    pub const fn uninitialized() -> MutexHandle {
        MutexHandle(0)
    }
}

#[derive(Copy, Clone)]
pub struct RwLockHandle(usize);

impl RwLockHandle {
    pub const fn uninitialized() -> RwLockHandle {
        RwLockHandle(0)
    }
}

#[derive(Copy, Clone)]
pub struct CondvarHandle(usize);

impl CondvarHandle {
    pub const fn uninitialized() -> CondvarHandle {
        CondvarHandle(0)
    }
}

pub trait Mutex: Sync {
    fn new(&self) -> MutexHandle;
    unsafe fn destroy(&self, m: MutexHandle);
    unsafe fn lock(&self, m: MutexHandle);
    unsafe fn try_lock(&self, m: MutexHandle) -> bool;
    unsafe fn unlock(&self, m: MutexHandle);
}

pub trait RwLock: Sync {
    fn new(&self) -> RwLockHandle;
    unsafe fn destroy(&self, m: RwLockHandle);
    unsafe fn read(&self, m: RwLockHandle);
    unsafe fn try_read(&self, m: RwLockHandle) -> bool;
    unsafe fn read_unlock(&self, m: RwLockHandle);
    unsafe fn write(&self, m: RwLockHandle);
    unsafe fn try_write(&self, m: RwLockHandle) -> bool;
    unsafe fn write_unlock(&self, m: RwLockHandle);
}

pub trait Condvar: Sync {
    fn new(&self) -> CondvarHandle;
    unsafe fn destroy(&self, cv: CondvarHandle);
    fn notify_one(&self, cv: CondvarHandle);
    fn notify_all(&self, cv: CondvarHandle);
    fn wait(&self, cv: CondvarHandle, m: MutexHandle);
    fn wait_timeout(&self, cv: CondvarHandle, m: MutexHandle, dur: Duration) -> bool;
}

pub trait Stdin: Sync {
    fn read(&self, data: &mut [u8]) -> io::Result<usize>;
}

pub trait Stdout: Sync {
    fn write(&self, data: &[u8]) -> io::Result<()>;
    fn flush(&self) -> io::Result<()>;
}

