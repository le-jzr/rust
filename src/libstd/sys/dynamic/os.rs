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
use path::{self, PathBuf};
use error::Error as StdError;
use ffi::{OsString, OsStr};
use vec;
use fmt;
use marker::PhantomData;

pub struct Env {
    iter: vec::IntoIter<(OsString, OsString)>,
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

pub fn env() -> Env {
    unimplemented!()
}

pub fn getenv(key: &OsStr) -> io::Result<Option<OsString>> {
    unimplemented!()
}

pub fn setenv(key: &OsStr, value: &OsStr) -> io::Result<()> {
    unimplemented!()
}

pub fn unsetenv(key: &OsStr) -> io::Result<()> {
    unimplemented!()
}

pub fn getcwd() -> io::Result<PathBuf> {
    unimplemented!()
}

pub fn chdir(p: &path::Path) -> io::Result<()> {
    unimplemented!()
}


pub struct SplitPaths<'a> {
    __todo: &'a (),
}

pub fn split_paths(unparsed: &OsStr) -> SplitPaths {
    SplitPaths {
        __todo: &(),
    }
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> { unimplemented!() }
    fn size_hint(&self) -> (usize, Option<usize>) { unimplemented!() }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsRef<OsStr>
{
    unimplemented!()
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "path segment contains separator `:`".fmt(f)
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str { "failed to join paths" }
}

pub fn home_dir() -> Option<PathBuf> {
    unimplemented!()
}

pub fn temp_dir() -> PathBuf {
    unimplemented!()
}

pub fn exit(code: i32) -> ! {
    unimplemented!()
}

pub fn current_exe() -> io::Result<PathBuf> {
    unimplemented!()
}

pub fn errno() -> i32 {
    unimplemented!()
}

pub fn error_string(errno: i32) -> String {
    unimplemented!()
}
