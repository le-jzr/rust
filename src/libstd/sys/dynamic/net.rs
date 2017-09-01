// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use io::{Result, Error};
use iter::Iterator;
use net::{SocketAddr, Shutdown, Ipv4Addr, Ipv6Addr};
use time::Duration;

#[allow(non_camel_case_types)]
pub mod netc {
    #[derive(Copy, Clone)]
    pub struct in_addr {
        pub s_addr: u32,
    }

    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    pub type in_port_t = u16;

    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sa_family_t {
        _todo: (),
    }

    pub const AF_INET: sa_family_t = sa_family_t { _todo: () };
    pub const AF_INET6: sa_family_t = sa_family_t { _todo: () };

    pub struct sockaddr {
        _todo: (),
    }

    pub type socklen_t = usize;
}

#[derive(Debug)]
pub struct TcpStream {
    _todo: (),
}

impl TcpStream {
    pub fn connect(addr: &SocketAddr) -> Result<TcpStream> {
        unimplemented!()
    }

    pub fn connect_timeout(_addr: &SocketAddr, _timeout: Duration) -> Result<TcpStream> {
        unimplemented!()
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        unimplemented!()
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        unimplemented!()
    }

    pub fn shutdown(&self, _how: Shutdown) -> Result<()> {
        unimplemented!()
    }

    pub fn duplicate(&self) -> Result<TcpStream> {
        unimplemented!()
    }

    pub fn read_timeout(&self) -> Result<Option<Duration>> {
        unimplemented!()
    }

    pub fn write_timeout(&self) -> Result<Option<Duration>> {
        unimplemented!()
    }

    pub fn set_read_timeout(&self, duration_option: Option<Duration>) -> Result<()> {
        unimplemented!()
    }

    pub fn set_write_timeout(&self, duration_option: Option<Duration>) -> Result<()> {
        unimplemented!()
    }

    pub fn ttl(&self) -> Result<u32> {
        unimplemented!()
    }

    pub fn set_ttl(&self, ttl: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn peek(&self, _buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    pub fn nodelay(&self) -> Result<bool> {
        unimplemented!()
    }

    pub fn set_nodelay(&self, _nodelay: bool) -> Result<()> {
        unimplemented!()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        unimplemented!()
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        unimplemented!()
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct TcpListener {
    _todo: (),
}

impl TcpListener {
    pub fn bind(addr: &SocketAddr) -> Result<TcpListener> {
        unimplemented!()
    }

    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)> {
        unimplemented!()
    }

    pub fn duplicate(&self) -> Result<TcpListener> {
        unimplemented!()
    }

    pub fn ttl(&self) -> Result<u32> {
        unimplemented!()
    }

    pub fn set_ttl(&self, ttl: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        unimplemented!()
    }

    pub fn only_v6(&self) -> Result<bool> {
        unimplemented!()
    }

    pub fn set_only_v6(&self, _only_v6: bool) -> Result<()> {
        unimplemented!()
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        unimplemented!()
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct UdpSocket {
    _todo: (),
}

impl UdpSocket {
    pub fn bind(addr: &SocketAddr) -> Result<UdpSocket> {
        unimplemented!()
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        unimplemented!()
    }

    pub fn peek_from(&self, _buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        unimplemented!()
    }

    pub fn send_to(&self, buf: &[u8], addr: &SocketAddr) -> Result<usize> {
        unimplemented!()
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        unimplemented!()
    }

    pub fn duplicate(&self) -> Result<UdpSocket> {
        unimplemented!()
    }

    pub fn read_timeout(&self) -> Result<Option<Duration>> {
        unimplemented!()
    }

    pub fn write_timeout(&self) -> Result<Option<Duration>> {
        unimplemented!()
    }

    pub fn set_read_timeout(&self, duration_option: Option<Duration>) -> Result<()> {
        unimplemented!()
    }

    pub fn set_write_timeout(&self, duration_option: Option<Duration>) -> Result<()> {
        unimplemented!()
    }

    pub fn broadcast(&self) -> Result<bool> {
        unimplemented!()
    }

    pub fn set_broadcast(&self, _broadcast: bool) -> Result<()> {
        unimplemented!()
    }

    pub fn multicast_loop_v4(&self) -> Result<bool> {
        unimplemented!()
    }

    pub fn multicast_loop_v6(&self) -> Result<bool> {
        unimplemented!()
    }

    pub fn multicast_ttl_v4(&self) -> Result<u32> {
        unimplemented!()
    }

    pub fn set_multicast_loop_v4(&self, _multicast_loop_v4: bool) -> Result<()> {
        unimplemented!()
    }

    pub fn set_multicast_loop_v6(&self, _multicast_loop_v6: bool) -> Result<()> {
        unimplemented!()
    }

    pub fn set_multicast_ttl_v4(&self, _multicast_ttl_v4: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn ttl(&self) -> Result<u32> {
        unimplemented!()
    }

    pub fn set_ttl(&self, ttl: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn join_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> Result<()> {
        unimplemented!()
    }

    pub fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn leave_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> Result<()> {
        unimplemented!()
    }

    pub fn leave_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> Result<()> {
        unimplemented!()
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        unimplemented!()
    }

    pub fn connect(&self, addr: &SocketAddr) -> Result<()> {
        unimplemented!()
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }

    pub fn peek(&self, _buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!()
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<()> {
        unimplemented!()
    }
}

pub struct LookupHost {
    _todo: (),
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub fn lookup_host(host: &str) -> Result<LookupHost> {
    unimplemented!()
}
