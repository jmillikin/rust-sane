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

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Handle(pub u32);

impl fmt::Debug for Handle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Handle({})", self.0)
	}
}

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

// [5.2.2] SANE_NET_GET_DEVICES {{{

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

// [5.2.3] SANE_NET_OPEN {{{

// OpenRequest {{{

#[derive(Eq, PartialEq)]
pub struct OpenRequest {
	inner: OpenRequestInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct OpenRequestInner<'a> {
	device_name: &'a CStr,
}

impl fmt::Debug for OpenRequest {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenRequest")
	}
}

impl OpenRequest {
	pub fn device_name(&self) -> &CStr {
		self.inner.device_name
	}
}

impl<'a> OpenRequestInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("device_name", &self.device_name)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a OpenRequest {
		unsafe {
			let ptr: *const OpenRequestInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for OpenRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		ProcedureNumber::OPEN.encode(w)?;
		self.device_name().encode(w)
	}
}

// }}}

// OpenRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct OpenRequestBuf {
	inner: OpenRequestInner<'static>,
	device_name: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl OpenRequestBuf {
	pub fn new() -> OpenRequestBuf {
		OpenRequestBuf {
			inner: OpenRequestInner {
				device_name: util::CSTR_EMPTY,
			},
			device_name: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_device_name(&mut self, device_name: impl Into<CString>) {
		let device_name = device_name.into();
		self.inner.device_name = unsafe { util::cstr_to_static(&device_name) };
		self.device_name = Cow::Owned(device_name);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<OpenRequest> for OpenRequestBuf {
	fn as_ref(&self) -> &OpenRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for OpenRequestBuf {
	fn clone(&self) -> Self {
		OpenRequestBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for OpenRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenRequestBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for OpenRequestBuf {
	type Target = OpenRequest;
	fn deref(&self) -> &OpenRequest {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for OpenRequestBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for OpenRequestBuf {
	fn eq(&self, other: &OpenRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenRequest> for OpenRequestBuf {
	fn eq(&self, other: &OpenRequest) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenRequestBuf> for OpenRequest {
	fn eq(&self, other: &OpenRequestBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&OpenRequest> for OpenRequestBuf {
	fn from(request: &OpenRequest) -> Self {
		let mut buf = OpenRequestBuf::new();
		if !request.device_name().is_empty() {
			buf.set_device_name(request.device_name());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for OpenRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for OpenRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is OPEN
		let device_name = CString::decode(r)?;

		let mut buf = OpenRequestBuf::new();
		if !device_name.is_empty() {
			buf.set_device_name(device_name);
		}
		Ok(buf)
	}
}

// }}}

// OpenReply {{{

#[derive(Eq, PartialEq)]
pub struct OpenReply {
	inner: OpenReplyInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct OpenReplyInner<'a> {
	status: crate::Status,
	handle: Handle,
	resource: &'a CStr,
}

impl fmt::Debug for OpenReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenReply")
	}
}

impl OpenReply {
	pub fn status(&self) -> crate::Status {
		self.inner.status
	}

	pub fn handle(&self) -> Handle {
		self.inner.handle
	}

	pub fn resource(&self) -> &CStr {
		self.inner.resource
	}
}

impl<'a> OpenReplyInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("status", &self.status)
			.field("handle", &self.handle)
			.field("resource", &self.resource)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a OpenReply {
		unsafe {
			let ptr: *const OpenReplyInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for OpenReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		self.handle().encode(w)?;
		self.resource().encode(w)
	}
}

// }}}

// OpenReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct OpenReplyBuf {
	inner: OpenReplyInner<'static>,
	resource: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl OpenReplyBuf {
	pub fn new() -> OpenReplyBuf {
		OpenReplyBuf {
			inner: OpenReplyInner {
				status: crate::Status::GOOD,
				handle: Handle(0),
				resource: util::CSTR_EMPTY,
			},
			resource: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_status(&mut self, status: crate::Status) {
		self.inner.status = status;
	}

	pub fn set_handle(&mut self, handle: Handle) {
		self.inner.handle = handle;
	}

	pub fn set_resource(&mut self, resource: impl Into<CString>) {
		let resource = resource.into();
		self.inner.resource = unsafe { util::cstr_to_static(&resource) };
		self.resource = Cow::Owned(resource);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<OpenReply> for OpenReplyBuf {
	fn as_ref(&self) -> &OpenReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for OpenReplyBuf {
	fn clone(&self) -> Self {
		OpenReplyBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for OpenReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OpenReplyBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for OpenReplyBuf {
	type Target = OpenReply;
	fn deref(&self) -> &OpenReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for OpenReplyBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for OpenReplyBuf {
	fn eq(&self, other: &OpenReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenReply> for OpenReplyBuf {
	fn eq(&self, other: &OpenReply) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OpenReplyBuf> for OpenReply {
	fn eq(&self, other: &OpenReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&OpenReply> for OpenReplyBuf {
	fn from(reply: &OpenReply) -> Self {
		let mut buf = OpenReplyBuf::new();
		buf.set_status(reply.status());
		buf.set_handle(reply.handle());
		if !reply.resource().is_empty() {
			buf.set_resource(reply.resource());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for OpenReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for OpenReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let status = crate::Status::decode(r)?;
		let handle = Handle::decode(r)?;
		let resource = CString::decode(r)?;

		let mut buf = OpenReplyBuf::new();
		buf.set_status(status);
		buf.set_handle(handle);
		if !resource.is_empty() {
			buf.set_resource(resource);
		}
		Ok(buf)
	}
}

// }}}

// }}}

// [5.2.4] SANE_NET_CLOSE {{{

// CloseRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct CloseRequest {
	handle: Handle,
}

impl CloseRequest {
	pub fn handle(&self) -> Handle {
		self.handle
	}
}

impl io::Encode for CloseRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		ProcedureNumber::CLOSE.encode(w)?;
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
				handle: Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: Handle) {
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
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is CLOSE
		let handle = Handle::decode(r)?;

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
			inner: CloseReply { _p: () }
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

// }}}

// [5.2.5] SANE_NET_GET_OPTION_DESCRIPTORS {{{

// GetOptionDescriptorsRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct GetOptionDescriptorsRequest {
	handle: Handle,
}

impl GetOptionDescriptorsRequest {
	pub fn handle(&self) -> Handle {
		self.handle
	}
}

impl io::Encode for GetOptionDescriptorsRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		ProcedureNumber::GET_OPTION_DESCRIPTORS.encode(w)?;
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
				handle: Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: Handle) {
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
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is GET_OPTION_DESCRIPTORS
		let handle = Handle::decode(r)?;

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

// }}}

// [5.2.7] SANE_NET_GET_PARAMETERS {{{

// GetParametersRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct GetParametersRequest {
	handle: Handle,
}

impl GetParametersRequest {
	pub fn handle(&self) -> Handle {
		self.handle
	}
}

impl io::Encode for GetParametersRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		ProcedureNumber::GET_PARAMETERS.encode(w)?;
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
				handle: Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: Handle) {
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
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is GET_PARAMETERS
		let handle = Handle::decode(r)?;

		Ok(GetParametersRequestBuf {
			inner: GetParametersRequest { handle },
		})
	}
}

// }}}

// GetParametersReply {{{

#[derive(Debug, Eq, PartialEq)]
pub struct GetParametersReply {
	status: crate::Status,
	parameters: crate::Parameters,
}

impl GetParametersReply {
	pub fn status(&self) -> crate::Status {
		self.status
	}

	pub fn parameters(&self) -> &crate::Parameters {
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
				status: crate::Status::GOOD,
				parameters: crate::Parameters::new(),
			},
		}
	}

	pub fn set_status(&mut self, status: crate::Status) {
		self.inner.status = status;
	}

	pub fn set_parameters(&mut self, parameters: impl Into<crate::Parameters>) {
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
		let status = crate::Status::decode(r)?;
		let parameters = crate::Parameters::decode(r)?;

		Ok(GetParametersReplyBuf {
			inner: GetParametersReply { status, parameters },
		})
	}
}

// }}}

// }}}

// [5.2.8] SANE_NET_START {{{

// StartRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct StartRequest {
	handle: Handle,
}

impl StartRequest {
	pub fn handle(&self) -> Handle {
		self.handle
	}
}

impl io::Encode for StartRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		ProcedureNumber::START.encode(w)?;
		self.handle.encode(w)
	}
}

// }}}

// StartRequestBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Eq, PartialEq)]
pub struct StartRequestBuf {
	inner: StartRequest,
}

#[cfg(any(doc, feature = "alloc"))]
impl StartRequestBuf {
	pub fn new() -> StartRequestBuf {
		StartRequestBuf {
			inner: StartRequest {
				handle: Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: Handle) {
		self.inner.handle = handle
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<StartRequest> for StartRequestBuf {
	fn as_ref(&self) -> &StartRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for StartRequestBuf {
	fn clone(&self) -> Self {
		StartRequestBuf {
			inner: StartRequest {
				handle: self.inner.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for StartRequestBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("StartRequestBuf")
			.field("handle", &self.inner.handle)
			.finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for StartRequestBuf {
	type Target = StartRequest;
	fn deref(&self) -> &StartRequest {
		&self.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartRequest> for StartRequestBuf {
	fn eq(&self, other: &StartRequest) -> bool {
		self.inner == *other
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartRequestBuf> for StartRequest {
	fn eq(&self, other: &StartRequestBuf) -> bool {
		*self == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&StartRequest> for StartRequestBuf {
	fn from(request: &StartRequest) -> Self {
		StartRequestBuf {
			inner: StartRequest {
				handle: request.handle,
			},
		}
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for StartRequestBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for StartRequestBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is START
		let handle = Handle::decode(r)?;

		Ok(StartRequestBuf {
			inner: StartRequest { handle },
		})
	}
}

// }}}

// StartReply {{{

#[derive(Eq, PartialEq)]
pub struct StartReply {
	inner: StartReplyInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct StartReplyInner<'a> {
	status: crate::Status,
	port: u16,
	byte_order: ByteOrder,
	resource: &'a CStr,
}

impl fmt::Debug for StartReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "StartReply")
	}
}

impl StartReply {
	pub fn status(&self) -> crate::Status {
		self.inner.status
	}

	pub fn port(&self) -> u16 {
		self.inner.port
	}

	pub fn byte_order(&self) -> ByteOrder {
		self.inner.byte_order
	}

	pub fn resource(&self) -> &CStr {
		self.inner.resource
	}
}

impl<'a> StartReplyInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("status", &self.status)
			.field("port", &self.port)
			.field("byte_order", &self.byte_order)
			.field("resource", &self.resource)
			.finish()
	}

	#[cfg(any(doc, feature = "alloc"))]
	fn as_ref(&self) -> &'a StartReply {
		unsafe {
			let ptr: *const StartReplyInner = self;
			&*(ptr.cast())
		}
	}
}

impl io::Encode for StartReply {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.status().encode(w)?;
		Word::new(u32::from(self.port())).encode(w)?;
		self.byte_order().encode(w)?;
		self.resource().encode(w)
	}
}

// }}}

// StartReplyBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct StartReplyBuf {
	inner: StartReplyInner<'static>,
	resource: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl StartReplyBuf {
	pub fn new() -> StartReplyBuf {
		StartReplyBuf {
			inner: StartReplyInner {
				status: crate::Status::GOOD,
				port: 0,
				byte_order: ByteOrder::LITTLE_ENDIAN,
				resource: util::CSTR_EMPTY,
			},
			resource: Cow::Borrowed(util::CSTR_EMPTY),
		}
	}

	pub fn set_status(&mut self, status: crate::Status) {
		self.inner.status = status;
	}

	pub fn set_port(&mut self, port: u16) {
		self.inner.port = port;
	}

	pub fn set_byte_order(&mut self, byte_order: ByteOrder) {
		self.inner.byte_order = byte_order;
	}

	pub fn set_resource(&mut self, resource: impl Into<CString>) {
		let resource = resource.into();
		self.inner.resource = unsafe { util::cstr_to_static(&resource) };
		self.resource = Cow::Owned(resource);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<StartReply> for StartReplyBuf {
	fn as_ref(&self) -> &StartReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for StartReplyBuf {
	fn clone(&self) -> Self {
		StartReplyBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for StartReplyBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "StartReplyBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for StartReplyBuf {
	type Target = StartReply;
	fn deref(&self) -> &StartReply {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for StartReplyBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for StartReplyBuf {
	fn eq(&self, other: &StartReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartReply> for StartReplyBuf {
	fn eq(&self, other: &StartReply) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<StartReplyBuf> for StartReply {
	fn eq(&self, other: &StartReplyBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&StartReply> for StartReplyBuf {
	fn from(reply: &StartReply) -> Self {
		let mut buf = StartReplyBuf::new();
		buf.set_status(reply.status());
		buf.set_port(reply.port());
		buf.set_byte_order(reply.byte_order());
		if !reply.resource().is_empty() {
			buf.set_resource(reply.resource());
		}
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Encode for StartReplyBuf {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		self.as_ref().encode(w)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl io::Decode for StartReplyBuf {
	fn decode<R: io::Read>(
		r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
		let status = crate::Status::decode(r)?;
		let port = Word::decode(r)?.as_u32();
		let byte_order = ByteOrder::decode(r)?;
		let resource = CString::decode(r)?;

		// FIXME: error if port > u16::MAX

		let mut buf = StartReplyBuf::new();
		buf.set_status(status);
		buf.set_port(port as u16);
		buf.set_byte_order(byte_order);
		if !resource.is_empty() {
			buf.set_resource(resource);
		}
		Ok(buf)
	}
}

// }}}

// }}}

// [5.2.9] SANE_NET_CANCEL {{{

// CancelRequest {{{

#[derive(Debug, Eq, PartialEq)]
pub struct CancelRequest {
	handle: Handle,
}

impl CancelRequest {
	pub fn handle(&self) -> Handle {
		self.handle
	}
}

impl io::Encode for CancelRequest {
	fn encode<W: io::Write>(
		&self,
		w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		ProcedureNumber::CANCEL.encode(w)?;
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
				handle: Handle(0),
			},
		}
	}

	pub fn set_handle(&mut self, handle: Handle) {
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
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is CANCEL
		let handle = Handle::decode(r)?;

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
			inner: CancelReply { _p: () }
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

// }}}

// [5.2.10] SANE_NET_AUTHORIZE {{{

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
		ProcedureNumber::AUTHORIZE.encode(w)?;
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
		let _proc_no = ProcedureNumber::decode(r)?;
		// FIXME: check procedure number is AUTHORIZE
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
			inner: AuthorizeReply { _p: () }
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
