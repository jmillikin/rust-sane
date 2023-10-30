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
#[cfg(any(doc, feature = "alloc"))]
use alloc::vec::Vec;

use core::ffi::CStr;
use core::fmt;

#[allow(unused_imports)]
use crate::{
	Action,
	Bool,
	Fixed,
	Int,
	Status,
	ValueType,
	Word,
};
use crate::net;
use crate::net::io;
use crate::util;

// ControlOptionRequest {{{

#[derive(Eq, PartialEq)]
pub struct ControlOptionRequest {
	inner: ControlOptionRequestInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct ControlOptionRequestInner<'a> {
	handle: net::Handle,
	option: u32,
	action: Action,
	value_type: ValueType,
	value: &'a [u8],
}

impl fmt::Debug for ControlOptionRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "ControlOptionRequest")
	}
}

impl ControlOptionRequest {
	pub fn handle(&self) -> net::Handle {
		self.inner.handle
	}

	pub fn option(&self) -> u32 {
		self.inner.option
	}

	pub fn action(&self) -> Action {
		self.inner.action
	}

	pub fn value_type(&self) -> ValueType {
		self.inner.value_type
	}

	pub fn value(&self) -> OptionValue<'_> {
		OptionValue {
			value_type: self.inner.value_type,
			bytes: self.inner.value,
		}
	}
}

impl<'a> ControlOptionRequestInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("handle", &self.handle)
			.field("option", &self.option)
			.field("action", &self.action)
			.field("value_type", &self.value_type)
			.field("value", &self.value)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a ControlOptionRequest {
		unsafe {
			let ptr: *const ControlOptionRequestInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for ControlOptionRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		net::ProcedureNumber::CONTROL_OPTION.encode(w)?;
		self.handle().encode(w)?;
		Word::new(self.option()).encode(w)?;
		self.action().encode(w)?;
		if self.action() != Action::SET_AUTO {
			self.value().encode(w)?;
		}
		Ok(())
	}
}

// }}}

// ControlOptionRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct ControlOptionRequestBuf {
	inner: ControlOptionRequestInner<'static>,
	value: Cow<'static, [u8]>,
}

#[cfg(any(doc, feature = "alloc"))]
impl ControlOptionRequestBuf {
	pub fn new() -> ControlOptionRequestBuf {
		ControlOptionRequestBuf {
			inner: ControlOptionRequestInner {
				handle: net::Handle(0),
				option: 0,
				action: Action::GET_VALUE,
				value_type: ValueType::BOOL,
				value: &[],
			},
			value: Cow::Borrowed(&[]),
		}
	}

	pub fn set_handle(&mut self, handle: net::Handle) {
		self.inner.handle = handle;
	}

	pub fn set_option(&mut self, option: u32) {
		self.inner.option = option;
	}

	pub fn set_action(&mut self, action: Action) {
		self.inner.action = action;
	}

