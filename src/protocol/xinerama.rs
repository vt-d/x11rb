// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Xinerama` X11 extension.

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
#[allow(unused_imports)]
use super::xproto;

pub use x11rb_protocol::protocol::xinerama::*;

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

fn send_query_version<'c, Conn>(req: QueryVersionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_version<Conn>(conn: &Conn, major: u8, minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major,
        minor,
    };
    send_query_version(request0, conn)
}

fn send_get_state<'c, Conn>(req: GetStateRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_state<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetStateRequest {
        window,
    };
    send_get_state(request0, conn)
}

fn send_get_screen_count<'c, Conn>(req: GetScreenCountRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetScreenCountReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_screen_count<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetScreenCountReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenCountRequest {
        window,
    };
    send_get_screen_count(request0, conn)
}

fn send_get_screen_size<'c, Conn>(req: GetScreenSizeRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetScreenSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_screen_size<Conn>(conn: &Conn, window: xproto::Window, screen: u32) -> Result<Cookie<'_, Conn, GetScreenSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetScreenSizeRequest {
        window,
        screen,
    };
    send_get_screen_size(request0, conn)
}

fn send_is_active<'c, Conn>(req: IsActiveRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, IsActiveReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn is_active<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, IsActiveReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = IsActiveRequest;
    send_is_active(request0, conn)
}

fn send_query_screens<'c, Conn>(req: QueryScreensRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryScreensReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_screens<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryScreensReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryScreensRequest;
    send_query_screens(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xinerama_query_version(&self, major: u8, minor: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major, minor)
    }
    fn xinerama_get_state(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetStateReply>, ConnectionError>
    {
        get_state(self, window)
    }
    fn xinerama_get_screen_count(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetScreenCountReply>, ConnectionError>
    {
        get_screen_count(self, window)
    }
    fn xinerama_get_screen_size(&self, window: xproto::Window, screen: u32) -> Result<Cookie<'_, Self, GetScreenSizeReply>, ConnectionError>
    {
        get_screen_size(self, window, screen)
    }
    fn xinerama_is_active(&self) -> Result<Cookie<'_, Self, IsActiveReply>, ConnectionError>
    {
        is_active(self)
    }
    fn xinerama_query_screens(&self) -> Result<Cookie<'_, Self, QueryScreensReply>, ConnectionError>
    {
        query_screens(self)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
