// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io;
use sys::ext;

pub struct Stdin(());
pub struct Stdout(());
pub struct Stderr(());

impl Stdin {
    pub fn new() -> io::Result<Stdin> { Ok(Stdin(())) }

    pub fn read(&self, data: &mut [u8]) -> io::Result<usize> {
        if let Some(stdin) = ext::STDIN.try_get() {
            stdin.read(data)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "os::dynamic::STDIN is not inititialized"))
        }
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> { Ok(Stdout(())) }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        if let Some(stdout) = ext::STDOUT.try_get() {
            stdout.write(data)?;
            Ok(data.len())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "os::dynamic::STDOUT is not inititialized"))
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        if let Some(stdout) = ext::STDOUT.try_get() {
            stdout.flush()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "os::dynamic::STDOUT is not inititialized"))
        }
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> { Ok(Stderr(())) }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        if let Some(stderr) = ext::STDERR.try_get() {
            stderr.write(data)?;
            Ok(data.len())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "os::dynamic::STDERR is not inititialized"))
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        if let Some(stderr) = ext::STDERR.try_get() {
            stderr.flush()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "os::dynamic::STDERR is not inititialized"))
        }
    }
}

// FIXME: right now this raw stderr handle is used in a few places because
//        std::io::stderr_raw isn't exposed, but once that's exposed this impl
//        should go away
impl io::Write for Stderr {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        Stderr::write(self, data)
    }

    fn flush(&mut self) -> io::Result<()> {
        Stderr::flush(self)
    }
}

// FIXME
pub const EBADF_ERR: i32 = 0xbadbeef as i32;

pub const STDIN_BUF_SIZE: usize = ::sys_common::io::DEFAULT_BUF_SIZE;
