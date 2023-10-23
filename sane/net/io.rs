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
	kind: DecodeErrorKind<IoError>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DecodeErrorKind<IoError> {
	InvalidBool(Word),
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
	kind: EncodeErrorKind<IoError>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum EncodeErrorKind<IoError> {
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
}

// }}}

// Writer {{{

pub struct Writer<'a, W> {
	w: &'a mut W,
}

impl<W: Write> Writer<'_, W> {
	fn write_bytes(&mut self, buf: &[u8]) -> Result<(), EncodeError<W::Error>> {
		self.w.write_all(buf).map_err(|e| EncodeError::io_err(e))
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

// }}}
