// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys::pipe::AnonPipe;
use sys::fs::File;
use ffi::OsStr;
use io;
use fmt;

#[derive(Debug)]
pub struct Command {
    _todo: (),
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        unimplemented!()
    }

    pub fn arg(&mut self, arg: &OsStr) {
        unimplemented!()
    }

    pub fn env(&mut self, key: &OsStr, val: &OsStr) {
        unimplemented!()
    }

    pub fn env_remove(&mut self, key: &OsStr) {
        unimplemented!()
    }

    pub fn env_clear(&mut self) {
        unimplemented!()
    }

    pub fn cwd(&mut self, dir: &OsStr) {
        unimplemented!()
    }

    pub fn stdin(&mut self, stdin: Stdio) {
        unimplemented!()
    }

    pub fn stdout(&mut self, stdout: Stdio) {
        unimplemented!()
    }

    pub fn stderr(&mut self, stderr: Stdio) {
        unimplemented!()
    }

    pub fn spawn(&mut self, default: Stdio, needs_stdin: bool) -> io::Result<(Process, StdioPipes)> {
        unimplemented!()
    }
/*
    pub fn uid(&mut self, id: u32) {
    }

    pub fn gid(&mut self, id: u32) {
    }

    pub fn before_exec(&mut self, f: Box<FnMut() -> io::Result<()> + Send + Sync>) {
    }

    pub fn exec(&mut self, default: Stdio) -> io::Error {
    }
*/
}

pub enum Stdio {
    Null,
    Inherit,
    MakePipe,
    // TODO
}

impl From<AnonPipe> for Stdio {
    fn from(pipe: AnonPipe) -> Stdio {
        unimplemented!()
    }
}

impl From<File> for Stdio {
    fn from(file: File) -> Stdio {
        unimplemented!()
    }
}

pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ExitStatus {
    _todo: (),
}

impl ExitStatus {
    pub fn success(&self) -> bool {
        unimplemented!()
    }

    pub fn code(&self) -> Option<i32> {
        unimplemented!()
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

pub struct Process {
    _todo: (),
}

impl Process {
    pub fn id(&self) -> u32 {
        unimplemented!()
    }

    pub fn kill(&mut self) -> io::Result<()> {
        unimplemented!()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        unimplemented!()
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        unimplemented!()
    }
}
