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

#![allow(unused_imports)]

#[cfg(any(doc, feature = "alloc"))]
use alloc::{
	borrow::Cow,
	boxed::Box,
	ffi::CString,
	vec::Vec,
};

use core::{
	ffi::CStr,
	fmt,
	mem::size_of,
	ptr,
};

const fn cstr(bytes: &[u8]) -> &CStr {
	unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
}

pub(crate) const CSTR_EMPTY: &CStr = cstr(b"\x00");

unsafe fn ref_array_next<T>(t: &T) -> &T {
	&*((t as *const T).add(1))
}

#[cfg(any(doc, feature = "alloc"))]
pub(crate) unsafe fn cstr_to_static(cstr: &CStr) -> &'static CStr {
	core::mem::transmute(cstr)
}

pub(crate) fn split_array_ref<T, const N: usize>(
	slice: &[T],
) -> (&[T; N], &[T]) {
	let (head, tail) = slice.split_at(N);
	(unsafe { &*(head.as_ptr() as *const [T; N]) }, tail)
}

fn iter_eq<X, Y>(
	x_iter: impl IntoIterator<Item = X>,
	y_iter: impl IntoIterator<Item = Y>,
) -> bool
where
	X: PartialEq<Y>,
{
	let mut x = x_iter.into_iter();
	let mut y = y_iter.into_iter();
	loop {
		let x_next = x.next();
		let y_next = y.next();
		match (x_next, y_next) {
			(None, None) => return true,
			(Some(x), Some(y)) => {
				if x != y {
					return false;
				}
			},
			_ => return false,
		}
	}
}

// Device {{{

#[derive(Eq, PartialEq)]
pub struct Device {
	inner: DeviceInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct DeviceInner<'a> {
	name: &'a CStr,
	vendor: &'a CStr,
	model: &'a CStr,
	kind: &'a CStr,
}

impl fmt::Debug for Device {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.inner.fmt(f, "Device")
	}
}

impl Device {
	pub fn name(&self) -> &CStr {
		self.inner.name
	}

	pub fn vendor(&self) -> &CStr {
		self.inner.vendor
	}

	pub fn model(&self) -> &CStr {
		self.inner.model
	}

	pub fn kind(&self) -> &CStr {
		self.inner.kind
	}
}

impl<'a> DeviceInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("name", &self.name)
			.field("vendor", &self.vendor)
			.field("model", &self.model)
			.field("kind", &self.kind)
			.finish()
	}

	fn as_ref(&self) -> &'a Device {
		unsafe {
			let ptr: *const DeviceInner = self;
			&*(ptr.cast())
		}
	}
}

// }}}

// DeviceRef {{{

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct DeviceRef<'a> {
	inner: DeviceInner<'a>,
}

impl<'a> DeviceRef<'a> {
	pub unsafe fn from_ptr(ptr: *const crate::Device) -> DeviceRef<'a> {
		let raw = &*ptr;
		let inner = DeviceInner {
			name: raw.name.to_c_str().unwrap_or(CSTR_EMPTY),
			vendor: raw.vendor.to_c_str().unwrap_or(CSTR_EMPTY),
			model: raw.model.to_c_str().unwrap_or(CSTR_EMPTY),
			kind: raw.r#type.to_c_str().unwrap_or(CSTR_EMPTY),
		};
		DeviceRef { inner }
	}
}

impl<'a> AsRef<Device> for DeviceRef<'a> {
	fn as_ref(&self) -> &Device {
		self.inner.as_ref()
	}
}

impl fmt::Debug for DeviceRef<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.inner.fmt(f, "DeviceRef")
	}
}

impl<'a> core::ops::Deref for DeviceRef<'a> {
	type Target = Device;
	fn deref(&self) -> &Device {
		self.inner.as_ref()
	}
}

impl PartialEq<Device> for DeviceRef<'_> {
	fn eq(&self, other: &Device) -> bool {
		self.inner == other.inner
	}
}

impl PartialEq<DeviceRef<'_>> for Device {
	fn eq(&self, other: &DeviceRef) -> bool {
		self.inner == other.inner
	}
}

// }}}

// DeviceBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct DeviceBuf {
	inner: DeviceInner<'static>,
	name: CString,
	vendor: Cow<'static, CStr>,
	model: Cow<'static, CStr>,
	kind: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl DeviceBuf {
	pub fn new(name: impl Into<CString>) -> DeviceBuf {
		let name = name.into();
		let inner = DeviceInner {
			name: unsafe { cstr_to_static(name.as_c_str()) },
			vendor: CSTR_EMPTY,
			model: CSTR_EMPTY,
			kind: CSTR_EMPTY,
		};
		DeviceBuf {
			inner,
			name,
			vendor: Cow::Borrowed(CSTR_EMPTY),
			model: Cow::Borrowed(CSTR_EMPTY),
			kind: Cow::Borrowed(CSTR_EMPTY),
		}
	}

	pub fn set_name(&mut self, name: impl Into<CString>) {
		self.name = name.into();
		self.inner.name = unsafe { cstr_to_static(self.name.as_c_str()) };
	}

	pub fn set_vendor(&mut self, vendor: impl Into<CString>) {
		let vendor = vendor.into();
		self.inner.vendor = unsafe { cstr_to_static(vendor.as_c_str()) };
		self.vendor = Cow::Owned(vendor);
	}

	pub fn set_model(&mut self, model: impl Into<CString>) {
		let model = model.into();
		self.inner.model = unsafe { cstr_to_static(model.as_c_str()) };
		self.model = Cow::Owned(model);
	}

	pub fn set_kind(&mut self, kind: impl Into<CString>) {
		let kind = kind.into();
		self.inner.kind = unsafe { cstr_to_static(kind.as_c_str()) };
		self.kind = Cow::Owned(kind);
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<Device> for DeviceBuf {
	fn as_ref(&self) -> &Device {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for DeviceBuf {
	fn clone(&self) -> Self {
		DeviceBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for DeviceBuf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.inner.fmt(f, "DeviceBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for DeviceBuf {
	type Target = Device;
	fn deref(&self) -> &Device {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for DeviceBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for DeviceBuf {
	fn eq(&self, other: &DeviceBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<Device> for DeviceBuf {
	fn eq(&self, other: &Device) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<DeviceBuf> for Device {
	fn eq(&self, other: &DeviceBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<DeviceRef<'_>> for DeviceBuf {
	fn eq(&self, other: &DeviceRef) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<DeviceBuf> for DeviceRef<'_> {
	fn eq(&self, other: &DeviceBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&Device> for DeviceBuf {
	fn from(dev: &Device) -> DeviceBuf {
		let mut buf = DeviceBuf::new(dev.name());
		if !dev.vendor().is_empty() {
			buf.set_vendor(dev.vendor());
		};
		if !dev.model().is_empty() {
			buf.set_model(dev.model());
		};
		if !dev.kind().is_empty() {
			buf.set_kind(dev.kind());
		};
		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<DeviceRef<'_>> for DeviceBuf {
	fn from(dev: DeviceRef) -> DeviceBuf {
		Self::from(dev.inner.as_ref())
	}
}

// }}}

// DevicesRef {{{

#[derive(Clone, Copy)]
pub struct DevicesRef<'a> {
	devices: &'a *const crate::Device,
}

impl fmt::Debug for DevicesRef<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

impl<'a> DevicesRef<'a> {
	pub unsafe fn from_ptr(
		ptr: *const *const crate::Device,
	) -> DevicesRef<'a> {
		DevicesRef { devices: &*ptr }
	}

	pub fn iter(&self) -> DevicesIter<'a> {
		DevicesIter { devices: self.devices }
	}
}

impl Eq for DevicesRef<'_> {}

impl PartialEq for DevicesRef<'_> {
	fn eq(&self, other: &DevicesRef) -> bool {
		iter_eq(self, other)
	}
}

impl<'a> IntoIterator for &DevicesRef<'a> {
	type Item = DeviceRef<'a>;
	type IntoIter = DevicesIter<'a>;

	fn into_iter(self) -> DevicesIter<'a> {
		self.iter()
	}
}

// }}}

// DevicesIter {{{

pub struct DevicesIter<'a> {
	devices: &'a *const crate::Device,
}

impl<'a> Iterator for DevicesIter<'a> {
	type Item = DeviceRef<'a>;

	fn next(&mut self) -> Option<DeviceRef<'a>> {
		Some(unsafe {
			let device_ptr: *const _ = self.devices.as_ref()?;
			self.devices = ref_array_next(self.devices);
			DeviceRef::from_ptr(device_ptr)
		})
	}
}

// }}}

// DevicesBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct DevicesBuf {
	devices: Vec<Box<crate::Device>>,
	device_ptrs: Vec<*const crate::Device>,
	strings: Vec<CString>,
}

#[cfg(any(doc, feature = "alloc"))]
impl DevicesBuf {
	pub fn new() -> DevicesBuf {
		let mut device_ptrs = Vec::new();
		device_ptrs.push(ptr::null());
		DevicesBuf {
			devices: Vec::new(),
			device_ptrs,
			strings: Vec::new(),
		}
	}

	pub fn len(&self) -> usize {
		self.devices.len()
	}

	pub fn push(&mut self, dev: DeviceBuf) {
		let cstr_empty_ptr = crate::StringConst::from_c_str(CSTR_EMPTY);

		let mut take_cstr = |cow: Cow<CStr>| -> crate::StringConst {
			if let Cow::Owned(cstr) = cow {
				if !cstr.is_empty() {
					let ptr = crate::StringConst::from_c_str(&cstr);
					self.strings.push(cstr);
					return ptr;
				}
			}
			cstr_empty_ptr
		};

		let mut raw = crate::Device::new();
		raw.name = take_cstr(Cow::Owned(dev.name));
		raw.vendor = take_cstr(dev.vendor);
		raw.model = take_cstr(dev.model);
		raw.r#type = take_cstr(dev.kind);

		let boxed = Box::new(raw);
		let boxed_ptr: *const crate::Device = Box::as_ref(&boxed);
		self.devices.push(boxed);
		self.device_ptrs.pop();
		self.device_ptrs.push(boxed_ptr);
		self.device_ptrs.push(ptr::null());
	}

	pub fn as_ptr(&self) -> *const *const crate::Device {
		self.device_ptrs.as_ptr()
	}

	pub fn iter<'a>(&'a self) -> DevicesIter<'a> {
		DevicesIter { devices: &self.device_ptrs[0] }
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for DevicesBuf {
	fn clone(&self) -> Self {
		let mut cloned = DevicesBuf::new();
		for device in self.iter() {
			cloned.push(DeviceBuf::from(device));
		}
		cloned
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for DevicesBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for DevicesBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for DevicesBuf {
	fn eq(&self, other: &DevicesBuf) -> bool {
		iter_eq(self, other)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl<'a> IntoIterator for &'a DevicesBuf {
	type Item = DeviceRef<'a>;
	type IntoIter = DevicesIter<'a>;

	fn into_iter(self) -> DevicesIter<'a> {
		self.iter()
	}
}

// }}}

// OptionDescriptor {{{

#[derive(Eq, PartialEq)]
pub struct OptionDescriptor {
	inner: OptionDescriptorInner<'static>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct OptionDescriptorInner<'a> {
	name: &'a CStr,
	title: &'a CStr,
	description: &'a CStr,
	value_type: crate::ValueType,
	unit: crate::Unit,
	size: u32,
	capabilities: Capabilities,
	constraint: Constraint<'a>,
}

impl fmt::Debug for OptionDescriptor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OptionDescriptor")
	}
}

impl OptionDescriptor {
	pub fn name(&self) -> &CStr {
		self.inner.name
	}

	pub fn title(&self) -> &CStr {
		self.inner.title
	}

	pub fn description(&self) -> &CStr {
		self.inner.description
	}

	pub fn value_type(&self) -> crate::ValueType {
		self.inner.value_type
	}

	pub fn unit(&self) -> crate::Unit {
		self.inner.unit
	}

	pub fn size(&self) -> usize {
		self.inner.size as usize
	}

	pub fn capabilities(&self) -> Capabilities {
		self.inner.capabilities
	}

	pub fn constraint(&self) -> Constraint {
		self.inner.constraint
	}
}

impl<'a> OptionDescriptorInner<'a> {
	fn fmt(&self, f: &mut fmt::Formatter, struct_name: &str) -> fmt::Result {
		f.debug_struct(struct_name)
			.field("name", &self.name)
			.field("title", &self.title)
			.field("description", &self.description)
			.field("value_type", &self.value_type)
			.field("unit", &self.unit)
			.field("size", &self.size)
			.field("capabilities", &self.capabilities)
			.field("constraint", &self.constraint)
			.finish()
	}

	fn as_ref(&self) -> &'a OptionDescriptor {
		unsafe {
			let ptr: *const OptionDescriptorInner = self;
			&*(ptr.cast())
		}
	}
}

// }}}

// OptionDescriptorRef {{{

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct OptionDescriptorRef<'a> {
	inner: OptionDescriptorInner<'a>,
}

impl<'a> OptionDescriptorRef<'a> {
	pub unsafe fn from_ptr(
		ptr: *const crate::OptionDescriptor,
	) -> OptionDescriptorRef<'a> {
		let raw = &*ptr;
		let inner = OptionDescriptorInner {
			name: raw.name.to_c_str().unwrap_or(CSTR_EMPTY),
			title: raw.title.to_c_str().unwrap_or(CSTR_EMPTY),
			description: raw.desc.to_c_str().unwrap_or(CSTR_EMPTY),
			value_type: raw.r#type,
			unit: raw.unit,
			size: raw.size.as_word().as_u32(),
			capabilities: Capabilities {
				bits: raw.cap.as_word().as_u32(),
			},
			constraint: Constraint::from_ptr(
				raw.r#type,
				raw.constraint_type,
				raw.constraint,
			).unwrap_or(Constraint::None),
		};
		OptionDescriptorRef { inner }
	}
}

impl<'a> AsRef<OptionDescriptor> for OptionDescriptorRef<'a> {
	fn as_ref(&self) -> &OptionDescriptor {
		self.inner.as_ref()
	}
}

impl fmt::Debug for OptionDescriptorRef<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.inner.fmt(f, "OptionDescriptorRef")
	}
}

impl<'a> core::ops::Deref for OptionDescriptorRef<'a> {
	type Target = OptionDescriptor;
	fn deref(&self) -> &OptionDescriptor {
		self.inner.as_ref()
	}
}

impl PartialEq<OptionDescriptor> for OptionDescriptorRef<'_> {
	fn eq(&self, other: &OptionDescriptor) -> bool {
		self.inner == other.inner
	}
}

impl PartialEq<OptionDescriptorRef<'_>> for OptionDescriptor {
	fn eq(&self, other: &OptionDescriptorRef) -> bool {
		self.inner == other.inner
	}
}

// }}}

// OptionDescriptorBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct OptionDescriptorBuf {
	inner: OptionDescriptorInner<'static>,
	raw: Box<crate::OptionDescriptor>,
	strings: Vec<CString>,
	constraint_range: Option<Box<crate::Range>>,
	constraint_word_list: Vec<crate::Word>,
	constraint_string_list: Vec<crate::StringConst>,
}

#[cfg(any(doc, feature = "alloc"))]
impl OptionDescriptorBuf {
	fn new(
		name: Option<CString>,
		title: Option<CString>,
		desc: Option<CString>,
	) -> OptionDescriptorBuf {
		let mut strings = Vec::new();

		let mut take_cstr = |cstring: Option<CString>| -> &'static CStr {
			match cstring {
				None => CSTR_EMPTY,
				Some(cstring) if cstring.is_empty() => CSTR_EMPTY,
				Some(cstring) => {
					let cstr = unsafe { cstr_to_static(cstring.as_ref()) };
					strings.push(cstring);
					cstr
				},
			}
		};

		let mut raw = Box::new(crate::OptionDescriptor::new());

		let name_cstr = take_cstr(name);
		let title_cstr = take_cstr(title);
		let desc_cstr = take_cstr(desc);

		raw.name = crate::StringConst::from_c_str(name_cstr);
		raw.title = crate::StringConst::from_c_str(title_cstr);
		raw.desc = crate::StringConst::from_c_str(desc_cstr);

		let inner = OptionDescriptorInner {
			name: name_cstr,
			title: title_cstr,
			description: desc_cstr,
			value_type: raw.r#type,
			unit: raw.unit,
			size: raw.size.as_word().as_u32(),
			capabilities: Capabilities::from_word(raw.cap.as_word()),
			constraint: Constraint::None,
		};

		OptionDescriptorBuf {
			inner,
			raw,
			strings,
			constraint_range: None,
			constraint_word_list: Vec::new(),
			constraint_string_list: Vec::new(),
		}
	}

	fn set_unit(&mut self, unit: crate::Unit) {
		self.raw.unit = unit;
		self.inner.unit = unit;
	}

	fn set_value_type(&mut self, value_type: crate::ValueType) {
		self.raw.r#type = value_type;
		self.inner.value_type = value_type;
	}

	fn set_size(&mut self, size: usize) {
		self.raw.size = crate::Int::new(size as i32);
		self.inner.size = size as u32;
	}

	fn set_capabilities(&mut self, capabilities: Capabilities) {
		self.raw.cap = crate::Int::from_word(capabilities.as_word());
		self.inner.capabilities = capabilities;
	}

	fn set_constraint_range(&mut self, range: crate::Range) {
		let range_box = Box::new(range);
		let range_ptr: *const crate::Range = range_box.as_ref();
		self.constraint_range = Some(range_box);
		self.raw.constraint_type = crate::ConstraintType::RANGE;
		self.raw.constraint = range_ptr.cast();
		self.update_constraint();
	}

	fn set_constraint_word_list(&mut self, words: Vec<crate::Word>) {
		let words_ptr: *const crate::Word = words.as_ptr();
		self.constraint_word_list = words;
		self.raw.constraint_type = crate::ConstraintType::WORD_LIST;
		self.raw.constraint = words_ptr.cast();
		self.update_constraint();
	}

	fn set_constraint_string_list(&mut self, mut values: Vec<CString>) {
		let string_list = &mut self.constraint_string_list;
		for value in values.iter() {
			string_list.push(crate::StringConst::from_c_str(&value));
		}
		string_list.push(crate::StringConst::null());
		self.strings.append(&mut values);
		self.raw.constraint_type = crate::ConstraintType::STRING_LIST;
		self.raw.constraint = string_list.as_ptr().cast();
		self.update_constraint();
	}

	fn update_constraint(&mut self) {
		self.inner.constraint = unsafe {
			Constraint::from_ptr(
				self.raw.r#type,
				self.raw.constraint_type,
				self.raw.constraint,
			).unwrap_or(Constraint::None)
		};
	}

	pub fn as_ptr(&self) -> *const crate::OptionDescriptor {
		Box::as_ref(&self.raw)
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl AsRef<OptionDescriptor> for OptionDescriptorBuf {
	fn as_ref(&self) -> &OptionDescriptor {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Clone for OptionDescriptorBuf {
	fn clone(&self) -> Self {
		OptionDescriptorBuf::from(self.as_ref())
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl fmt::Debug for OptionDescriptorBuf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.inner.fmt(f, "OptionDescriptorBuf")
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl core::ops::Deref for OptionDescriptorBuf {
	type Target = OptionDescriptor;
	fn deref(&self) -> &OptionDescriptor {
		self.inner.as_ref()
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl Eq for OptionDescriptorBuf {}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq for OptionDescriptorBuf {
	fn eq(&self, other: &OptionDescriptorBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OptionDescriptor> for OptionDescriptorBuf {
	fn eq(&self, other: &OptionDescriptor) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OptionDescriptorBuf> for OptionDescriptor {
	fn eq(&self, other: &OptionDescriptorBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OptionDescriptorRef<'_>> for OptionDescriptorBuf {
	fn eq(&self, other: &OptionDescriptorRef) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl PartialEq<OptionDescriptorBuf> for OptionDescriptorRef<'_> {
	fn eq(&self, other: &OptionDescriptorBuf) -> bool {
		self.inner == other.inner
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<&OptionDescriptor> for OptionDescriptorBuf {
	fn from(dev: &OptionDescriptor) -> OptionDescriptorBuf {
		let mut name = None;
		let mut title = None;
		let mut desc = None;

		if !dev.name().is_empty() {
			name = Some(CString::from(dev.name()));
		}
		if !dev.title().is_empty() {
			title = Some(CString::from(dev.title()));
		}
		if !dev.description().is_empty() {
			desc = Some(CString::from(dev.description()));
		}

		let mut buf = OptionDescriptorBuf::new(name, title, desc);
		buf.set_value_type(dev.value_type());
		buf.set_size(dev.size());
		buf.set_capabilities(dev.capabilities());

		match dev.inner.constraint {
			Constraint::None => {},
			Constraint::IntRange(range) => {
				buf.set_constraint_range(*range);
			},
			Constraint::FixedRange(range) => {
				buf.set_constraint_range(*range);
			},
			Constraint::IntList(word_list) => {
				buf.set_constraint_word_list(word_list.iter().collect());
			},
			Constraint::FixedList(word_list) => {
				buf.set_constraint_word_list(word_list.iter().collect());
			},
			Constraint::StringList(string_list) => {
				let strings = string_list.iter().map(|s| s.into()).collect();
				buf.set_constraint_string_list(strings);
			},
		}

		buf
	}
}

#[cfg(any(doc, feature = "alloc"))]
impl From<OptionDescriptorRef<'_>> for OptionDescriptorBuf {
	fn from(dev: OptionDescriptorRef) -> OptionDescriptorBuf {
		Self::from(dev.inner.as_ref())
	}
}

// }}}

// BoolOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct BoolOptionBuilder {
	name: CString,
	title: Option<CString>,
	description: Option<CString>,
	capabilities: Capabilities,
}

#[cfg(any(doc, feature = "alloc"))]
impl BoolOptionBuilder {
	pub fn new(name: impl Into<CString>) -> Self {
		Self {
			name: name.into(),
			title: None,
			description: None,
			capabilities: Capabilities::NONE,
		}
	}

	pub fn title(mut self, title: impl Into<CString>) -> Self {
		self.title = Some(title.into());
		self
	}

	pub fn description(
		mut self,
		description: impl Into<CString>,
	) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn capabilities(mut self, capabilities: Capabilities) -> Self {
		self.capabilities = capabilities;
		self
	}

	pub fn build(self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			Some(self.name),
			self.title,
			self.description,
		);
		buf.set_value_type(crate::ValueType::BOOL);
		buf.set_size(size_of::<crate::Bool>());
		buf.set_capabilities(self.capabilities);
		buf
	}
}

// }}}

// IntOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct IntOptionBuilder {
	name: CString,
	title: Option<CString>,
	description: Option<CString>,
	unit: crate::Unit,
	capabilities: Capabilities,
	size: i32,
	range: Option<crate::Range>,
	word_list: Option<Vec<crate::Word>>,
}

#[cfg(any(doc, feature = "alloc"))]
impl IntOptionBuilder {
	pub fn new(name: impl Into<CString>) -> Self {
		Self {
			name: name.into(),
			title: None,
			description: None,
			unit: crate::Unit::NONE,
			capabilities: Capabilities::NONE,
			size: size_of::<crate::Int>() as i32,
			range: None,
			word_list: None,
		}
	}

	pub fn title(mut self, title: impl Into<CString>) -> Self {
		self.title = Some(title.into());
		self
	}

	pub fn description(
		mut self,
		description: impl Into<CString>,
	) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn unit(mut self, unit: crate::Unit) -> Self {
		self.unit = unit;
		self
	}

	pub fn capabilities(mut self, capabilities: Capabilities) -> Self {
		self.capabilities = capabilities;
		self
	}

	pub fn count(mut self, count: usize) -> Self {
		// FIXME: assert count > 0 ?
		// FIXME: assert count*sizeof(Int) <= i32::MAX ?
		self.size = (count * size_of::<crate::Int>()) as i32;
		self
	}

	pub fn range(mut self, min: i32, max: i32, quant: i32) -> Self {
		let mut range = crate::Range::new();
		range.min = crate::Int::new(min).as_word();
		range.max = crate::Int::new(max).as_word();
		range.quant = crate::Int::new(quant).as_word();
		self.range = Some(range);
		self.word_list = None;
		self
	}

	pub fn values(self, values: impl AsRef<[i32]>) -> Self {
		let values = values.as_ref();
		let mut word_list = Vec::with_capacity(values.len() + 1);
		word_list.push(crate::Word::new(
			values.len() as u32,
		));
		for value in values {
			word_list.push(crate::Int::new(*value).as_word());
		}
		unsafe { self.constraint_word_list(word_list) }
	}

	pub unsafe fn constraint_word_list(
		mut self,
		word_list: Vec<crate::Word>,
	) -> Self {
		self.word_list = Some(word_list);
		self.range = None;
		self
	}

	pub fn build(self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			Some(self.name),
			self.title,
			self.description,
		);
		buf.set_value_type(crate::ValueType::INT);
		buf.set_size(self.size as usize);
		buf.set_unit(self.unit);
		buf.set_capabilities(self.capabilities);

		if let Some(range) = self.range {
			buf.set_constraint_range(range);
		} else if let Some(word_list) = self.word_list {
			buf.set_constraint_word_list(word_list);
		}

		buf
	}
}

// }}}

// FixedOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct FixedOptionBuilder {
	name: CString,
	title: Option<CString>,
	description: Option<CString>,
	unit: crate::Unit,
	capabilities: Capabilities,
	size: i32,
	range: Option<crate::Range>,
	word_list: Option<Vec<crate::Word>>,
}

#[cfg(any(doc, feature = "alloc"))]
impl FixedOptionBuilder {
	pub fn new(name: impl Into<CString>) -> Self {
		Self {
			name: name.into(),
			title: None,
			description: None,
			unit: crate::Unit::NONE,
			capabilities: Capabilities::NONE,
			size: size_of::<crate::Fixed>() as i32,
			range: None,
			word_list: None,
		}
	}

	pub fn title(mut self, title: impl Into<CString>) -> Self {
		self.title = Some(title.into());
		self
	}

	pub fn description(
		mut self,
		description: impl Into<CString>,
	) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn unit(mut self, unit: crate::Unit) -> Self {
		self.unit = unit;
		self
	}

	pub fn capabilities(mut self, capabilities: Capabilities) -> Self {
		self.capabilities = capabilities;
		self
	}

	pub fn count(mut self, count: usize) -> Self {
		// FIXME: assert count > 0 ?
		// FIXME: assert count*sizeof(Int) <= i32::MAX ?
		self.size = (count * size_of::<crate::Fixed>()) as i32;
		self
	}

	pub fn range(
		mut self,
		min: crate::Fixed,
		max: crate::Fixed,
		quant: crate::Fixed,
	) -> Self {
		let mut range = crate::Range::new();
		range.min = min.as_word();
		range.max = max.as_word();
		range.quant = quant.as_word();
		self.range = Some(range);
		self.word_list = None;
		self
	}

	pub fn values(self, values: impl AsRef<[crate::Fixed]>) -> Self {
		let values = values.as_ref();
		let mut word_list = Vec::with_capacity(values.len() + 1);
		word_list.push(crate::Word::new(
			values.len() as u32,
		));
		for value in values {
			word_list.push(value.as_word());
		}
		unsafe { self.constraint_word_list(word_list) }
	}

	pub unsafe fn constraint_word_list(
		mut self,
		word_list: Vec<crate::Word>,
	) -> Self {
		self.word_list = Some(word_list);
		self.range = None;
		self
	}

	pub fn build(self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			Some(self.name),
			self.title,
			self.description,
		);
		buf.set_value_type(crate::ValueType::FIXED);
		buf.set_size(self.size as usize);
		buf.set_unit(self.unit);
		buf.set_capabilities(self.capabilities);

		if let Some(range) = self.range {
			buf.set_constraint_range(range);
		} else if let Some(word_list) = self.word_list {
			buf.set_constraint_word_list(word_list);
		}

		buf
	}
}

// }}}

// StringOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct StringOptionBuilder {
	name: CString,
	title: Option<CString>,
	description: Option<CString>,
	unit: crate::Unit,
	capabilities: Capabilities,
	size: i32,
	values: Option<Vec<CString>>,
}

#[cfg(any(doc, feature = "alloc"))]
impl StringOptionBuilder {
	pub fn new(name: impl Into<CString>, size: usize) -> Self {
		// FIXME: assert size <= i32::MAX
		Self {
			name: name.into(),
			title: None,
			description: None,
			unit: crate::Unit::NONE,
			capabilities: Capabilities::NONE,
			size: size as i32,
			values: None,
		}
	}

	pub fn title(mut self, title: impl Into<CString>) -> Self {
		self.title = Some(title.into());
		self
	}

	pub fn description(
		mut self,
		description: impl Into<CString>,
	) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn unit(mut self, unit: crate::Unit) -> Self {
		self.unit = unit;
		self
	}

	pub fn capabilities(mut self, capabilities: Capabilities) -> Self {
		self.capabilities = capabilities;
		self
	}

	pub fn values(mut self, values: impl Into<Vec<CString>>) -> Self {
		self.values = Some(values.into());
		self
	}

	pub fn build(self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			Some(self.name),
			self.title,
			self.description,
		);
		buf.set_value_type(crate::ValueType::STRING);
		buf.set_size(self.size as usize);
		buf.set_unit(self.unit);
		buf.set_capabilities(self.capabilities);

		if let Some(values) = self.values {
			buf.set_constraint_string_list(values);
		}

		buf
	}
}

// }}}

// ButtonOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct ButtonOptionBuilder {
	name: CString,
	title: Option<CString>,
	description: Option<CString>,
	capabilities: Capabilities,
}

#[cfg(any(doc, feature = "alloc"))]
impl ButtonOptionBuilder {
	pub fn new(name: impl Into<CString>) -> Self {
		Self {
			name: name.into(),
			title: None,
			description: None,
			capabilities: Capabilities::NONE,
		}
	}

	pub fn title(mut self, title: impl Into<CString>) -> Self {
		self.title = Some(title.into());
		self
	}

	pub fn description(
		mut self,
		description: impl Into<CString>,
	) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn capabilities(mut self, capabilities: Capabilities) -> Self {
		self.capabilities = capabilities;
		self
	}

	pub fn build(self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			Some(self.name),
			self.title,
			self.description,
		);
		buf.set_value_type(crate::ValueType::BUTTON);
		buf.set_size(0);
		buf.set_capabilities(self.capabilities);
		buf
	}
}

// }}}

// GroupOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct GroupOptionBuilder {
	title: Option<CString>,
	description: Option<CString>,
}

#[cfg(any(doc, feature = "alloc"))]
impl GroupOptionBuilder {
	pub fn new() -> Self {
		Self {
			title: None,
			description: None,
		}
	}

	pub fn title(mut self, title: impl Into<CString>) -> Self {
		self.title = Some(title.into());
		self
	}

	pub fn description(
		mut self,
		description: impl Into<CString>,
	) -> Self {
		self.description = Some(description.into());
		self
	}

	pub fn build(self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			None,
			self.title,
			self.description,
		);
		buf.set_value_type(crate::ValueType::GROUP);
		buf.set_size(0);
		buf
	}
}

// }}}

// Capabilities {{{

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Capabilities {
	bits: u32,
}

impl fmt::Debug for Capabilities {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Capabilities ")?;
		let mut dbg = f.debug_set();
		for bit in 0..32 {
			let mask: u32 = 1 << bit;
			if self.bits & mask == 0 {
				continue;
			}
			dbg.entry(&DebugCapabilityBit(mask));
		}
		dbg.finish()
	}
}

