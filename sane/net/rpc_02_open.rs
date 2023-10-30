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

use crate::Status;
use crate::net;
use crate::net::io;
#[cfg(any(doc, feature = "alloc"))]
use crate::util;

// OpenRequest {{{

#[derive(Eq, PartialEq)]
pub struct OpenRequest {
	inner: OpenRequestInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct OpenRequestInner<'a> {
	device_name: &'a CStr,
}

impl fmt::Debug for OpenRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenRequest")
	}
}

impl OpenRequest {
	pub fn device_name(&self) -> &CStr {
		self.inner.device_name
	}
}

impl<'a> OpenRequestInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("device_name", &self.device_name)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a OpenRequest {
		unsafe {
			let ptr: *const OpenRequestInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for OpenRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.device_name().encode(w)
	}
}

// }}}

// OpenRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct OpenRequestBuf {
	inner: OpenRequestInner<'static>,
	device_name: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl OpenRequestBuf {
	pub fn new() -> OpenRequestBuf {
		OpenRequestBuf {
			inner: OpenRequestInner {
				device_name: util::CSTR_EMPTY,
			},
			device_name: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_device_name(&mut self, device_name: impl Into<CString>) {
		let device_name = device_name.into();
		self.inner.device_name = unsafe { util::cstr_to_static(&device_name) };
		self.device_name = Cow::Owned(device_name);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<OpenRequest> for OpenRequestBuf {
	fn as_ref(&self) -> &OpenRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for OpenRequestBuf {
	fn clone(&self) -> Self {
		OpenRequestBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for OpenRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenRequestBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for OpenRequestBuf {
	type Target = OpenRequest;
	fn deref(&self) -> &OpenRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for OpenRequestBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for OpenRequestBuf {
	fn eq(&self, other: &OpenRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenRequest> for OpenRequestBuf {
	fn eq(&self, other: &OpenRequest) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenRequestBuf> for OpenRequest {
	fn eq(&self, other: &OpenRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&OpenRequest> for OpenRequestBuf {
	fn from(request: &OpenRequest) -> Self {
		let mut buf = OpenRequestBuf::new();
		if !request.device_name().is_empty() {
			buf.set_device_name(request.device_name());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for OpenRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for OpenRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let device_name = CString::decode(r)?;

		let mut buf = OpenRequestBuf::new();
		if !device_name.is_empty() {
			buf.set_device_name(device_name);
		}
		Ok(buf)
	}
}

// }}}

// OpenReply {{{

#[derive(Eq, PartialEq)]
pub struct OpenReply {
	inner: OpenReplyInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct OpenReplyInner<'a> {
	status: Status,
	handle: net::Handle,
	resource: &'a CStr,
}

impl fmt::Debug for OpenReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenReply")
	}
}

impl OpenReply {
	pub fn status(&self) -> Status {
		self.inner.status
	}

	pub fn handle(&self) -> net::Handle {
		self.inner.handle
	}

	pub fn resource(&self) -> &CStr {
		self.inner.resource
	}
}

impl<'a> OpenReplyInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("status", &self.status)
			.field("handle", &self.handle)
			.field("resource", &self.resource)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a OpenReply {
		unsafe {
			let ptr: *const OpenReplyInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for OpenReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		self.handle().encode(w)?;
		self.resource().encode(w)
	}
}

// }}}

// OpenReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct OpenReplyBuf {
	inner: OpenReplyInner<'static>,
	resource: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl OpenReplyBuf {
	pub fn new() -> OpenReplyBuf {
		OpenReplyBuf {
			inner: OpenReplyInner {
				status: Status::GOOD,
				handle: net::Handle(0),
				resource: util::CSTR_EMPTY,
			},
			resource: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_status(&mut self, status: Status) {
		self.inner.status = status;
	}

	pub fn set_handle(&mut self, handle: net::Handle) {
		self.inner.handle = handle;
	}

	pub fn set_resource(&mut self, resource: impl Into<CString>) {
		let resource = resource.into();
		self.inner.resource = unsafe { util::cstr_to_static(&resource) };
		self.resource = Cow::Owned(resource);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<OpenReply> for OpenReplyBuf {
	fn as_ref(&self) -> &OpenReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for OpenReplyBuf {
	fn clone(&self) -> Self {
		OpenReplyBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for OpenReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenReplyBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for OpenReplyBuf {
	type Target = OpenReply;
	fn deref(&self) -> &OpenReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for OpenReplyBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for OpenReplyBuf {
	fn eq(&self, other: &OpenReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenReply> for OpenReplyBuf {
	fn eq(&self, other: &OpenReply) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenReplyBuf> for OpenReply {
	fn eq(&self, other: &OpenReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&OpenReply> for OpenReplyBuf {
	fn from(reply: &OpenReply) -> Self {
		let mut buf = OpenReplyBuf::new();
		buf.set_status(reply.status());
		buf.set_handle(reply.handle());
		if !reply.resource().is_empty() {
			buf.set_resource(reply.resource());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for OpenReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for OpenReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let status = Status::decode(r)?;
		let handle = net::Handle::decode(r)?;
		let resource = CString::decode(r)?;

		let mut buf = OpenReplyBuf::new();
		buf.set_status(status);
		buf.set_handle(handle);
		if !resource.is_empty() {
			buf.set_resource(resource);
		}
		Ok(buf)
	}
}

// }}}
