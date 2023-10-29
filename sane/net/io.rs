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
use alloc::{
	ffi::CString,
	vec::Vec,
};

use core::{
	convert::TryFrom,
	ffi::CStr,
};

use crate::Word;

// Read {{{

pub trait Read {
	type Error;

	fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

#[cfg(any(doc, feature = "std"))]
impl Read for std::net::TcpStream {
	type Error = std::io::Error;

	fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
		std::io::Read::read_exact(self, buf)
	}
}

#[cfg(any(doc, feature = "std"))]
impl<R: std::io::Read> Read for std::io::BufReader<R> {
	type Error = std::io::Error;

	fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
		std::io::Read::read_exact(self, buf)
	}
}

#[cfg(any(doc, feature = "std"))]
impl<T: AsRef<[u8]>> Read for std::io::Cursor<T> {
	type Error = std::io::Error;

	fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
		std::io::Read::read_exact(self, buf)
	}
}

// }}}

// Write {{{

pub trait Write {
	type Error;

	fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(any(doc, feature = "std"))]
impl Write for std::net::TcpStream {
	type Error = std::io::Error;

	fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
		std::io::Write::write_all(self, buf)
	}
}

#[cfg(any(doc, feature = "std"))]
impl<W: std::io::Write> Write for std::io::BufWriter<W> {
	type Error = std::io::Error;

	fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
		std::io::Write::write_all(self, buf)
	}
}

#[cfg(any(doc, feature = "std"))]
impl Write for std::io::Cursor<&mut Vec<u8>> {
	type Error = std::io::Error;

	fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
		std::io::Write::write_all(self, buf)
	}
}

// }}}

// Codec {{{

pub struct Codec {
	_p: (),
}

impl Codec {
	pub const BINARY_V3: Codec = Codec { _p: () };

	pub fn reader<'a, R>(&self, r: &'a mut R) -> Reader<'a, R> {
		Reader { r }
	}

	pub fn writer<'a, W>(&self, w: &'a mut W) -> Writer<'a, W> {
		Writer { w }
	}
}

// }}}

// Decode {{{

pub trait Decode: Sized {
	fn decode<R: Read>(
		reader: &mut Reader<R>,
	) -> Result<Self, DecodeError<R::Error>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeError<IoError> {
	pub(crate) kind: DecodeErrorKind<IoError>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum DecodeErrorKind<IoError> {
	SizeOverflow(u32),
	TryReserveError(usize),
	InvalidString,
	InvalidOptionType,
	InvalidBool(Word),
	InvalidValueType(crate::ValueType),
	InvalidConstraint(crate::ValueType, crate::ConstraintType),
	NullPtr,
	IoError(IoError),
}

impl<IoError> DecodeError<IoError> {
	fn io_err(err: IoError) -> Self {
		DecodeError {
			kind: DecodeErrorKind::IoError(err),
		}
	}
}

// }}}

// Encode {{{

pub trait Encode {
	fn encode<W: Write>(
		&self,
		writer: &mut Writer<W>,
	) -> Result<(), EncodeError<W::Error>>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncodeError<IoError> {
	pub(crate) kind: EncodeErrorKind<IoError>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum EncodeErrorKind<IoError> {
	SizeOverflow(usize),
	InvalidOptionType,
	IoError(IoError),
}

impl<IoError> EncodeError<IoError> {
	fn io_err(err: IoError) -> Self {
		EncodeError {
			kind: EncodeErrorKind::IoError(err),
		}
	}
}

// }}}

// Reader {{{

pub struct Reader<'a, R> {
	r: &'a mut R,
}

impl<R: Read> Reader<'_, R> {
	fn read_bytes(
		&mut self,
		buf: &mut [u8],
	) -> Result<(), DecodeError<R::Error>> {
		self.r.read_exact(buf).map_err(|e| DecodeError::io_err(e))
	}

	#[cfg(any(doc, feature = "alloc"))]
	pub(crate) fn read_vec(
		&mut self,
		len: usize,
	) -> Result<Vec<u8>, DecodeError<R::Error>> {
		let mut bytes = Vec::new();
		if len == 0 {
			return Ok(bytes);
		}
		if let Err(_) = bytes.try_reserve(len) {
			return Err(DecodeError {
				kind: DecodeErrorKind::TryReserveError(len),
			});
		}
		bytes.resize(len, 0u8);
		self.read_bytes(&mut bytes)?;
		Ok(bytes)
	}

	#[cfg(any(doc, feature = "alloc"))]
	pub(crate) fn read_size(&mut self) -> Result<usize, DecodeError<R::Error>> {
		let size = Word::decode(self)?.as_u32();
		match usize::try_from(size) {
			Ok(size) => Ok(size),
			Err(_) => Err(DecodeError {
				kind: DecodeErrorKind::SizeOverflow(size),
			}),
		}
	}

	#[cfg(any(doc, feature = "alloc"))]
	pub(crate) fn read_ptr<T: Decode>(
		&mut self
	) -> Result<Option<T>, DecodeError<R::Error>> {
		let is_null = crate::Bool::decode(self)?;
		if is_null == crate::Bool::TRUE {
			return Ok(None);
		}
		Ok(Some(T::decode(self)?))
	}
}

// }}}

// Writer {{{

pub struct Writer<'a, W> {
	w: &'a mut W,
}

impl<W: Write> Writer<'_, W> {
	pub(crate) fn write_bytes(
		&mut self,
		buf: &[u8],
	) -> Result<(), EncodeError<W::Error>> {
		self.w.write_all(buf).map_err(|e| EncodeError::io_err(e))
	}