impl Capabilities {
	pub const NONE: Capabilities = Capabilities { bits: 0 };

	pub const SOFT_SELECT: Capabilities = Capabilities {
		bits: crate::CAP_SOFT_SELECT | crate::CAP_SOFT_DETECT,
	};

	pub const HARD_SELECT: Capabilities = Capabilities {
		bits: crate::CAP_HARD_SELECT,
	};

	pub const fn as_word(self) -> crate::Word {
		crate::Word::new(self.bits)
	}

	pub const fn from_word(word: crate::Word) -> Capabilities {
		Capabilities { bits: word.as_u32() }
	}

	pub fn can_soft_select(self) -> bool {
		self.bits & crate::CAP_SOFT_SELECT != 0
	}

	pub fn can_hard_select(self) -> bool {
		self.bits & crate::CAP_HARD_SELECT != 0
	}

	pub fn can_soft_detect(self) -> bool {
		self.bits & crate::CAP_SOFT_DETECT != 0
	}

	pub fn set_soft_detect(&mut self, soft_detect: bool) {
		if !self.can_soft_select() {
			self.set(crate::CAP_SOFT_DETECT, soft_detect)
		}
	}

	pub fn is_emulated(self) -> bool {
		self.bits & crate::CAP_EMULATED != 0
	}

