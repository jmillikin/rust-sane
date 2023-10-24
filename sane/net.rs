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
	borrow::Cow,
	ffi::CString,
};

use core::{
	ffi::CStr,
	fmt,
};

use crate::{
	Bool,
	ConstraintType,
	Int,
	Word,
};
use crate::util;

pub mod io;

pub const VERSION_CODE: u32 = 0x01010003;

// ByteOrder {{{

/// `SANE_Net_Byte_Order`
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ByteOrder(u32);

impl ByteOrder {
	/// `SANE_NET_LITTLE_ENDIAN`
	pub const LITTLE_ENDIAN: ByteOrder = ByteOrder(0x1234);

	/// `SANE_NET_BIG_ENDIAN`
	pub const BIG_ENDIAN: ByteOrder = ByteOrder(0x4321);

	pub const fn from_word(word: Word) -> ByteOrder {
		ByteOrder(word.as_u32())
	}

	pub const fn as_word(self) -> Word {
		Word::new(self.0)
	}
}

impl fmt::Debug for ByteOrder {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::BIG_ENDIAN => f.write_str("SANE_NET_BIG_ENDIAN"),
			Self::LITTLE_ENDIAN => f.write_str("SANE_NET_LITTLE_ENDIAN"),
			_ => write!(f, "SANE_Net_Byte_Order({:#X})", self.0),
		}
	}
}

// }}}

// ProcedureNumber {{{

/// `SANE_Net_Procedure_Number`
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ProcedureNumber(u32);

const PROCEDURE_NUMBER_STR: [&str; 11] = [
	/* 0 */ "SANE_NET_INIT",
	/* 1 */ "SANE_NET_GET_DEVICES",
	/* 2 */ "SANE_NET_OPEN",
	/* 3 */ "SANE_NET_CLOSE",
	/* 4 */ "SANE_NET_GET_OPTION_DESCRIPTORS",
	/* 5 */ "SANE_NET_CONTROL_OPTION",
	/* 6 */ "SANE_NET_GET_PARAMETERS",
	/* 7 */ "SANE_NET_START",
	/* 8 */ "SANE_NET_CANCEL",
	/* 9 */ "SANE_NET_AUTHORIZE",
	/* 10 */ "SANE_NET_EXIT",
];

impl ProcedureNumber {
	/// `SANE_NET_INIT`
	pub const INIT: ProcedureNumber = ProcedureNumber(0);

	/// `SANE_NET_GET_DEVICES`
	pub const GET_DEVICES: ProcedureNumber = ProcedureNumber(1);

	/// `SANE_NET_OPEN`
	pub const OPEN: ProcedureNumber = ProcedureNumber(2);

	/// `SANE_NET_CLOSE`
	pub const CLOSE: ProcedureNumber = ProcedureNumber(3);

	/// `SANE_NET_GET_OPTION_DESCRIPTORS`
	pub const GET_OPTION_DESCRIPTORS: ProcedureNumber = ProcedureNumber(4);

	/// `SANE_NET_CONTROL_OPTION`
	pub const CONTROL_OPTION: ProcedureNumber = ProcedureNumber(5);

	/// `SANE_NET_GET_PARAMETERS`
	pub const GET_PARAMETERS: ProcedureNumber = ProcedureNumber(6);

	/// `SANE_NET_START`
	pub const START: ProcedureNumber = ProcedureNumber(7);

	/// `SANE_NET_CANCEL`
	pub const CANCEL: ProcedureNumber = ProcedureNumber(8);

	/// `SANE_NET_AUTHORIZE`
	pub const AUTHORIZE: ProcedureNumber = ProcedureNumber(9);

	/// `SANE_NET_EXIT`
	pub const EXIT: ProcedureNumber = ProcedureNumber(10);

	pub const fn from_word(word: Word) -> ProcedureNumber {
		ProcedureNumber(word.as_u32())
	}

	pub const fn as_word(self) -> Word {
		Word::new(self.0)
	}
}

impl fmt::Debug for ProcedureNumber {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match PROCEDURE_NUMBER_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			None => write!(f, "SANE_Net_Procedure_Number({:#X})", self.0),
		}
	}
}

// }}}

// Handle {{{

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Handle(pub u32);

impl io::Decode for Handle {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		Ok(Handle(Word::decode(r)?.as_u32()))
	}
}

impl io::Encode for Handle {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		Word::new(self.0).encode(w)
	}
}

// }}}

// [5.2.1] SANE_NET_INIT {{{

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
		ProcedureNumber::INIT.encode(w)?;
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
				version_code: VERSION_CODE,
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
		let _proc_no = ProcedureNumber::decode(r)?;
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
	status: crate::Status,
	version_code: u32,
}

impl InitReply {
	pub fn status(&self) -> crate::Status {
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
				status: crate::Status::GOOD,
				version_code: VERSION_CODE,
			},
		}
	}

	pub fn set_status(&mut self, status: crate::Status) {
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
		let status = crate::Status::decode(r)?;
		let version_code = Word::decode(r)?.as_u32();

		Ok(InitReplyBuf {
			inner: InitReply { status, version_code },
		})
	}
}

