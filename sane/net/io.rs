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

	pub fn decoder<'a, R>(&self, r: &'a mut R) -> Decoder<'a, R> {
		Decoder { r }
	}

	pub fn encoder<'a, W>(&self, w: &'a mut W) -> Encoder<'a, W> {
		Encoder { w }
	}
}

// }}}

// Decode {{{

pub trait Decode: Sized {
	fn decode<R: Read>(
		decoder: &mut Decoder<R>,
	) -> Result<Self, DecodeError<R::Error>>;
}

pub struct Decoder<'a, R> {
	r: &'a mut R,
}

impl<R: Read> Decoder<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> Result<(), DecodeError<R::Error>> {
		self.r.read_exact(buf).map_err(|e| DecodeError::IoError(e))
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DecodeError<IoError> {
	InvalidBool(crate::Word),
	IoError(IoError),
}

// }}}

// Encode {{{

pub trait Encode {
	fn encode<W: Write>(
		&self,
		encoder: &mut Encoder<W>,
	) -> Result<(), EncodeError<W::Error>>;
}

pub struct Encoder<'a, W> {
	w: &'a mut W,
}

impl<W: Write> Encoder<'_, W> {
	fn write(&mut self, buf: &[u8]) -> Result<(), EncodeError<W::Error>> {
		self.w.write_all(buf).map_err(|e| EncodeError::IoError(e))
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EncodeError<IoError> {
	IoError(IoError),
}

// }}}

// [5.1.1] Primitive Data Types {{{

impl Decode for crate::Word {
	fn decode<R: Read>(d: &mut Decoder<R>) -> Result<Self, DecodeError<R::Error>> {
		let mut bytes = [0u8; 4];
		d.read(&mut bytes)?;
		Ok(Self::new(u32::from_be_bytes(bytes)))
	}
}

impl Encode for crate::Word {
	fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError<W::Error>> {
		let bytes = self.as_u32().to_be_bytes();
		e.write(&bytes)
	}
}

impl Decode for crate::Bool {
	fn decode<R: Read>(d: &mut Decoder<R>) -> Result<Self, DecodeError<R::Error>> {
		let word = crate::Word::decode(d)?;
		if word.as_u32() == 0 {
			return Ok(Self::FALSE);
		}
		if word.as_u32() == 1 {
			return Ok(Self::TRUE);
		}
		Err(DecodeError::InvalidBool(word))
	}
}

impl Encode for crate::Bool {
	fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), EncodeError<W::Error>> {
		self.as_word().encode(e)
	}
}

macro_rules! decode_encode_as_word {
	($type:ty) => {
		impl Decode for $type {
			fn decode<R: Read>(
				d: &mut Decoder<R>,
			) -> Result<Self, DecodeError<R::Error>> {
				Ok(Self::from_word(crate::Word::decode(d)?))
			}
		}

		impl Encode for $type {
			fn encode<W: Write>(
				&self,
				e: &mut Encoder<W>,
			) -> Result<(), EncodeError<W::Error>> {
				self.as_word().encode(e)
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