	pub fn set_emulated(&mut self, emulated: bool) {
		self.set(crate::CAP_EMULATED, emulated)
	}

	pub fn is_automatic(self) -> bool {
		self.bits & crate::CAP_AUTOMATIC != 0
	}

	pub fn set_automatic(&mut self, automatic: bool) {
		self.set(crate::CAP_AUTOMATIC, automatic)
	}

	pub fn is_active(self) -> bool {
		self.bits & crate::CAP_INACTIVE == 0
	}

	pub fn set_active(&mut self, active: bool) {
		self.set(crate::CAP_INACTIVE, !active)
	}

	pub fn is_advanced(self) -> bool {
		self.bits & crate::CAP_ADVANCED != 0
	}

	pub fn set_advanced(&mut self, advanced: bool) {
		self.set(crate::CAP_ADVANCED, advanced)
	}

	fn set(&mut self, mask: u32, set: bool) {
		if set {
			self.bits |= mask;
		} else {
			self.bits &= !mask;
		}
	}
}

struct DebugCapabilityBit(u32);

impl fmt::Debug for DebugCapabilityBit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.0 {
			crate::CAP_SOFT_SELECT => f.write_str("SANE_CAP_SOFT_SELECT"),
			crate::CAP_HARD_SELECT => f.write_str("SANE_CAP_HARD_SELECT"),
			crate::CAP_SOFT_DETECT => f.write_str("SANE_CAP_SOFT_DETECT"),
			crate::CAP_EMULATED => f.write_str("SANE_CAP_EMULATED"),
			crate::CAP_AUTOMATIC => f.write_str("SANE_CAP_AUTOMATIC"),
			crate::CAP_INACTIVE => f.write_str("SANE_CAP_INACTIVE"),
			crate::CAP_ADVANCED => f.write_str("SANE_CAP_ADVANCED"),
			_ => write!(f, "{:#010X}", self.0),
		}
	}
}

