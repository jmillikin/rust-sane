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

use crate::Word;
use crate::net::io;
#[cfg(any(doc, feature = "alloc"))]
use crate::util;

// AuthorizeRequest {{{

#[derive(Eq, PartialEq)]
pub struct AuthorizeRequest {
	inner: AuthorizeRequestInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct AuthorizeRequestInner<'a> {
	resource: &'a CStr,
	username: &'a CStr,
	password: &'a CStr,
}

impl fmt::Debug for AuthorizeRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "AuthorizeRequest")
	}
}

impl AuthorizeRequest {
	pub fn resource(&self) -> &CStr {
		self.inner.resource
	}

	pub fn username(&self) -> &CStr {
		self.inner.username
	}

	pub fn password(&self) -> &CStr {
		self.inner.password
	}
}

impl<'a> AuthorizeRequestInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("resource", &self.resource)
			.field("username", &self.username)
			.field("password", &self.password)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a AuthorizeRequest {
		unsafe {
			let ptr: *const AuthorizeRequestInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for AuthorizeRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.resource().encode(w)?;
		self.username().encode(w)?;
		self.password().encode(w)
	}
}

// }}}

// AuthorizeRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct AuthorizeRequestBuf {
	inner: AuthorizeRequestInner<'static>,
	resource: Cow<'static, CStr>,
	username: Cow<'static, CStr>,
	password: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl AuthorizeRequestBuf {
	pub fn new() -> AuthorizeRequestBuf {
		AuthorizeRequestBuf {
			inner: AuthorizeRequestInner {
				resource: util::CSTR_EMPTY,
				username: util::CSTR_EMPTY,
				password: util::CSTR_EMPTY,
			},
			resource: Cow::Borrowed(util::CSTR_EMPTY),
			username: Cow::Borrowed(util::CSTR_EMPTY),
			password: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_resource(&mut self, resource: impl Into<CString>) {
		let resource = resource.into();
		self.inner.resource = unsafe { util::cstr_to_static(&resource) };
		self.resource = Cow::Owned(resource);
	}

	pub fn set_username(&mut self, username: impl Into<CString>) {
		let username = username.into();
		self.inner.username = unsafe { util::cstr_to_static(&username) };
		self.username = Cow::Owned(username);
	}

	pub fn set_password(&mut self, password: impl Into<CString>) {
		let password = password.into();
		self.inner.password = unsafe { util::cstr_to_static(&password) };
		self.password = Cow::Owned(password);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<AuthorizeRequest> for AuthorizeRequestBuf {
	fn as_ref(&self) -> &AuthorizeRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for AuthorizeRequestBuf {
	fn clone(&self) -> Self {
		AuthorizeRequestBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for AuthorizeRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "AuthorizeRequestBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for AuthorizeRequestBuf {
	type Target = AuthorizeRequest;
	fn deref(&self) -> &AuthorizeRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for AuthorizeRequestBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for AuthorizeRequestBuf {
	fn eq(&self, other: &AuthorizeRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<AuthorizeRequest> for AuthorizeRequestBuf {
	fn eq(&self, other: &AuthorizeRequest) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<AuthorizeRequestBuf> for AuthorizeRequest {
	fn eq(&self, other: &AuthorizeRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&AuthorizeRequest> for AuthorizeRequestBuf {
	fn from(request: &AuthorizeRequest) -> Self {
		let mut buf = AuthorizeRequestBuf::new();
		if !request.resource().is_empty() {
			buf.set_resource(request.resource());
		}
		if !request.username().is_empty() {
			buf.set_username(request.username());
		}
		if !request.password().is_empty() {
			buf.set_password(request.password());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for AuthorizeRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for AuthorizeRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let resource = CString::decode(r)?;
		let username = CString::decode(r)?;
		let password = CString::decode(r)?;

		let mut buf = AuthorizeRequestBuf::new();
		if !resource.is_empty() {
			buf.set_resource(resource);
		}
		if !username.is_empty() {
			buf.set_username(username);
		}
		if !password.is_empty() {
			buf.set_password(password);
		}
		Ok(buf)
	}
}

// }}}

// AuthorizeReply {{{

#[derive(Eq, PartialEq)]
pub struct AuthorizeReply {
	_p: (),
}

impl fmt::Debug for AuthorizeReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("AuthorizeReply").finish()
	}
}

impl io::Encode for AuthorizeReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		Word::new(0).encode(w)
	}
}

// }}}

// AuthorizeReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct AuthorizeReplyBuf {
	inner: AuthorizeReply,
}

#[cfg(any(doc, feature = "alloc"))]
impl AuthorizeReplyBuf {
	pub fn new() -> AuthorizeReplyBuf {
		AuthorizeReplyBuf {
			inner: AuthorizeReply { _p: () },
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<AuthorizeReply> for AuthorizeReplyBuf {
	fn as_ref(&self) -> &AuthorizeReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for AuthorizeReplyBuf {
	fn clone(&self) -> Self {
		AuthorizeReplyBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for AuthorizeReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("AuthorizeReplyBuf").finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for AuthorizeReplyBuf {
	type Target = AuthorizeReply;
	fn deref(&self) -> &AuthorizeReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<AuthorizeReply> for AuthorizeReplyBuf {
	fn eq(&self, _other: &AuthorizeReply) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<AuthorizeReplyBuf> for AuthorizeReply {
	fn eq(&self, _other: &AuthorizeReplyBuf) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&AuthorizeReply> for AuthorizeReplyBuf {
	fn from(_reply: &AuthorizeReply) -> Self {
		AuthorizeReplyBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for AuthorizeReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for AuthorizeReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _dummy = Word::decode(r)?;
		Ok(AuthorizeReplyBuf::new())
	}
}

// }}}
