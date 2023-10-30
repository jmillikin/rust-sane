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

use core::fmt;

#[allow(unused_imports)]
use crate::{Bool, Status, Word};
use crate::net::io;
use crate::util;

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
		_w: &mut io::Writer<W>,
	) -> Result<(), io::EncodeError<W::Error>> {
		Ok(())
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
		_r: &mut io::Reader<R>,
	) -> Result<Self, io::DecodeError<R::Error>> {
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
	status: Status,
	devices: &'a [&'a util::Device],
}

impl fmt::Debug for GetDevicesReply {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "GetDevicesReply")
	}
}

impl GetDevicesReply {
	pub fn status(&self) -> Status {
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
		Word::new((self.devices().len() + 1) as u32).encode(w)?;
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
				status: Status::GOOD,
				devices: &[],
			},
			devices: Vec::new(),
			device_refs: Vec::new(),
		}
	}

	pub fn set_status(&mut self, status: Status) {
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
		let status = Status::decode(r)?;
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