// }}}

// Constraint {{{

#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Constraint<'a> {
	None,
	IntRange(&'a crate::Range),
	FixedRange(&'a crate::Range),
	IntList(WordList<'a>),
	FixedList(WordList<'a>),
	StringList(StringList<'a>),
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ConstraintError {
	InvalidType(crate::ConstraintType),
	TypeMismatch(crate::ValueType, crate::ConstraintType),
}

impl<'a> Constraint<'a> {
	pub unsafe fn from_ptr(
		value_type: crate::ValueType,
		constraint_type: crate::ConstraintType,
		ptr: *const (),
	) -> Result<Constraint<'a>, ConstraintError> {
		use crate::ConstraintType as C;
		use crate::ValueType as V;

		match constraint_type {
			C::NONE => Ok(Self::None),
			C::RANGE => match value_type {
				V::INT => Ok(Self::IntRange(
					&*(ptr.cast())
				)),
				V::FIXED => Ok(Self::FixedRange(
					&*(ptr.cast())
				)),
				_ => Err(ConstraintError::TypeMismatch(
					value_type,
					constraint_type,
				)),
			},
			C::WORD_LIST => match value_type {
				V::INT => Ok(Self::IntList(
					WordList::from_ptr(ptr.cast())
				)),
				V::FIXED => Ok(Self::FixedList(
					WordList::from_ptr(ptr.cast())
				)),
				_ => Err(ConstraintError::TypeMismatch(
					value_type,
					constraint_type,
				)),
			},
			C::STRING_LIST => match value_type {
				V::STRING => Ok(Self::StringList(
					StringList::from_ptr(ptr.cast())
				)),
				_ => Err(ConstraintError::TypeMismatch(
					value_type,
					constraint_type,
				)),
			},
			_ => Err(ConstraintError::InvalidType(constraint_type)),
		}
	}
}

impl fmt::Debug for Constraint<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use crate::{Fixed, Int};
		match self {
			Constraint::None => f.write_str("None"),
			Constraint::IntRange(range) => {
				let mut dbg = f.debug_struct("Range");
				dbg.field("min", &Int::from_word(range.min));
				dbg.field("max", &Int::from_word(range.max));
				dbg.field("quant", &Int::from_word(range.quant));
				dbg.finish()
			},
			Constraint::FixedRange(range) => {
				let mut dbg = f.debug_struct("Range");
				dbg.field("min", &Fixed::from_word(range.min));
				dbg.field("max", &Fixed::from_word(range.max));
				dbg.field("quant", &Fixed::from_word(range.quant));
				dbg.finish()
			},
			Constraint::IntList(values) => {
				f.debug_list()
					.entries(values.iter().map(Int::from_word))
					.finish()
			},
			Constraint::FixedList(values) => {
				f.debug_list()
					.entries(values.iter().map(Fixed::from_word))
					.finish()
			},
			Constraint::StringList(values) => values.fmt(f),
		}
	}
}

// }}}

// WordList {{{

#[derive(Copy, Clone)]
pub struct WordList<'a> {
	words: &'a crate::Word,
}

impl PartialEq for WordList<'_> {
	fn eq(&self, other: &WordList) -> bool {
		iter_eq(self, other)
	}
}

