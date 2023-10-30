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

use core::fmt;

use crate::Word;
use crate::net;
use crate::net::io;

// CloseRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct CloseRequest {
	handle: net::Handle,
}

impl CloseRequest {
	pub fn handle(&self) -> net::Handle {
		self.handle
	}
}

impl io::Encode for CloseRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.handle.encode(w)
	}
}

// }}}

// CloseRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct CloseRequestBuf {
	inner: CloseRequest,
}

#[cfg(any(doc, feature = "alloc"))]
impl CloseRequestBuf {
	pub fn new() -> CloseRequestBuf {
		CloseRequestBuf {
			inner: CloseRequest {
				handle: net::Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: net::Handle) {
		self.inner.handle = handle
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<CloseRequest> for CloseRequestBuf {
	fn as_ref(&self) -> &CloseRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for CloseRequestBuf {
	fn clone(&self) -> Self {
		CloseRequestBuf {
			inner: CloseRequest {
				handle: self.inner.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for CloseRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CloseRequestBuf")
			.field("handle", &self.inner.handle)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for CloseRequestBuf {
	type Target = CloseRequest;
	fn deref(&self) -> &CloseRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CloseRequest> for CloseRequestBuf {
	fn eq(&self, other: &CloseRequest) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CloseRequestBuf> for CloseRequest {
	fn eq(&self, other: &CloseRequestBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&CloseRequest> for CloseRequestBuf {
	fn from(request: &CloseRequest) -> Self {
		CloseRequestBuf {
			inner: CloseRequest {
				handle: request.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for CloseRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for CloseRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let handle = net::Handle::decode(r)?;

		Ok(CloseRequestBuf {
			inner: CloseRequest { handle },
		})
	}
}

// }}}

// CloseReply {{{

#[derive(Eq, PartialEq)]
pub struct CloseReply {
	_p: (),
}

impl fmt::Debug for CloseReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CloseReply").finish()
	}
}

impl io::Encode for CloseReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		Word::new(0).encode(w)
	}
}

// }}}

// CloseReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct CloseReplyBuf {
	inner: CloseReply,
}

#[cfg(any(doc, feature = "alloc"))]
impl CloseReplyBuf {
	pub fn new() -> CloseReplyBuf {
		CloseReplyBuf {
			inner: CloseReply { _p: () },
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<CloseReply> for CloseReplyBuf {
	fn as_ref(&self) -> &CloseReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for CloseReplyBuf {
	fn clone(&self) -> Self {
		CloseReplyBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for CloseReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CloseReplyBuf").finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for CloseReplyBuf {
	type Target = CloseReply;
	fn deref(&self) -> &CloseReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CloseReply> for CloseReplyBuf {
	fn eq(&self, _other: &CloseReply) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CloseReplyBuf> for CloseReply {
	fn eq(&self, _other: &CloseReplyBuf) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&CloseReply> for CloseReplyBuf {
	fn from(_reply: &CloseReply) -> Self {
		CloseReplyBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for CloseReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for CloseReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _dummy = Word::decode(r)?;
		Ok(CloseReplyBuf::new())
	}
}

// }}}
