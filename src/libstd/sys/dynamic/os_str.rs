// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use borrow::Cow;
use fmt;
use str;
use mem;

#[derive(Clone, Hash)]
pub struct Buf {
    pub inner: String
}

pub struct Slice {
    pub inner: str
}

impl fmt::Debug for Slice {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, formatter)
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, formatter)
    }
}

impl fmt::Debug for Buf {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), formatter)
    }
}

impl fmt::Display for Buf {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_slice(), formatter)
    }
}

impl Buf {
    pub fn from_string(s: String) -> Buf {
        Buf { inner: s }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Buf {
        Buf {
            inner: String::with_capacity(capacity)
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    pub fn as_slice(&self) -> &Slice {
        unsafe { mem::transmute(&*self.inner) }
    }

    pub fn into_string(self) -> Result<String, Buf> {
        Ok(self.inner)
    }

    pub fn push_slice(&mut self, s: &Slice) {
        self.inner.push_str(&s.inner)
    }

    #[inline]
    pub fn into_box(self) -> Box<Slice> {
        unsafe { mem::transmute(self.inner.into_boxed_str()) }
    }

    #[inline]
    pub fn from_box(boxed: Box<Slice>) -> Buf {
        let inner: Box<str> = unsafe { mem::transmute(boxed) };
        Buf { inner: inner.into_string() }
    }
}

impl Slice {

    pub fn from_str(s: &str) -> &Slice {
        unsafe { mem::transmute(s) }
    }

    pub fn to_str(&self) -> Option<&str> {
        Some(&self.inner)
    }

    pub fn to_string_lossy(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }

    pub fn to_owned(&self) -> Buf {
        Buf { inner: self.inner.to_string() }
    }

    #[inline]
    pub fn into_box(&self) -> Box<Slice> {
        let boxed: Box<str> = self.inner.into();
        unsafe { mem::transmute(boxed) }
    }

    pub fn empty_box() -> Box<Slice> {
        let boxed: Box<str> = Default::default();
        unsafe { mem::transmute(boxed) }
    }
}
