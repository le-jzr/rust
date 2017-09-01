// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    _todo: (),
}

impl Instant {
    pub fn now() -> Instant {
        unimplemented!()
    }

    pub fn sub_instant(&self, other: &Instant) -> Duration {
        unimplemented!()
    }

    pub fn add_duration(&self, other: &Duration) -> Instant {
        unimplemented!()
    }

    pub fn sub_duration(&self, other: &Duration) -> Instant {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime {
    _todo: (),
}

impl SystemTime {
    pub fn now() -> SystemTime {
        unimplemented!()
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        unimplemented!()
    }

    pub fn add_duration(&self, other: &Duration) -> SystemTime {
        unimplemented!()
    }

    pub fn sub_duration(&self, other: &Duration) -> SystemTime {
        unimplemented!()
    }
}

pub const UNIX_EPOCH: SystemTime = SystemTime { _todo: () };