impl Eq for WordList<'_> {}

impl fmt::Debug for WordList<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

impl<'a> WordList<'a> {
	pub unsafe fn from_ptr(ptr: *const crate::Word) -> WordList<'a> {
		WordList { words: &*ptr }
	}

	pub fn as_ptr(&self) -> *const crate::Word {
		self.words
	}

	pub fn iter(&self) -> WordListIter<'a> {
		WordListIter::new(self.words)
	}
}

impl<'a> IntoIterator for &WordList<'a> {
	type Item = crate::Word;
	type IntoIter = WordListIter<'a>;

	fn into_iter(self) -> WordListIter<'a> {
		self.iter()
	}
}

// }}}

// WordListIter {{{

pub struct WordListIter<'a> {
	words: &'a crate::Word,
	len: u32,
}

impl<'a> WordListIter<'a> {
	fn new(words: &'a crate::Word) -> WordListIter<'a> {
		unsafe {
			WordListIter {
				words: ref_array_next(words),
				len: ptr::read(words).as_u32(),
			}
		}
	}
}

impl<'a> Iterator for WordListIter<'a> {
	type Item = crate::Word;

	fn next(&mut self) -> Option<crate::Word> {
		if self.len == 0 {
			return None;
		}
		self.len -= 1;

		unsafe {
			let word = ptr::read(self.words);
			self.words = ref_array_next(self.words);
			Some(word)
		}
	}
}

// }}}

// StringList {{{

#[derive(Copy, Clone)]
pub struct StringList<'a> {
	strings: &'a crate::StringConst,
}