// }}}

// }}}

// 5.2.2 SANE_NET_GET_DEVICES {{{

// GetDevicesRequest {{{

#[derive(Eq, PartialEq)]
pub struct GetDevicesRequest {
	_p: (),
}

impl fmt::Debug for GetDevicesRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GetDevicesRequest").finish()
	}
}

impl io::Encode for GetDevicesRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		ProcedureNumber::GET_DEVICES.encode(w)
	}
}

// }}}

// GetDevicesRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct GetDevicesRequestBuf {
	inner: GetDevicesRequest,
}

#[cfg(any(doc, feature = "alloc"))]
impl GetDevicesRequestBuf {
	pub fn new() -> GetDevicesRequestBuf {
		GetDevicesRequestBuf {
			inner: GetDevicesRequest { _p: () },
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<GetDevicesRequest> for GetDevicesRequestBuf {
	fn as_ref(&self) -> &GetDevicesRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for GetDevicesRequestBuf {
	fn clone(&self) -> Self {
		GetDevicesRequestBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for GetDevicesRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("GetDevicesRequestBuf").finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for GetDevicesRequestBuf {
	type Target = GetDevicesRequest;
	fn deref(&self) -> &GetDevicesRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetDevicesRequest> for GetDevicesRequestBuf {
	fn eq(&self, _other: &GetDevicesRequest) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetDevicesRequestBuf> for GetDevicesRequest {
	fn eq(&self, _other: &GetDevicesRequestBuf) -> bool {
		true
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&GetDevicesRequest> for GetDevicesRequestBuf {
	fn from(_request: &GetDevicesRequest) -> Self {
		GetDevicesRequestBuf::new()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for GetDevicesRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for GetDevicesRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is GET_DEVICES
		Ok(GetDevicesRequestBuf::new())
	}
}

// }}}

// GetDevicesReply {{{

#[derive(Eq, PartialEq)]
pub struct GetDevicesReply {
	inner: GetDevicesReplyInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct GetDevicesReplyInner<'a> {
	status: crate::Status,
	devices: &'a [&'a util::Device],
}

impl fmt::Debug for GetDevicesReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "GetDevicesReply")
	}
}

impl GetDevicesReply {
	pub fn status(&self) -> crate::Status {
		self.inner.status
	}

	pub fn devices(&self) -> &[&util::Device] {
		self.inner.devices
	}
}

impl<'a> GetDevicesReplyInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("status", &self.status)
			.field("devices", &self.devices)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a GetDevicesReply {
		unsafe {
			let ptr: *const GetDevicesReplyInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for GetDevicesReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		Word::new(self.devices().len() as u32).encode(w)?;
		for device in self.devices() {
			w.write_ptr(Some(*device))?;
		}
		w.write_ptr(Option::<&util::Device>::None)
	}
}

// }}}

// GetDevicesReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct GetDevicesReplyBuf {
	inner: GetDevicesReplyInner<'static>,
	devices: Vec<util::DeviceBuf>,
	device_refs: Vec<&'static util::Device>,
}

#[cfg(any(doc, feature = "alloc"))]
impl GetDevicesReplyBuf {
	pub fn new() -> GetDevicesReplyBuf {
		GetDevicesReplyBuf {
			inner: GetDevicesReplyInner {
				status: crate::Status::GOOD,
				devices: &[],
			},
			devices: Vec::new(),
			device_refs: Vec::new(),
		}
	}

	pub fn set_status(&mut self, status: crate::Status) {
		self.inner.status = status;
	}

	pub fn set_devices(&mut self, devices: impl Into<Vec<util::DeviceBuf>>) {
		let devices = devices.into();
		self.devices = devices;
		self.device_refs.truncate(0);
		for device in self.devices.iter() {
			self.device_refs.push(unsafe {
				core::mem::transmute(device.as_ref())
			});
		}
		self.inner.devices = unsafe {
			core::mem::transmute(self.device_refs.as_slice())
		};
	}

	pub fn into_devices(self) -> Vec<util::DeviceBuf> {
		self.devices
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<GetDevicesReply> for GetDevicesReplyBuf {
	fn as_ref(&self) -> &GetDevicesReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for GetDevicesReplyBuf {
	fn clone(&self) -> Self {
		GetDevicesReplyBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for GetDevicesReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "GetDevicesReplyBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for GetDevicesReplyBuf {
	type Target = GetDevicesReply;
	fn deref(&self) -> &GetDevicesReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for GetDevicesReplyBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for GetDevicesReplyBuf {
	fn eq(&self, other: &GetDevicesReplyBuf) -> bool {
		self.devices() == other.devices()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetDevicesReply> for GetDevicesReplyBuf {
	fn eq(&self, other: &GetDevicesReply) -> bool {
		self.devices() == other.devices()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<GetDevicesReplyBuf> for GetDevicesReply {
	fn eq(&self, other: &GetDevicesReplyBuf) -> bool {
		self.devices() == other.devices()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&GetDevicesReply> for GetDevicesReplyBuf {
	fn from(reply: &GetDevicesReply) -> Self {
		let devices: Vec<util::DeviceBuf> = reply.devices()
			.iter()
			.map(|&d| d.into())
			.collect();
		let mut buf = GetDevicesReplyBuf::new();
		buf.set_status(reply.status());
		buf.set_devices(devices);
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for GetDevicesReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for GetDevicesReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let status = crate::Status::decode(r)?;
		let devices_len = r.read_size()?;
		let mut devices = Vec::with_capacity(devices_len);
		for _ii in 0..devices_len {
			let is_null = Bool::decode(r)?;
			if is_null == Bool::TRUE {
				break;
			}

			// FIXME: verify NUL termination: there should only be a single
			//   NULL device pointer, and it should be at the end of the list
			//   (ii == devices_len-1)

			devices.push(util::DeviceBuf::decode(r)?);
		}

		let mut buf = GetDevicesReplyBuf::new();
		buf.set_status(status);
		buf.set_devices(devices);
		Ok(buf)
	}
}

// }}}

// }}}

impl io::Decode for crate::Range {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let mut range = crate::Range::new();
		range.min = Word::decode(r)?;
		range.max = Word::decode(r)?;
		range.quant = Word::decode(r)?;
		Ok(range)
	}
}

impl io::Encode for crate::Range {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.min.encode(w)?;
		self.max.encode(w)?;
		self.quant.encode(w)
	}
}

impl io::Decode for crate::Parameters {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let mut params = crate::Parameters::new();
		params.format = crate::Frame::decode(r)?;
		params.last_frame = Bool::decode(r)?;
		params.bytes_per_line = Int::decode(r)?;
		params.pixels_per_line = Int::decode(r)?;
		params.lines = Int::decode(r)?;
		params.depth = Int::decode(r)?;
		Ok(params)
	}
}

impl io::Encode for crate::Parameters {
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

impl io::Encode for util::Device {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.name().encode(w)?;
		self.vendor().encode(w)?;
		self.model().encode(w)?;
		self.kind().encode(w)
	}
}

impl io::Encode for util::DeviceRef<'_> {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for util::DeviceBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for util::DeviceBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let dev_name = CString::decode(r)?;
		let mut dev = util::DeviceBuf::new(dev_name);
		dev.set_vendor(CString::decode(r)?);
		dev.set_model(CString::decode(r)?);
		dev.set_kind(CString::decode(r)?);
		Ok(dev)
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
		let opt_type = crate::ValueType::decode(r)?;
		let unit = crate::Unit::decode(r)?;
		let size = r.read_size()?;
		let cap = util::Capabilities::from_word(Word::decode(r)?);

		match opt_type {
			crate::ValueType::BOOL => {
				expect_constraint_none(r)?;
				assert_eq!(size, 4); // FIXME: return error, not assert
				Ok(util::BoolOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.capabilities(cap)
					.build())
			},
			crate::ValueType::INT => {
				let builder = util::IntOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.unit(unit)
					.capabilities(cap);
				read_int_option(r, builder, size)
			},
			crate::ValueType::FIXED => {
				let builder = util::FixedOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.unit(unit)
					.capabilities(cap);
				read_fixed_option(r, builder, size)
			},
			crate::ValueType::STRING => {
				let builder = util::StringOptionBuilder::new(name, size)
					.title(title)
					.description(desc)
					.unit(unit)
					.capabilities(cap);
				read_string_option(r, builder)
			},
			crate::ValueType::BUTTON => {
				expect_constraint_none(r)?;
				// check size == 0 ?
				Ok(util::ButtonOptionBuilder::new(name)
					.title(title)
					.description(desc)
					.capabilities(cap)
					.build())
			},
			crate::ValueType::GROUP => {
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
			crate::ValueType::BOOL,
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
			let range: crate::Range = range;
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
					crate::ValueType::INT,
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
			let range: crate::Range = range;
			builder = builder.range(
				crate::Fixed::from_word(range.min),
				crate::Fixed::from_word(range.max),
				crate::Fixed::from_word(range.quant),
			);
		},
		ConstraintType::WORD_LIST => {
			builder = builder.values(read_fixed_list(r)?);
		},
		_ => {
			return Err(io::DecodeError {
				kind: io::DecodeErrorKind::InvalidConstraint(
					crate::ValueType::FIXED,
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
					crate::ValueType::STRING,
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
) -> Result<Vec<crate::Fixed>, io::DecodeError<R::Error>> {
	use io::Decode;
	let (len, mut vec) = new_vec_for_array(r)?;
	for ii in 0..len {
		let word = Word::decode(r)?;
		// FIXME: validate that the first item in the word list
		// is the expected length?
		if ii > 0 {
			vec.push(crate::Fixed::from_word(word));
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
