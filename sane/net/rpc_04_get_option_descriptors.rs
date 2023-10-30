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
use alloc::ffi::CString;
#[cfg(any(doc, feature = "alloc"))]
use alloc::vec::Vec;

use core::ffi::CStr;
use core::fmt;

#[allow(unused_imports)]
use crate::{
	Bool,
	ConstraintType,
	Fixed,
	Int,
	Range,
	ValueType,
	Word,
};
use crate::net;
use crate::net::io;
use crate::util;

// GetOptionDescriptorsRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct GetOptionDescriptorsRequest {
	handle: net::Handle,
}

impl GetOptionDescriptorsRequest {
	pub fn handle(&self) -> net::Handle {
		self.handle
	}
}

impl io::Encode for GetOptionDescriptorsRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		net::ProcedureNumber::GET_OPTION_DESCRIPTORS.encode(w)?;
		self.handle.encode(w)
	}
}

// }}}

// GetOptionDescriptorsRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct GetOptionDescriptorsRequestBuf {
	inner: GetOptionDescriptorsRequest,
}

#[cfg(any(doc, feature = "alloc"))]
impl GetOptionDescriptorsRequestBuf {
	pub fn new() -> GetOptionDescriptorsRequestBuf {
		GetOptionDescriptorsRequestBuf {
			inner: GetOptionDescriptorsRequest {
				handle: net::Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: net::Handle) {
		self.inner.handle = handle
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<GetOptionDescriptorsRequest> for GetOptionDescriptorsRequestBuf {
	fn as_ref(&self) -> &GetOptionDescriptorsRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for GetOptionDescriptorsRequestBuf {
	fn clone(&self) -> Self {
		GetOptionDescriptorsRequestBuf {
			inner: GetOptionDescriptorsRequest {
				handle: self.inner.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for GetOptionDescriptorsRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GetOptionDescriptorsRequestBuf")
			.field("handle", &self.inner.handle)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for GetOptionDescriptorsRequestBuf {
	type Target = GetOptionDescriptorsRequest;
	fn deref(&self) -> &GetOptionDescriptorsRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetOptionDescriptorsRequest> for GetOptionDescriptorsRequestBuf {
	fn eq(&self, other: &GetOptionDescriptorsRequest) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetOptionDescriptorsRequestBuf> for GetOptionDescriptorsRequest {
	fn eq(&self, other: &GetOptionDescriptorsRequestBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&GetOptionDescriptorsRequest> for GetOptionDescriptorsRequestBuf {
	fn from(request: &GetOptionDescriptorsRequest) -> Self {
		GetOptionDescriptorsRequestBuf {
			inner: GetOptionDescriptorsRequest {
				handle: request.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for GetOptionDescriptorsRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for GetOptionDescriptorsRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _proc_no = net::ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is GET_OPTION_DESCRIPTORS
		let handle = net::Handle::decode(r)?;

		Ok(GetOptionDescriptorsRequestBuf {
			inner: GetOptionDescriptorsRequest { handle },
		})
	}
}

// }}}

// GetOptionDescriptorsReply {{{

#[derive(Eq, PartialEq)]
pub struct GetOptionDescriptorsReply {
	inner: GetOptionDescriptorsReplyInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct GetOptionDescriptorsReplyInner<'a> {
	option_descriptors: &'a [&'a util::OptionDescriptor],
}

impl fmt::Debug for GetOptionDescriptorsReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "GetOptionDescriptorsReply")
	}
}

impl GetOptionDescriptorsReply {
	pub fn option_descriptors(&self) -> &[&util::OptionDescriptor] {
		self.inner.option_descriptors
	}
}

impl<'a> GetOptionDescriptorsReplyInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("option_descriptors", &self.option_descriptors)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a GetOptionDescriptorsReply {
		unsafe {
			let ptr: *const GetOptionDescriptorsReplyInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for GetOptionDescriptorsReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		Word::new((self.option_descriptors().len() + 1) as u32).encode(w)?;
		for opt_desc in self.option_descriptors() {
			w.write_ptr(Some(*opt_desc))?;
		}
		w.write_ptr(Option::<&util::OptionDescriptor>::None)
	}
}

// }}}

// GetOptionDescriptorsReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct GetOptionDescriptorsReplyBuf {
	inner: GetOptionDescriptorsReplyInner<'static>,
	opt_descs: Vec<util::OptionDescriptorBuf>,
	opt_desc_refs: Vec<&'static util::OptionDescriptor>,
}

#[cfg(any(doc, feature = "alloc"))]
impl GetOptionDescriptorsReplyBuf {
	pub fn new() -> GetOptionDescriptorsReplyBuf {
		GetOptionDescriptorsReplyBuf {
			inner: GetOptionDescriptorsReplyInner {
				option_descriptors: &[],
			},
			opt_descs: Vec::new(),
			opt_desc_refs: Vec::new(),
		}
	}

	pub fn set_option_descriptors(
		&mut self,
		option_descriptors: impl Into<Vec<util::OptionDescriptorBuf>>,
	) {
		let opt_descs = option_descriptors.into();
		self.opt_descs = opt_descs;
		self.opt_desc_refs.truncate(0);
		for opt_desc in self.opt_descs.iter() {
			self.opt_desc_refs.push(unsafe {
				core::mem::transmute(opt_desc.as_ref())
			});
		}
		self.inner.option_descriptors = unsafe {
			core::mem::transmute(self.opt_desc_refs.as_slice())
		};
	}

	pub fn into_option_descriptors(self) -> Vec<util::OptionDescriptorBuf> {
		self.opt_descs
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<GetOptionDescriptorsReply> for GetOptionDescriptorsReplyBuf {
	fn as_ref(&self) -> &GetOptionDescriptorsReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for GetOptionDescriptorsReplyBuf {
	fn clone(&self) -> Self {
		GetOptionDescriptorsReplyBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for GetOptionDescriptorsReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "GetOptionDescriptorsReplyBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for GetOptionDescriptorsReplyBuf {
	type Target = GetOptionDescriptorsReply;
	fn deref(&self) -> &GetOptionDescriptorsReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for GetOptionDescriptorsReplyBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for GetOptionDescriptorsReplyBuf {
	fn eq(&self, other: &GetOptionDescriptorsReplyBuf) -> bool {
		self.option_descriptors() == other.option_descriptors()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetOptionDescriptorsReply> for GetOptionDescriptorsReplyBuf {
	fn eq(&self, other: &GetOptionDescriptorsReply) -> bool {
		self.option_descriptors() == other.option_descriptors()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetOptionDescriptorsReplyBuf> for GetOptionDescriptorsReply {
	fn eq(&self, other: &GetOptionDescriptorsReplyBuf) -> bool {
		self.option_descriptors() == other.option_descriptors()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&GetOptionDescriptorsReply> for GetOptionDescriptorsReplyBuf {
	fn from(reply: &GetOptionDescriptorsReply) -> Self {
		let opt_descs: Vec<util::OptionDescriptorBuf> =
			reply.option_descriptors()
				.iter()
				.map(|&d| d.into())
				.collect();
		let mut buf = GetOptionDescriptorsReplyBuf::new();
		buf.set_option_descriptors(opt_descs);
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for GetOptionDescriptorsReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for GetOptionDescriptorsReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let opt_descs_len = r.read_size()?;
		let mut opt_descs = Vec::with_capacity(opt_descs_len);
		for _ii in 0..opt_descs_len {
			let is_null = Bool::decode(r)?;
			if is_null == Bool::TRUE {
				break;
			}

			// FIXME: verify NUL termination: there should only be a single
			//   NULL option descriptor, and it should be at the end of the list
			//   (ii == opt_descs_len-1)

			opt_descs.push(util::OptionDescriptorBuf::decode(r)?);
		}

		let mut buf = GetOptionDescriptorsReplyBuf::new();
		buf.set_option_descriptors(opt_descs);
		Ok(buf)
	}
}

// }}}

impl io::Decode for Range {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let mut range = Range::new();
		range.min = Word::decode(r)?;
		range.max = Word::decode(r)?;
		range.quant = Word::decode(r)?;
		Ok(range)
	}
}

impl io::Encode for Range {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.min.encode(w)?;
		self.max.encode(w)?;
		self.quant.encode(w)
	}
}

impl io::Encode for util::OptionDescriptor {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.name().encode(w)?;
		self.title().encode(w)?;
		self.description().encode(w)?;
		self.value_type().encode(w)?;
		self.unit().encode(w)?;
		Int::new(self.size() as i32).encode(w)?; // FIXME: integer cast
		self.capabilities().as_word().encode(w)?;

		match self.constraint() {
			util::Constraint::None => {
				ConstraintType::NONE.encode(w)
			},
			util::Constraint::IntRange(range) => {
				ConstraintType::RANGE.encode(w)?;
				w.write_ptr(Some(range))
			},
			util::Constraint::FixedRange(range) => {
				ConstraintType::RANGE.encode(w)?;
				w.write_ptr(Some(range))
			},
			util::Constraint::IntList(values) => {
				ConstraintType::WORD_LIST.encode(w)?;
				let len = values.iter().count();
				w.write_size(len+1)?; // FIXME: overflow?
				w.write_size(len)?;
				for word in values.iter() {
					word.encode(w)?;
				}
				Ok(())
			},
			util::Constraint::FixedList(values) => {
				ConstraintType::WORD_LIST.encode(w)?;
				let len = values.iter().count();
				w.write_size(len+1)?; // FIXME: overflow?
				w.write_size(len)?;
				for word in values.iter() {
					word.encode(w)?;
				}
				Ok(())
			},
			util::Constraint::StringList(values) => {
				ConstraintType::STRING_LIST.encode(w)?;
				let len = values.iter().count();
				w.write_size(len+1)?; // FIXME: overflow?
				for cstr in values.iter() {
					cstr.encode(w)?;
				}
				Option::<&CStr>::None.encode(w)
			},
		}
	}
}

impl io::Encode for util::OptionDescriptorRef<'_> {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for util::OptionDescriptorBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for util::OptionDescriptorBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let name = CString::decode(r)?;
		let title = CString::decode(r)?;
		let desc = CString::decode(r)?;
		let opt_type = ValueType::decode(r)?;
		let unit = crate::Unit::decode(r)?;
		let size = r.read_size()?;
		let cap = util::Capabilities::from_word(Word::decode(r)?);

		match opt_type {
			ValueType::BOOL => {
				expect_constraint_none(r)?;
				assert_eq!(size, 4); // FIXME: return error, not assert
				Ok(util::BoolOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.capabilities(cap)
					.build())
			},
			ValueType::INT => {
				let builder = util::IntOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.unit(unit)
					.capabilities(cap);
				read_int_option(r, builder, size)
			},
			ValueType::FIXED => {
				let builder = util::FixedOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.unit(unit)
					.capabilities(cap);
				read_fixed_option(r, builder, size)
			},
			ValueType::STRING => {
				let builder = util::StringOptionBuilder::new(name, size)
					.title(title)
					.description(desc)
					.unit(unit)
					.capabilities(cap);
				read_string_option(r, builder)
			},
			ValueType::BUTTON => {
				expect_constraint_none(r)?;
				// check size == 0 ?
				Ok(util::ButtonOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.capabilities(cap)
					.build())
			},
			ValueType::GROUP => {
				expect_constraint_none(r)?;
				// check size == 0 ?
				Ok(util::GroupOptionBuilder::new()
					.title(title)
					.description(desc)
					.build())
			},
			_ => Err(io::DecodeError::<R::Error> {
				kind: io::DecodeErrorKind::InvalidValueType(opt_type),
			}),
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
fn expect_constraint_none<R: io::Read>(
	r: &mut io::Reader<R>,
) -> Result<(), io::DecodeError<R::Error>> {
	use io::Decode;

	let constraint_type = ConstraintType::decode(r)?;
	if constraint_type == ConstraintType::NONE {
		return Ok(());
	}
	Err(io::DecodeError {
		kind: io::DecodeErrorKind::InvalidConstraint(
			ValueType::BOOL,
			constraint_type,
		),
	})
}

#[cfg(any(doc, feature = "alloc"))]
fn read_int_option<R: io::Read>(
	r: &mut io::Reader<R>,
	mut builder: util::IntOptionBuilder,
	size: usize,
) -> Result<util::OptionDescriptorBuf, io::DecodeError<R::Error>> {
	use io::Decode;

	assert_eq!(size % 4, 0); // FIXME: return error, not assert
	assert!(size >= 4); // FIXME: return error, not assert
	let count = size / 4;
	builder = builder.count(count);

	let constraint_type = ConstraintType::decode(r)?;
	match constraint_type {
		ConstraintType::NONE => {},
		ConstraintType::RANGE => {
			let Some(range) = r.read_ptr()? else {
				return Err(io::DecodeError {
					kind: io::DecodeErrorKind::NullPtr,
				});
			};
			let range: Range = range;
			builder = builder.range(
				Int::from_word(range.min).as_i32(),
				Int::from_word(range.max).as_i32(),
				Int::from_word(range.quant).as_i32(),
			);
		},
		ConstraintType::WORD_LIST => {
			builder = builder.values(read_i32_list(r)?);
		},
		_ => {
			return Err(io::DecodeError {
				kind: io::DecodeErrorKind::InvalidConstraint(
					ValueType::INT,
					constraint_type,
				),
			});
		},
	};

	Ok(builder.build())
}

#[cfg(any(doc, feature = "alloc"))]
fn read_fixed_option<R: io::Read>(
	r: &mut io::Reader<R>,
	mut builder: util::FixedOptionBuilder,
	size: usize,
) -> Result<util::OptionDescriptorBuf, io::DecodeError<R::Error>> {
	use io::Decode;

	assert_eq!(size % 4, 0); // FIXME: return error, not assert
	assert!(size >= 4); // FIXME: return error, not assert
	let count = size / 4;
	builder = builder.count(count);

	let constraint_type = ConstraintType::decode(r)?;
	match constraint_type {
		ConstraintType::NONE => {},
		ConstraintType::RANGE => {
			let Some(range) = r.read_ptr()? else {
				return Err(io::DecodeError {
					kind: io::DecodeErrorKind::NullPtr,
				});
			};
			let range: Range = range;
			builder = builder.range(
				Fixed::from_word(range.min),
				Fixed::from_word(range.max),
				Fixed::from_word(range.quant),
			);
		},
		ConstraintType::WORD_LIST => {
			builder = builder.values(read_fixed_list(r)?);
		},
		_ => {
			return Err(io::DecodeError {
				kind: io::DecodeErrorKind::InvalidConstraint(
					ValueType::FIXED,
					constraint_type,
				),
			});
		},
	};

	Ok(builder.build())
}

#[cfg(any(doc, feature = "alloc"))]
fn read_string_option<R: io::Read>(
	r: &mut io::Reader<R>,
	mut builder: util::StringOptionBuilder,
) -> Result<util::OptionDescriptorBuf, io::DecodeError<R::Error>> {
	use io::Decode;

	let constraint_type = ConstraintType::decode(r)?;
	match constraint_type {
		ConstraintType::NONE => {},
		ConstraintType::STRING_LIST => {
			builder = builder.values(read_cstring_list(r)?);
		},
		_ => {
			return Err(io::DecodeError {
				kind: io::DecodeErrorKind::InvalidConstraint(
					ValueType::STRING,
					constraint_type,
				),
			});
		},
	};

	Ok(builder.build())
}

#[cfg(any(doc, feature = "alloc"))]
fn new_vec_for_array<R: io::Read, T>(
	r: &mut io::Reader<R>,
) -> Result<(usize, Vec<T>), io::DecodeError<R::Error>> {
	let len = r.read_size()?;
	if len == 0 {
		// FIXME: return an error, as this is a protocol violation
		return Ok((len, Vec::new()));
	}

	let mut vec = Vec::new();
	if let Err(_) = vec.try_reserve(len-1) {
		return Err(io::DecodeError {
			kind: io::DecodeErrorKind::TryReserveError(len),
		});
	}

	Ok((len, vec))
}

#[cfg(any(doc, feature = "alloc"))]
fn read_i32_list<R: io::Read>(
	r: &mut io::Reader<R>,
) -> Result<Vec<i32>, io::DecodeError<R::Error>> {
	use io::Decode;
	let (len, mut vec) = new_vec_for_array(r)?;
	for ii in 0..len {
		let word = Word::decode(r)?;
		// FIXME: validate that the first item in the word list
		// is the expected length?
		if ii > 0 {
			vec.push(Int::from_word(word).as_i32());
		}
	}
	Ok(vec)
}

#[cfg(any(doc, feature = "alloc"))]
fn read_fixed_list<R: io::Read>(
	r: &mut io::Reader<R>,
) -> Result<Vec<Fixed>, io::DecodeError<R::Error>> {
	use io::Decode;
	let (len, mut vec) = new_vec_for_array(r)?;
	for ii in 0..len {
		let word = Word::decode(r)?;
		// FIXME: validate that the first item in the word list
		// is the expected length?
		if ii > 0 {
			vec.push(Fixed::from_word(word));
		}
	}
	Ok(vec)
}

#[cfg(any(doc, feature = "alloc"))]
fn read_cstring_list<R: io::Read>(
	r: &mut io::Reader<R>,
) -> Result<Vec<CString>, io::DecodeError<R::Error>> {
	use io::Decode;
	let (len, mut vec) = new_vec_for_array(r)?;
	for _ii in 0..len {
		let value = Option::<CString>::decode(r)?;
		// FIXME: all values should be non-NULL until ii==len-1, which must
		// be NULL.
		if let Some(value) = value {
			vec.push(value);
		}
	}
	Ok(vec)
}
