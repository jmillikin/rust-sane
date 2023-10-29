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

// InitRequest {{{

#[derive(Eq, PartialEq)]
pub struct InitRequest {
	inner: InitRequestInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct InitRequestInner<'a> {
	version_code: u32,
	username: &'a CStr,
}

impl fmt::Debug for InitRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "InitRequest")
	}
}

impl InitRequest {
	pub fn version_code(&self) -> u32 {
		self.inner.version_code
	}

	pub fn username(&self) -> &CStr {
		self.inner.username
	}
}

impl<'a> InitRequestInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("version_code", &self.version_code)
			.field("username", &self.username)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a InitRequest {
		unsafe {
			let ptr: *const InitRequestInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for InitRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		net::ProcedureNumber::INIT.encode(w)?;
		Word::new(self.version_code()).encode(w)?;
		self.username().encode(w)
	}
}

// }}}

// InitRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct InitRequestBuf {
	inner: InitRequestInner<'static>,
	username: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl InitRequestBuf {
	pub fn new() -> InitRequestBuf {
		InitRequestBuf {
			inner: InitRequestInner {
				version_code: net::VERSION_CODE,
				username: util::CSTR_EMPTY,
			},
			username: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_version_code(&mut self, version_code: u32) {
		self.inner.version_code = version_code;
	}

	pub fn set_username(&mut self, username: impl Into<CString>) {
		let username = username.into();
		self.inner.username = unsafe { util::cstr_to_static(&username) };
		self.username = Cow::Owned(username);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<InitRequest> for InitRequestBuf {
	fn as_ref(&self) -> &InitRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for InitRequestBuf {
	fn clone(&self) -> Self {
		InitRequestBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for InitRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "InitRequestBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for InitRequestBuf {
	type Target = InitRequest;
	fn deref(&self) -> &InitRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for InitRequestBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for InitRequestBuf {
	fn eq(&self, other: &InitRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<InitRequest> for InitRequestBuf {
	fn eq(&self, other: &InitRequest) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<InitRequestBuf> for InitRequest {
	fn eq(&self, other: &InitRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&InitRequest> for InitRequestBuf {
	fn from(request: &InitRequest) -> Self {
		let mut buf = InitRequestBuf::new();
		buf.set_version_code(request.version_code());
		if !request.username().is_empty() {
			buf.set_username(request.username());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for InitRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for InitRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _proc_no = net::ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is INIT
		let version_code = Word::decode(r)?.as_u32();
		let username = CString::decode(r)?;

		let mut buf = InitRequestBuf::new();
		buf.set_version_code(version_code);
		if !username.is_empty() {
			buf.set_username(username);
		}

		Ok(buf)
	}
}

// }}}

// InitReply {{{

#[derive(Debug, Eq, PartialEq)]
pub struct InitReply {
	status: Status,
	version_code: u32,
}

impl InitReply {
	pub fn status(&self) -> Status {
		self.status
	}

	pub fn version_code(&self) -> u32 {
		self.version_code
	}
}

impl io::Encode for InitReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		Word::new(self.version_code()).encode(w)
	}
}

// }}}

// InitReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct InitReplyBuf {
	inner: InitReply,
}

#[cfg(any(doc, feature = "alloc"))]
impl InitReplyBuf {
	pub fn new() -> InitReplyBuf {
		InitReplyBuf {
			inner: InitReply {
				status: Status::GOOD,
				version_code: net::VERSION_CODE,
			},
		}
	}

	pub fn set_status(&mut self, status: Status) {
		self.inner.status = status;
	}

	pub fn set_version_code(&mut self, version_code: u32) {
		self.inner.version_code = version_code;
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<InitReply> for InitReplyBuf {
	fn as_ref(&self) -> &InitReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for InitReplyBuf {
	fn clone(&self) -> Self {
		InitReplyBuf {
			inner: InitReply {
				status: self.inner.status,
				version_code: self.inner.version_code,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for InitReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("InitReplyBuf")
			.field("status", &self.status)
			.field("version_code", &self.version_code)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for InitReplyBuf {
	type Target = InitReply;
	fn deref(&self) -> &InitReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<InitReply> for InitReplyBuf {
	fn eq(&self, other: &InitReply) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<InitReplyBuf> for InitReply {
	fn eq(&self, other: &InitReplyBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&InitReply> for InitReplyBuf {
	fn from(reply: &InitReply) -> Self {
		InitReplyBuf {
			inner: InitReply {
				status: reply.status,
				version_code: reply.version_code,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for InitReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for InitReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let status = Status::decode(r)?;
		let version_code = Word::decode(r)?.as_u32();

		Ok(InitReplyBuf {
			inner: InitReply { status, version_code },
		})
	}
}

// }}}

