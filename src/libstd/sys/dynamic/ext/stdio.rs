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

pub trait Stdin: Sync {
    fn read(&self, data: &mut [u8]) -> io::Result<usize>;
}

pub trait Stdout: Sync {
    fn write(&self, data: &[u8]) -> io::Result<()>;
    fn flush(&self) -> io::Result<()>;
}

