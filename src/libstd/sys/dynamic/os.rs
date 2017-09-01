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
use iter;
use slice;
use ffi::{OsString, OsStr};
use sys::ext::{OsStrExt, OsStringExt};
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

// FIXME: Duplicated.
pub struct SplitPaths<'a> {
    iter: iter::Map<slice::Split<'a, u8, fn(&u8) -> bool>,
                    fn(&'a [u8]) -> PathBuf>,
}

pub fn split_paths(unparsed: &OsStr) -> SplitPaths {
    fn bytes_to_path(b: &[u8]) -> PathBuf {
        PathBuf::from(<OsStr as OsStrExt>::from_bytes(b))
    }
    fn is_colon(b: &u8) -> bool { *b == b':' }
    let unparsed = unparsed.as_bytes();
    SplitPaths {
        iter: unparsed.split(is_colon as fn(&u8) -> bool)
                      .map(bytes_to_path as fn(&[u8]) -> PathBuf)
    }
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

// FIXME: Duplicated.
#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsRef<OsStr>
{
    let mut joined = Vec::new();
    let sep = b':';

    for (i, path) in paths.enumerate() {
        let path = path.as_ref().as_bytes();
        if i > 0 { joined.push(sep) }
        if path.contains(&sep) {
            return Err(JoinPathsError)
        }
        joined.extend_from_slice(path);
    }
    Ok(OsStringExt::from_vec(joined))
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