impl PartialEq for StringList<'_> {
	fn eq(&self, other: &StringList) -> bool {
		iter_eq(self, other)
	}
}

impl Eq for StringList<'_> {}

impl fmt::Debug for StringList<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

impl<'a> StringList<'a> {
	pub unsafe fn from_ptr(ptr: *const crate::StringConst) -> StringList<'a> {
		StringList { strings: &*ptr }
	}

	pub fn as_ptr(&self) -> *const crate::StringConst {
		self.strings
	}

	pub fn iter(&self) -> StringListIter<'a> {
		StringListIter { strings: self.strings }
	}
}

impl<'a> IntoIterator for &StringList<'a> {
	type Item = &'a CStr;
	type IntoIter = StringListIter<'a>;

	fn into_iter(self) -> StringListIter<'a> {
		self.iter()
	}
}

// }}}

// StringListIter {{{

pub struct StringListIter<'a> {
	strings: &'a crate::StringConst,
}

impl<'a> Iterator for StringListIter<'a> {
	type Item = &'a CStr;

	fn next(&mut self) -> Option<&'a CStr> {
		unsafe {
			let raw = core::ptr::read(self.strings);
			let cstr = raw.to_c_str()?;
			self.strings = ref_array_next(self.strings);
			Some(cstr)
		}
	}
}

