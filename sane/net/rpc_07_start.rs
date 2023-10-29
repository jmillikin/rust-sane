// Copyright (c) 2023 John Millikin <john@john-millikin.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//
// SPDX-License-Identifier: 0BSD

#[cfg(any(doc, feature = "alloc"))]
use alloc::borrow::Cow;
#[cfg(any(doc, feature = "alloc"))]
use alloc::ffi::CString;

use core::ffi::CStr;
use core::fmt;

use crate::{Status, Word};
use crate::net;
use crate::net::io;
#[cfg(any(doc, feature = "alloc"))]
use crate::util;

// StartRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct StartRequest {
	handle: net::Handle,
}

impl StartRequest {
	pub fn handle(&self) -> net::Handle {
		self.handle
	}
}

impl io::Encode for StartRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		net::ProcedureNumber::START.encode(w)?;
		self.handle.encode(w)
	}
}

// }}}

// StartRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct StartRequestBuf {
	inner: StartRequest,
}

#[cfg(any(doc, feature = "alloc"))]
impl StartRequestBuf {
	pub fn new() -> StartRequestBuf {
		StartRequestBuf {
			inner: StartRequest {
				handle: net::Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: net::Handle) {
		self.inner.handle = handle
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<StartRequest> for StartRequestBuf {
	fn as_ref(&self) -> &StartRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for StartRequestBuf {
	fn clone(&self) -> Self {
		StartRequestBuf {
			inner: StartRequest {
				handle: self.inner.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for StartRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("StartRequestBuf")
			.field("handle", &self.inner.handle)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for StartRequestBuf {
	type Target = StartRequest;
	fn deref(&self) -> &StartRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartRequest> for StartRequestBuf {
	fn eq(&self, other: &StartRequest) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartRequestBuf> for StartRequest {
	fn eq(&self, other: &StartRequestBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&StartRequest> for StartRequestBuf {
	fn from(request: &StartRequest) -> Self {
		StartRequestBuf {
			inner: StartRequest {
				handle: request.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for StartRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for StartRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _proc_no = net::ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is START
		let handle = net::Handle::decode(r)?;

		Ok(StartRequestBuf {
			inner: StartRequest { handle },
		})
	}
}

// }}}

// StartReply {{{

#[derive(Eq, PartialEq)]
pub struct StartReply {
	inner: StartReplyInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct StartReplyInner<'a> {
	status: Status,
	port: u16,
	byte_order: net::ByteOrder,
	resource: &'a CStr,
}

impl fmt::Debug for StartReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "StartReply")
	}
}

impl StartReply {
	pub fn status(&self) -> Status {
		self.inner.status
	}

	pub fn port(&self) -> u16 {
		self.inner.port
	}

	pub fn byte_order(&self) -> net::ByteOrder {
		self.inner.byte_order
	}

	pub fn resource(&self) -> &CStr {
		self.inner.resource
	}
}

impl<'a> StartReplyInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("status", &self.status)
			.field("port", &self.port)
			.field("byte_order", &self.byte_order)
			.field("resource", &self.resource)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a StartReply {
		unsafe {
			let ptr: *const StartReplyInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for StartReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		Word::new(u32::from(self.port())).encode(w)?;
		self.byte_order().encode(w)?;
		self.resource().encode(w)
	}
}

// }}}

// StartReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct StartReplyBuf {
	inner: StartReplyInner<'static>,
	resource: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl StartReplyBuf {
	pub fn new() -> StartReplyBuf {
		StartReplyBuf {
			inner: StartReplyInner {
				status: Status::GOOD,
				port: 0,
				byte_order: net::ByteOrder::LITTLE_ENDIAN,
				resource: util::CSTR_EMPTY,
			},
			resource: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_status(&mut self, status: Status) {
		self.inner.status = status;
	}

	pub fn set_port(&mut self, port: u16) {
		self.inner.port = port;
	}

	pub fn set_byte_order(&mut self, byte_order: net::ByteOrder) {
		self.inner.byte_order = byte_order;
	}

	pub fn set_resource(&mut self, resource: impl Into<CString>) {
		let resource = resource.into();
		self.inner.resource = unsafe { util::cstr_to_static(&resource) };
		self.resource = Cow::Owned(resource);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<StartReply> for StartReplyBuf {
	fn as_ref(&self) -> &StartReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for StartReplyBuf {
	fn clone(&self) -> Self {
		StartReplyBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for StartReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "StartReplyBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for StartReplyBuf {
	type Target = StartReply;
	fn deref(&self) -> &StartReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for StartReplyBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for StartReplyBuf {
	fn eq(&self, other: &StartReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartReply> for StartReplyBuf {
	fn eq(&self, other: &StartReply) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartReplyBuf> for StartReply {
	fn eq(&self, other: &StartReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&StartReply> for StartReplyBuf {
	fn from(reply: &StartReply) -> Self {
		let mut buf = StartReplyBuf::new();
		buf.set_status(reply.status());
		buf.set_port(reply.port());
		buf.set_byte_order(reply.byte_order());
		if !reply.resource().is_empty() {
			buf.set_resource(reply.resource());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for StartReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for StartReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let status = Status::decode(r)?;
		let port = Word::decode(r)?.as_u32();
		let byte_order = net::ByteOrder::decode(r)?;
		let resource = CString::decode(r)?;

		// FIXME: error if port > u16::MAX

		let mut buf = StartReplyBuf::new();
		buf.set_status(status);
		buf.set_port(port as u16);
		buf.set_byte_order(byte_order);
		if !resource.is_empty() {
			buf.set_resource(resource);
		}
		Ok(buf)
	}
}

// }}}