	pub fn set_value(&mut self, value: impl Into<OptionValueBuf>) {
		let value = value.into();
		let bytes = value.bytes;
		self.inner.value_type = value.value_type;
		self.inner.value = unsafe { core::mem::transmute(bytes.as_slice()) };
		self.value = Cow::Owned(bytes);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<ControlOptionRequest> for ControlOptionRequestBuf {
	fn as_ref(&self) -> &ControlOptionRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for ControlOptionRequestBuf {
	fn clone(&self) -> Self {
		ControlOptionRequestBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for ControlOptionRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "ControlOptionRequestBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for ControlOptionRequestBuf {
	type Target = ControlOptionRequest;
	fn deref(&self) -> &ControlOptionRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for ControlOptionRequestBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for ControlOptionRequestBuf {
	fn eq(&self, other: &ControlOptionRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<ControlOptionRequest> for ControlOptionRequestBuf {
	fn eq(&self, other: &ControlOptionRequest) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<ControlOptionRequestBuf> for ControlOptionRequest {
	fn eq(&self, other: &ControlOptionRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&ControlOptionRequest> for ControlOptionRequestBuf {
	fn from(request: &ControlOptionRequest) -> Self {
		let mut buf = ControlOptionRequestBuf::new();
		buf.set_handle(request.handle());
		buf.set_option(request.option());
		buf.set_action(request.action());
		buf.set_value(request.value());
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for ControlOptionRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for ControlOptionRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _proc_no = net::ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is CONTROL_OPTION

		let mut buf = ControlOptionRequestBuf::new();
		buf.set_handle(net::Handle::decode(r)?);
		buf.set_option(Word::decode(r)?.as_u32());
		buf.set_action(Action::decode(r)?);
		if buf.action() != Action::SET_AUTO {
			buf.set_value(OptionValueBuf::decode(r)?);
		}
		Ok(buf)
	}
}

// }}}

// ControlOptionReply {{{

#[derive(Eq, PartialEq)]
pub struct ControlOptionReply {
	inner: ControlOptionReplyInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct ControlOptionReplyInner<'a> {
	status: Status,
	info: u32,
	value_type: ValueType,
	value: &'a [u8],
	resource: &'a CStr,
}

impl fmt::Debug for ControlOptionReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "ControlOptionReply")
	}
}

impl ControlOptionReply {
	pub fn status(&self) -> Status {
		self.inner.status
	}

	pub fn info(&self) -> u32 {
		self.inner.info
	}

	pub fn value_type(&self) -> ValueType {
		self.inner.value_type
	}

	pub fn value(&self) -> OptionValue<'_> {
		OptionValue {
			value_type: self.inner.value_type,
			bytes: self.inner.value,
		}
	}

	pub fn resource(&self) -> &CStr {
		self.inner.resource
	}
}

impl<'a> ControlOptionReplyInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("status", &self.status)
			.field("info", &self.info)
			.field("value_type", &self.value_type)
			.field("value", &self.value)
			.field("resource", &self.resource)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a ControlOptionReply {
		unsafe {
			let ptr: *const ControlOptionReplyInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for ControlOptionReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		Word::new(self.info()).encode(w)?;
		self.value().encode(w)?;
		self.resource().encode(w)
	}
}

// }}}

// ControlOptionReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct ControlOptionReplyBuf {
	inner: ControlOptionReplyInner<'static>,
	resource: Cow<'static, CStr>,
	value: Cow<'static, [u8]>,
}

#[cfg(any(doc, feature = "alloc"))]
impl ControlOptionReplyBuf {
	pub fn new() -> ControlOptionReplyBuf {
		ControlOptionReplyBuf {
			inner: ControlOptionReplyInner {
				status: Status::GOOD,
				info: 0,
				value_type: ValueType::BOOL,
				value: &[],
				resource: util::CSTR_EMPTY,
			},
			value: Cow::Borrowed(&[]),
			resource: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_status(&mut self, status: Status) {
		self.inner.status = status;
	}

	pub fn set_info(&mut self, info: u32) {
		self.inner.info = info;
	}

	pub fn set_value(&mut self, value: impl Into<OptionValueBuf>) {
		let value = value.into();
		let bytes = value.bytes;
		self.inner.value_type = value.value_type;
		self.inner.value = unsafe { core::mem::transmute(bytes.as_slice()) };
		self.value = Cow::Owned(bytes);
	}

	pub fn set_resource(&mut self, resource: impl Into<CString>) {
		let resource = resource.into();
		self.inner.resource = unsafe { util::cstr_to_static(&resource) };
		self.resource = Cow::Owned(resource);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<ControlOptionReply> for ControlOptionReplyBuf {
	fn as_ref(&self) -> &ControlOptionReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for ControlOptionReplyBuf {
	fn clone(&self) -> Self {
		ControlOptionReplyBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for ControlOptionReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "ControlOptionReplyBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for ControlOptionReplyBuf {
	type Target = ControlOptionReply;
	fn deref(&self) -> &ControlOptionReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for ControlOptionReplyBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for ControlOptionReplyBuf {
	fn eq(&self, other: &ControlOptionReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<ControlOptionReply> for ControlOptionReplyBuf {
	fn eq(&self, other: &ControlOptionReply) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<ControlOptionReplyBuf> for ControlOptionReply {
	fn eq(&self, other: &ControlOptionReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&ControlOptionReply> for ControlOptionReplyBuf {
	fn from(request: &ControlOptionReply) -> Self {
		let mut buf = ControlOptionReplyBuf::new();
		buf.set_status(request.status());
		buf.set_info(request.info());
		buf.set_value(request.value());
		if !request.resource().is_empty() {
			buf.set_resource(request.resource());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for ControlOptionReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for ControlOptionReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let mut buf = ControlOptionReplyBuf::new();
		buf.set_status(Status::decode(r)?);
		buf.set_info(Word::decode(r)?.as_u32());
		buf.set_value(OptionValueBuf::decode(r)?);
		let resource = CString::decode(r)?;
		if !resource.is_empty() {
			buf.set_resource(resource);
		}
		Ok(buf)
	}
}

// }}}

// OptionValue {{{

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OptionValue<'a> {
	value_type: ValueType,
	bytes: &'a [u8],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionValueError {
	_p: (),
}

impl OptionValueError {
	fn err<T>() -> Result<T, OptionValueError> {
		Err(OptionValueError { _p: () })
	}
}

impl OptionValue<'_> {
	pub const BUTTON: OptionValue<'static> = OptionValue {
		value_type: ValueType::BUTTON,
		bytes: &[],
	};
}

impl<'a> OptionValue<'a> {
	pub fn as_bytes(&self) -> &[u8] {
		self.bytes
	}

	fn to_u32(self) -> Result<u32, OptionValueError> {
		if self.bytes.len() != 4 {
			return OptionValueError::err();
		}
		let (word_bytes, _) = util::split_array_ref(self.bytes);
		Ok(u32::from_be_bytes(*word_bytes))
	}

	pub fn to_bool(self) -> Result<bool, OptionValueError> {
		match self.to_u32()? {
			0 => Ok(false),
			1 => Ok(true),
			_ => OptionValueError::err(),
		}
	}

	pub fn to_i32(self) -> Result<i32, OptionValueError> {
		Ok(self.to_u32()? as i32)
	}

	#[cfg(any(doc, feature = "alloc"))]
	pub fn to_i32_list(self) -> Result<Vec<i32>, OptionValueError> {
		let mut bytes = self.bytes;

		// FIXME
		let len = bytes.len() / 4;
		if bytes.len() != 4 * len {
			return OptionValueError::err();
		}

		let mut vec = Vec::with_capacity(len);
		for _ii in 0..len {
			let word_bytes: &[u8; 4];
			(word_bytes, bytes) = util::split_array_ref(bytes);
			vec.push(i32::from_be_bytes(*word_bytes));
		}
		Ok(vec)
	}

	pub fn to_fixed(self) -> Result<Fixed, OptionValueError> {
		Ok(Fixed::from_word(Word::new(self.to_u32()?)))
	}

	#[cfg(any(doc, feature = "alloc"))]
	pub fn to_fixed_list(self) -> Result<Vec<Fixed>, OptionValueError> {
		let mut bytes = self.bytes;

		// FIXME
		let len = bytes.len() / 4;
		if bytes.len() != 4 * len {
			return OptionValueError::err();
		}

		let mut vec = Vec::with_capacity(len);
		for _ii in 0..len {
			let word_bytes: &[u8; 4];
			(word_bytes, bytes) = util::split_array_ref(bytes);
			let word = Word::new(u32::from_be_bytes(*word_bytes));
			vec.push(Fixed::from_word(word));
		}
		Ok(vec)
	}

	pub fn to_cstr(self) -> Result<&'a CStr, OptionValueError> {
		let mut bytes = self.bytes;
		let nul_idx = match bytes.iter().position(|&b| b == 0) {
			Some(x) => x,
			None => return OptionValueError::err(),
		};
		let new_len = nul_idx + 1;
		if new_len < bytes.len() {
			bytes = &bytes[..new_len];
		}
		Ok(unsafe { CStr::from_bytes_with_nul_unchecked(bytes) })
	}

	#[cfg(any(doc, feature = "alloc"))]
	pub fn to_cstring(self) -> Result<CString, OptionValueError> {
		Ok(CString::from(self.to_cstr()?))
	}

	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		use io::Encode;
		use ValueType as T;

		let mut value_size: Option<u32> = None;
		let mut value_count: Option<u32> = None;
		match self.value_type {
			T::BOOL => {
				assert_eq!(self.bytes.len(), 4);
				value_size = Some(4);
				value_count = Some(1);
			},
			T::INT | T::FIXED => {
				assert_eq!(self.bytes.len() % 4, 0);
				value_size = Some(self.bytes.len() as u32);
				value_count = Some((self.bytes.len() / 4) as u32);
			},
			T::STRING => {
				assert!(self.bytes.len() > 0);
				value_size = Some(self.bytes.len() as u32);
			},
			T::BUTTON => {},
			_ => return Err(io::EncodeError {
				kind: io::EncodeErrorKind::InvalidOptionType,
			}),
		}

		self.value_type.encode(w)?;
		if let Some(value_size) = value_size {
			Word::new(value_size).encode(w)?;
		}
		if let Some(value_count) = value_count {
			Word::new(value_count).encode(w)?;
		}
		if self.bytes.len() > 0 {
			w.write_bytes(self.bytes)?;
		}
		Ok(())
	}
}

// }}}

// OptionValueBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionValueBuf {
	value_type: ValueType,
	bytes: Vec<u8>,
}

#[cfg(any(doc, feature = "alloc"))]
impl OptionValueBuf {
	pub fn as_bytes(&self) -> &[u8] {
		&self.bytes
	}

	pub fn from_bool(value: bool) -> OptionValueBuf {
		let value = Bool::new(value).as_word().as_u32();
		OptionValueBuf {
			value_type: ValueType::BOOL,
			bytes: Vec::from(value.to_be_bytes()),
		}
	}

	pub fn from_i32(value: i32) -> OptionValueBuf {
		OptionValueBuf {
			value_type: ValueType::INT,
			bytes: Vec::from(value.to_be_bytes()),
		}
	}

	pub fn from_i32_list(values: &[i32]) -> OptionValueBuf {
		let mut bytes = Vec::with_capacity(4 * values.len());
		for value in values {
			let value = Int::new(*value).as_word().as_u32();
			bytes.extend_from_slice(&value.to_be_bytes());
		}
		OptionValueBuf {
			value_type: ValueType::INT,
			bytes,
		}
	}

	pub fn from_fixed(value: Fixed) -> OptionValueBuf {
		OptionValueBuf {
			value_type: ValueType::FIXED,
			bytes: Vec::from(value.as_word().as_u32().to_be_bytes()),
		}
	}

	pub fn from_fixed_list(values: &[Fixed]) -> OptionValueBuf {
		let mut bytes = Vec::with_capacity(4 * values.len());
		for value in values {
			let value = value.as_word().as_u32();
			bytes.extend_from_slice(&value.to_be_bytes());
		}
		OptionValueBuf {
			value_type: ValueType::FIXED,
			bytes,
		}
	}

	pub fn from_cstring(value: impl Into<CString>) -> OptionValueBuf {
		OptionValueBuf {
			value_type: ValueType::STRING,
			bytes: value.into().into_bytes_with_nul(),
		}
	}

	pub fn from_cstring_with_size(
		value: impl Into<CString>,
		size: usize,
	) -> OptionValueBuf {
		let mut bytes = value.into().into_bytes_with_nul();
		assert!(size >= bytes.len());
		if size > bytes.len() {
			bytes.resize(size, 0u8);
		}
		OptionValueBuf {
			value_type: ValueType::STRING,
			bytes,
		}
	}

	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		use io::Decode;
		use ValueType as T;

		let value_type = T::decode(r)?;
		match value_type {
			T::BOOL => {
				let value_size = Word::decode(r)?.as_u32();
				let value_count = Word::decode(r)?.as_u32();

				// FIXME: decode error instead of assert
				assert_eq!(value_size, 4);
				assert_eq!(value_count, 1);

				let value = Bool::decode(r)?;
				Ok(Self::from_bool(value == Bool::TRUE))
			},
			T::INT | T::FIXED => {
				let value_size = Word::decode(r)?.as_u32();
				let value_count = Word::decode(r)?.as_u32();

				// FIXME: decode error instead of assert
				assert_eq!(value_size, value_count * 4);

				let bytes = r.read_vec(value_size as usize)?;
				Ok(Self { value_type, bytes })
			},
			T::STRING => {
				let bytes_len = r.read_size()?;
				let bytes = r.read_vec(bytes_len)?;
				if bytes_len == 0 {
					return Ok(Self { value_type, bytes });
				}
				if bytes.iter().position(|&b| b == 0).is_none() {
					return Err(io::DecodeError {
						kind: io::DecodeErrorKind::InvalidString,
					});
				}
				Ok(Self { value_type, bytes })
			},
			T::BUTTON => {
				let value_size = Word::decode(r)?.as_u32();
				// FIXME: decode error instead of assert
				assert_eq!(value_size, 0);
				Ok(Self { value_type, bytes: Vec::new() })
			},
			_ => Err(io::DecodeError {
				kind: io::DecodeErrorKind::InvalidOptionType,
			}),
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<OptionValue<'_>> for OptionValueBuf {
	fn from(value: OptionValue) -> Self {
		Self {
			value_type: value.value_type,
			bytes: Vec::from(value.bytes),
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&OptionValue<'_>> for OptionValueBuf {
	fn from(value: &OptionValue) -> Self {
		Self::from(*value)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OptionValue<'_>> for OptionValueBuf {
	fn eq(&self, other: &OptionValue) -> bool {
		self.value_type == other.value_type && self.bytes == other.bytes
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OptionValueBuf> for OptionValue<'_> {
	fn eq(&self, other: &OptionValueBuf) -> bool {
		self.value_type == other.value_type && self.bytes == other.bytes
	}
}

// }}}
