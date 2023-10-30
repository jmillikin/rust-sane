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
use core::fmt;

use crate::{Bool, Int, Parameters, Status};
use crate::net;
use crate::net::io;

// GetParametersRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct GetParametersRequest {
	handle: net::Handle,
}

impl GetParametersRequest {
	pub fn handle(&self) -> net::Handle {
		self.handle
	}
}

impl io::Encode for GetParametersRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.handle.encode(w)
	}
}

// }}}

// GetParametersRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct GetParametersRequestBuf {
	inner: GetParametersRequest,
}

#[cfg(any(doc, feature = "alloc"))]
impl GetParametersRequestBuf {
	pub fn new() -> GetParametersRequestBuf {
		GetParametersRequestBuf {
			inner: GetParametersRequest {
				handle: net::Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: net::Handle) {
		self.inner.handle = handle
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<GetParametersRequest> for GetParametersRequestBuf {
	fn as_ref(&self) -> &GetParametersRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for GetParametersRequestBuf {
	fn clone(&self) -> Self {
		GetParametersRequestBuf {
			inner: GetParametersRequest {
				handle: self.inner.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for GetParametersRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GetParametersRequestBuf")
			.field("handle", &self.inner.handle)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for GetParametersRequestBuf {
	type Target = GetParametersRequest;
	fn deref(&self) -> &GetParametersRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetParametersRequest> for GetParametersRequestBuf {
	fn eq(&self, other: &GetParametersRequest) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetParametersRequestBuf> for GetParametersRequest {
	fn eq(&self, other: &GetParametersRequestBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&GetParametersRequest> for GetParametersRequestBuf {
	fn from(request: &GetParametersRequest) -> Self {
		GetParametersRequestBuf {
			inner: GetParametersRequest {
				handle: request.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for GetParametersRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for GetParametersRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let handle = net::Handle::decode(r)?;

		Ok(GetParametersRequestBuf {
			inner: GetParametersRequest { handle },
		})
	}
}

// }}}

// GetParametersReply {{{

#[derive(Debug, Eq, PartialEq)]
pub struct GetParametersReply {
	status: Status,
	parameters: Parameters,
}

impl GetParametersReply {
	pub fn status(&self) -> Status {
		self.status
	}

	pub fn parameters(&self) -> &Parameters {
		&self.parameters
	}
}

impl io::Encode for GetParametersReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		self.parameters().encode(w)
	}
}

// }}}

// GetParametersReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct GetParametersReplyBuf {
	inner: GetParametersReply,
}

#[cfg(any(doc, feature = "alloc"))]
impl GetParametersReplyBuf {
	pub fn new() -> GetParametersReplyBuf {
		GetParametersReplyBuf {
			inner: GetParametersReply {
				status: Status::GOOD,
				parameters: Parameters::new(),
			},
		}
	}

	pub fn set_status(&mut self, status: Status) {
		self.inner.status = status;
	}

	pub fn set_parameters(&mut self, parameters: impl Into<Parameters>) {
		self.inner.parameters = parameters.into();
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<GetParametersReply> for GetParametersReplyBuf {
	fn as_ref(&self) -> &GetParametersReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for GetParametersReplyBuf {
	fn clone(&self) -> Self {
		GetParametersReplyBuf {
			inner: GetParametersReply {
				status: self.inner.status,
				parameters: self.inner.parameters,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for GetParametersReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GetParametersReplyBuf")
			.field("status", &self.status)
			.field("parameters", &self.parameters)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for GetParametersReplyBuf {
	type Target = GetParametersReply;
	fn deref(&self) -> &GetParametersReply {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetParametersReply> for GetParametersReplyBuf {
	fn eq(&self, other: &GetParametersReply) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetParametersReplyBuf> for GetParametersReply {
	fn eq(&self, other: &GetParametersReplyBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&GetParametersReply> for GetParametersReplyBuf {
	fn from(reply: &GetParametersReply) -> Self {
		GetParametersReplyBuf {
			inner: GetParametersReply {
				status: reply.status,
				parameters: reply.parameters,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for GetParametersReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for GetParametersReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let status = Status::decode(r)?;
		let parameters = Parameters::decode(r)?;

		Ok(GetParametersReplyBuf {
			inner: GetParametersReply { status, parameters },
		})
	}
}

// }}}

impl io::Decode for Parameters {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let mut params = Parameters::new();
		params.format = crate::Frame::decode(r)?;
		params.last_frame = Bool::decode(r)?;
		params.bytes_per_line = Int::decode(r)?;
		params.pixels_per_line = Int::decode(r)?;
		params.lines = Int::decode(r)?;
		params.depth = Int::decode(r)?;
		Ok(params)
	}
}

impl io::Encode for Parameters {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.format.encode(w)?;
		self.last_frame.encode(w)?;
		self.bytes_per_line.encode(w)?;
		self.pixels_per_line.encode(w)?;
		self.lines.encode(w)?;
		self.depth.encode(w)
	}
}
