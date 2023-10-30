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

// CancelRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct CancelRequest {
	handle: net::Handle,
}

impl CancelRequest {
	pub fn handle(&self) -> net::Handle {
		self.handle
	}
}

impl io::Encode for CancelRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.handle.encode(w)
	}
}

// }}}

// CancelRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct CancelRequestBuf {
	inner: CancelRequest,
}

#[cfg(any(doc, feature = "alloc"))]
impl CancelRequestBuf {
	pub fn new() -> CancelRequestBuf {
		CancelRequestBuf {
			inner: CancelRequest {
				handle: net::Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: net::Handle) {
		self.inner.handle = handle
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<CancelRequest> for CancelRequestBuf {
	fn as_ref(&self) -> &CancelRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for CancelRequestBuf {
	fn clone(&self) -> Self {
		CancelRequestBuf {
			inner: CancelRequest {
				handle: self.inner.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for CancelRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CancelRequestBuf")
			.field("handle", &self.inner.handle)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for CancelRequestBuf {
	type Target = CancelRequest;
	fn deref(&self) -> &CancelRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CancelRequest> for CancelRequestBuf {
	fn eq(&self, other: &CancelRequest) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CancelRequestBuf> for CancelRequest {
	fn eq(&self, other: &CancelRequestBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&CancelRequest> for CancelRequestBuf {
	fn from(request: &CancelRequest) -> Self {
		CancelRequestBuf {
			inner: CancelRequest {
				handle: request.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for CancelRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for CancelRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let handle = net::Handle::decode(r)?;

		Ok(CancelRequestBuf {
			inner: CancelRequest { handle },
		})
	}
}

// }}}

// CancelReply {{{

#[derive(Eq, PartialEq)]
pub struct CancelReply {
	_p: (),
}

impl fmt::Debug for CancelReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CancelReply").finish()
	}
}

impl io::Encode for CancelReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		Word::new(0).encode(w)
	}
}

// }}}

// CancelReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct CancelReplyBuf {
	inner: CancelReply,
}

#[cfg(any(doc, feature = "alloc"))]
impl CancelReplyBuf {
	pub fn new() -> CancelReplyBuf {
		CancelReplyBuf {
			inner: CancelReply { _p: () },
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<CancelReply> for CancelReplyBuf {
	fn as_ref(&self) -> &CancelReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for CancelReplyBuf {
	fn clone(&self) -> Self {
		CancelReplyBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for CancelReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CancelReplyBuf").finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for CancelReplyBuf {
	type Target = CancelReply;
	fn deref(&self) -> &CancelReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CancelReply> for CancelReplyBuf {
	fn eq(&self, _other: &CancelReply) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<CancelReplyBuf> for CancelReply {
	fn eq(&self, _other: &CancelReplyBuf) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&CancelReply> for CancelReplyBuf {
	fn from(_reply: &CancelReply) -> Self {
		CancelReplyBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for CancelReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for CancelReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _dummy = Word::decode(r)?;
		Ok(CancelReplyBuf::new())
	}
}

// }}}
