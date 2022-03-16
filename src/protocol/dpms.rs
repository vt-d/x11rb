// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DPMS` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use std::io::IoSlice;
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::connection::Connection as X11Connection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::ConnectionError;
#[allow(unused_imports)]
use crate::errors::ReplyOrIdError;

pub use x11rb_protocol::protocol::dpms::*;

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

fn send_get_version<'c, Conn>(req: GetVersionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_version<Conn>(conn: &Conn, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Conn, GetVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetVersionRequest {
        client_major_version,
        client_minor_version,
    };
    send_get_version(request0, conn)
}

fn send_capable<'c, Conn>(req: CapableRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, CapableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn capable<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, CapableReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CapableRequest;
    send_capable(request0, conn)
}

fn send_get_timeouts<'c, Conn>(req: GetTimeoutsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetTimeoutsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_timeouts<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetTimeoutsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetTimeoutsRequest;
    send_get_timeouts(request0, conn)
}

fn send_set_timeouts<'c, Conn>(req: SetTimeoutsRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_timeouts<Conn>(conn: &Conn, standby_timeout: u16, suspend_timeout: u16, off_timeout: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetTimeoutsRequest {
        standby_timeout,
        suspend_timeout,
        off_timeout,
    };
    send_set_timeouts(request0, conn)
}

fn send_enable<'c, Conn>(req: EnableRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn enable<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = EnableRequest;
    send_enable(request0, conn)
}

fn send_disable<'c, Conn>(req: DisableRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn disable<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DisableRequest;
    send_disable(request0, conn)
}

fn send_force_level<'c, Conn>(req: ForceLevelRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn force_level<Conn>(conn: &Conn, power_level: DPMSMode) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ForceLevelRequest {
        power_level,
    };
    send_force_level(request0, conn)
}

fn send_info<'c, Conn>(req: InfoRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, InfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn info<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, InfoReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InfoRequest;
    send_info(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn dpms_get_version(&self, client_major_version: u16, client_minor_version: u16) -> Result<Cookie<'_, Self, GetVersionReply>, ConnectionError>
    {
        get_version(self, client_major_version, client_minor_version)
    }
    fn dpms_capable(&self) -> Result<Cookie<'_, Self, CapableReply>, ConnectionError>
    {
        capable(self)
    }
    fn dpms_get_timeouts(&self) -> Result<Cookie<'_, Self, GetTimeoutsReply>, ConnectionError>
    {
        get_timeouts(self)
    }
    fn dpms_set_timeouts(&self, standby_timeout: u16, suspend_timeout: u16, off_timeout: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_timeouts(self, standby_timeout, suspend_timeout, off_timeout)
    }
    fn dpms_enable(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        enable(self)
    }
    fn dpms_disable(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        disable(self)
    }
    fn dpms_force_level(&self, power_level: DPMSMode) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        force_level(self, power_level)
    }
    fn dpms_info(&self) -> Result<Cookie<'_, Self, InfoReply>, ConnectionError>
    {
        info(self)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
