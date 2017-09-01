// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io::{self, SeekFrom};
use path::{ Path, PathBuf };
use sys::time::SystemTime;
use iter::Iterator;
use ffi::OsString;

#[derive(Clone, Debug)]
pub(crate) struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    // system-specific
    //custom_flags: i32,
    //mode: mode_t,
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        unimplemented!()
    }

    pub fn read(&mut self, read: bool) { self.read = read; }
    pub fn write(&mut self, write: bool) { self.write = write; }
    pub fn append(&mut self, append: bool) { self.append = append; }
    pub fn truncate(&mut self, truncate: bool) { self.truncate = truncate; }
    pub fn create(&mut self, create: bool) { self.create = create; }
    pub fn create_new(&mut self, create_new: bool) { self.create_new = create_new; }
}

#[derive(Debug)]
pub(crate) struct File {
    _todo: (),
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        unimplemented!()
    }

    pub fn fsync(&self) -> io::Result<()> {
        unimplemented!()
    }

    pub fn datasync(&self) -> io::Result<()> {
        unimplemented!()
    }

    pub fn truncate(&self, size: u64) -> io::Result<()> {
        unimplemented!()
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unimplemented!()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unimplemented!()
    }

    pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
        unimplemented!()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn flush(&self) -> io::Result<()> {
        unimplemented!()
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub(crate) struct DirBuilder {
    _todo: (),
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        unimplemented!()
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub(crate) struct FileAttr {
    _todo: (),
}

impl FileAttr {
    pub fn file_type(&self) -> FileType {
        unimplemented!()
    }

    pub fn size(&self) -> u64 {
        unimplemented!()
    }

    pub fn perm(&self) -> FilePermissions {
        unimplemented!()
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        unimplemented!()
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        unimplemented!()
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub(crate) struct ReadDir {
    _todo: (),
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        unimplemented!()
    }
}

pub(crate) struct DirEntry {
    _todo: (),
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        unimplemented!()
    }

    pub fn file_name(&self) -> OsString {
        unimplemented!()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unimplemented!()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FilePermissions {
    _todo: (),
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        unimplemented!()
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct FileType {
    _todo: (),
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        unimplemented!()
    }

    pub fn is_file(&self) -> bool {
        unimplemented!()
    }

    pub fn is_symlink(&self) -> bool {
        unimplemented!()
    }
}



pub(crate) fn readdir(p: &Path) -> io::Result<ReadDir> {
    unimplemented!()
}

pub(crate) fn unlink(p: &Path) -> io::Result<()> {
    unimplemented!()
}

pub(crate) fn rename(old: &Path, new: &Path) -> io::Result<()> {
    unimplemented!()
}

pub(crate) fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    unimplemented!()
}

pub(crate) fn rmdir(p: &Path) -> io::Result<()> {
    unimplemented!()
}

pub(crate) fn remove_dir_all(path: &Path) -> io::Result<()> {
    unimplemented!()
}

pub(crate) fn readlink(p: &Path) -> io::Result<PathBuf> {
    unimplemented!()
}

pub(crate) fn symlink(src: &Path, dst: &Path) -> io::Result<()> {
    unimplemented!()
}

pub(crate) fn link(src: &Path, dst: &Path) -> io::Result<()> {
    unimplemented!()
}

pub(crate) fn stat(p: &Path) -> io::Result<FileAttr> {
    unimplemented!()
}

pub(crate) fn lstat(p: &Path) -> io::Result<FileAttr> {
    unimplemented!()
}

pub(crate) fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    unimplemented!()
}

pub(crate) fn copy(from: &Path, to: &Path) -> io::Result<u64> {
    unimplemented!()
}