	pub(crate) fn write_size(
		&mut self,
		size: usize,
	) -> Result<(), EncodeError<W::Error>> {
		match u32::try_from(size) {
			Ok(size) => Word::new(size).encode(self),
			Err(_) => Err(EncodeError {
				kind: EncodeErrorKind::SizeOverflow(size),
			}),
		}
	}

	pub(crate) fn write_ptr<T: Encode>(
		&mut self,
		value: Option<&T>,
	) -> Result<(), EncodeError<W::Error>> {
		match value {
			None => crate::Bool::TRUE.encode(self),
			Some(value) => {
				crate::Bool::FALSE.encode(self)?;
				value.encode(self)
			},
		}
	}
}

// }}}

// [5.1.1] Primitive Data Types {{{

impl Decode for Word {
	fn decode<R: Read>(
		r: &mut Reader<R>,
	) -> Result<Self, DecodeError<R::Error>> {
		let mut bytes = [0u8; 4];
		r.read_bytes(&mut bytes)?;
		Ok(Self::new(u32::from_be_bytes(bytes)))
	}
}

impl Encode for Word {
	fn encode<W: Write>(
		&self,
		w: &mut Writer<W>,
	) -> Result<(), EncodeError<W::Error>> {
		let bytes = self.as_u32().to_be_bytes();
		w.write_bytes(&bytes)
	}
}

impl Decode for crate::Bool {
	fn decode<R: Read>(
		r: &mut Reader<R>,
	) -> Result<Self, DecodeError<R::Error>> {
		let word = Word::decode(r)?;
		match word.as_u32() {
			0 => Ok(Self::FALSE),
			1 => Ok(Self::TRUE),
			_ => Err(DecodeError {
				kind: DecodeErrorKind::InvalidBool(word),
			}),
		}
	}
}

impl Encode for crate::Bool {
	fn encode<W: Write>(
		&self,
		w: &mut Writer<W>,
	) -> Result<(), EncodeError<W::Error>> {
		self.as_word().encode(w)
	}
}

macro_rules! decode_encode_as_word {
	($type:ty) => {
		impl Decode for $type {
			fn decode<R: Read>(
				r: &mut Reader<R>,
			) -> Result<Self, DecodeError<R::Error>> {
				Ok(Self::from_word(Word::decode(r)?))
			}
		}

		impl Encode for $type {
			fn encode<W: Write>(
				&self,
				w: &mut Writer<W>,
			) -> Result<(), EncodeError<W::Error>> {
				self.as_word().encode(w)
			}
		}
	};
}

decode_encode_as_word!(crate::Int);
decode_encode_as_word!(crate::Fixed);

decode_encode_as_word!(crate::Status);
decode_encode_as_word!(crate::ValueType);
decode_encode_as_word!(crate::Unit);
decode_encode_as_word!(crate::ConstraintType);
decode_encode_as_word!(crate::Action);
decode_encode_as_word!(crate::Frame);
decode_encode_as_word!(crate::net::ByteOrder);
decode_encode_as_word!(crate::net::ProcedureNumber);

impl Encode for CStr {
	fn encode<W: Write>(
		&self,
		w: &mut Writer<W>,
	) -> Result<(), EncodeError<W::Error>> {
		let bytes = self.to_bytes_with_nul();
		w.write_size(bytes.len())?;
		w.write_bytes(bytes)
	}
}

impl Encode for Option<&CStr> {
	fn encode<W: Write>(
		&self,
		w: &mut Writer<W>,
	) -> Result<(), EncodeError<W::Error>> {
		match self {
			None => Word::new(0).encode(w),
			Some(cstr) => cstr.encode(w),
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Encode for CString {
	fn encode<W: Write>(
		&self,
		w: &mut Writer<W>,
	) -> Result<(), EncodeError<W::Error>> {
		self.as_c_str().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Encode for Option<CString> {
	fn encode<W: Write>(
		&self,
		w: &mut Writer<W>,
	) -> Result<(), EncodeError<W::Error>> {
		self.as_ref().map(|s| s.as_c_str()).encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Decode for CString {
	fn decode<R: Read>(
		r: &mut Reader<R>,
	) -> Result<Self, DecodeError<R::Error>> {
		if let Some(value) = Decode::decode(r)? {
			return Ok(value);
		}
		try_cstring_new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
fn try_cstring_new<E>() -> Result<CString, DecodeError<E>> {
	let len = 1;
	let mut vec = Vec::new();
	if let Err(_) = vec.try_reserve(len) {
		return Err(DecodeError {
			kind: DecodeErrorKind::TryReserveError(len),
		});
	}
	vec.resize(len, 0u8);
	Ok(unsafe { CString::from_vec_with_nul_unchecked(vec) })
}

#[cfg(any(doc, feature = "alloc"))]
impl Decode for Option<CString> {
	fn decode<R: Read>(
		r: &mut Reader<R>,
	) -> Result<Self, DecodeError<R::Error>> {
		let bytes_len = r.read_size()?;
		if bytes_len == 0 {
			return Ok(None);
		}
		let bytes = r.read_vec(bytes_len)?;
		if let Some(cstring) = cstring_from_vec_until_nul(bytes) {
			return Ok(Some(cstring));
		}
		Err(DecodeError {
			kind: DecodeErrorKind::InvalidString,
		})
	}
}

#[cfg(any(doc, feature = "alloc"))]
fn cstring_from_vec_until_nul(mut bytes: Vec<u8>) -> Option<CString> {
	let nul_idx = bytes.iter().position(|&b| b == 0)?;
	let new_len = nul_idx + 1;
	if new_len < bytes.len() {
		bytes.truncate(new_len);
	}
	Some(unsafe { CString::from_vec_with_nul_unchecked(bytes) })
}

// }}}