// }}}

// BufWriter {{{

pub(crate) struct BufWriter<'a> {
	buf: &'a mut [u8],
	count: usize,
}

impl<'a> BufWriter<'a> {
	pub(crate) fn new(buf: &'a mut [u8]) -> BufWriter<'a> {
		BufWriter { buf, count: 0 }
	}

	pub(crate) fn into_bytes(self) -> &'a [u8] {
		&self.buf[..self.count]
	}
}

impl fmt::Write for BufWriter<'_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let avail = &mut self.buf[self.count..];
		let b = s.as_bytes();
		if b.len() > avail.len() {
			return Err(fmt::Error);
		}
		avail[..b.len()].copy_from_slice(b);
		self.count += b.len();
		Ok(())
	}
}

// }}}

// Vendors {{{

/// `"Abaton"`
pub const VENDOR_ABATON: &CStr = cstr(b"Abaton\x00");

/// `"Acer"`
pub const VENDOR_ACER: &CStr = cstr(b"Acer\x00");

/// `"AGFA"`
pub const VENDOR_AGFA: &CStr = cstr(b"AGFA\x00");

/// `"Apple"`
pub const VENDOR_APPLE: &CStr = cstr(b"Apple\x00");

/// `"Artec"`
pub const VENDOR_ARTEC: &CStr = cstr(b"Artec\x00");

/// `"Avision"`
pub const VENDOR_AVISION: &CStr = cstr(b"Avision\x00");

/// `"CANON"`
pub const VENDOR_CANON: &CStr = cstr(b"CANON\x00");

/// `"Connectix"`
pub const VENDOR_CONNECTIX: &CStr = cstr(b"Connectix\x00");

/// `"Epson"`
pub const VENDOR_EPSON: &CStr = cstr(b"Epson\x00");

/// `"Fujitsu"`
pub const VENDOR_FUJITSU: &CStr = cstr(b"Fujitsu\x00");

/// `"Hewlett-Packard"`
pub const VENDOR_HEWLETT_PACKARD: &CStr = cstr(b"Hewlett-Packard\x00");

/// `"IBM"`
pub const VENDOR_IBM: &CStr = cstr(b"IBM\x00");

/// `"Kodak"`
pub const VENDOR_KODAK: &CStr = cstr(b"Kodak\x00");

/// `"Lexmark"`
pub const VENDOR_LEXMARK: &CStr = cstr(b"Lexmark\x00");

/// `"Logitech"`
pub const VENDOR_LOGITECH: &CStr = cstr(b"Logitech\x00");

/// `"Microtek"`
pub const VENDOR_MICROTEK: &CStr = cstr(b"Microtek\x00");

/// `"Minolta"`
pub const VENDOR_MINOLTA: &CStr = cstr(b"Minolta\x00");

/// `"Mitsubishi"`
pub const VENDOR_MITSUBISHI: &CStr = cstr(b"Mitsubishi\x00");

/// `"Mustek"`
pub const VENDOR_MUSTEK: &CStr = cstr(b"Mustek\x00");

/// `"NEC"`
pub const VENDOR_NEC: &CStr = cstr(b"NEC\x00");

/// `"Nikon"`
pub const VENDOR_NIKON: &CStr = cstr(b"Nikon\x00");

/// `"Noname"`
pub const VENDOR_NONAME: &CStr = cstr(b"Noname\x00");

/// `"Plustek"`
pub const VENDOR_PLUSTEK: &CStr = cstr(b"Plustek\x00");

/// `"Polaroid"`
pub const VENDOR_POLAROID: &CStr = cstr(b"Polaroid\x00");

/// `"Relisys"`
pub const VENDOR_RELISYS: &CStr = cstr(b"Relisys\x00");

/// `"Ricoh"`
pub const VENDOR_RICOH: &CStr = cstr(b"Ricoh\x00");

/// `"Sharp"`
pub const VENDOR_SHARP: &CStr = cstr(b"Sharp\x00");

/// `"Siemens"`
pub const VENDOR_SIEMENS: &CStr = cstr(b"Siemens\x00");

/// `"Tamarack"`
pub const VENDOR_TAMARACK: &CStr = cstr(b"Tamarack\x00");

/// `"UMAX"`
pub const VENDOR_UMAX: &CStr = cstr(b"UMAX\x00");

// }}}

// Device types {{{

/// `"film scanner"`
pub const TYPE_FILM_SCANNER: &CStr = cstr(b"film scanner\x00");

/// `"flatbed scanner"`
pub const TYPE_FLATBED_SCANNER: &CStr = cstr(b"flatbed scanner\x00");

/// `"frame grabber"`
pub const TYPE_FRAME_GRABBER: &CStr = cstr(b"frame grabber\x00");

/// `"handheld scanner"`
pub const TYPE_HANDHELD_SCANNER: &CStr = cstr(b"handheld scanner\x00");

/// `"multi-function peripheral"`
pub const TYPE_MULTI_FUNCTION_PERIPHERAL: &CStr = cstr(b"multi-function peripheral\x00");

/// `"sheetfed scanner"`
pub const TYPE_SHEETFED_SCANNER: &CStr = cstr(b"sheetfed scanner\x00");

/// `"still camera"`
pub const TYPE_STILL_CAMERA: &CStr = cstr(b"still camera\x00");

/// `"video camera"`
pub const TYPE_VIDEO_CAMERA: &CStr = cstr(b"video camera\x00");

/// `"virtual device"`
pub const TYPE_VIRTUAL_DEVICE: &CStr = cstr(b"virtual device\x00");

// }}}
