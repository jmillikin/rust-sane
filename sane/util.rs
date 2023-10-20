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

const CSTR_EMPTY: &CStr = cstr(b"\x00");

unsafe fn ref_array_next<T>(t: &T) -> &T {
	&*((t as *const T).add(1))
}

// Device {{{

#[derive(Clone, Copy, Debug)]
pub struct Device<'a> {
	name: &'a CStr,
	vendor: &'a CStr,
	model: &'a CStr,
	kind: &'a CStr,
}

impl Device<'_> {
	pub fn name(&self) -> &CStr {
		self.name
	}

	pub fn vendor(&self) -> &CStr {
		self.vendor
	}

	pub fn model(&self) -> &CStr {
		self.model
	}

	pub fn kind(&self) -> &CStr {
		self.kind
	}
}

impl<'a> Device<'a> {
	pub unsafe fn from_ptr(ptr: *const crate::Device) -> Device<'a> {
		let raw = &*ptr;
		Device {
			name: raw.name.to_c_str().unwrap_or(CSTR_EMPTY),
			vendor: raw.vendor.to_c_str().unwrap_or(CSTR_EMPTY),
			model: raw.model.to_c_str().unwrap_or(CSTR_EMPTY),
			kind: raw.r#type.to_c_str().unwrap_or(CSTR_EMPTY),
		}
	}
}

// }}}

// DeviceBuf {{{

#[cfg(any(doc, feature = "alloc"))]
#[derive(Clone, Debug)]
pub struct DeviceBuf {
	name: CString,
	vendor: Cow<'static, CStr>,
	model: Cow<'static, CStr>,
	kind: Cow<'static, CStr>,
}

#[cfg(any(doc, feature = "alloc"))]
impl DeviceBuf {
	pub fn new(name: impl Into<CString>) -> DeviceBuf {
		DeviceBuf {
			name: name.into(),
			vendor: Cow::Borrowed(CSTR_EMPTY),
			model: Cow::Borrowed(CSTR_EMPTY),
			kind: Cow::Borrowed(CSTR_EMPTY),
		}
	}

	pub fn name(&self) -> &CStr {
		&self.name
	}

	pub fn set_name(&mut self, name: impl Into<CString>) {
		self.name = name.into();
	}

	pub fn vendor(&self) -> &CStr {
		self.vendor.as_ref()
	}

	pub fn set_vendor(&mut self, vendor: impl Into<CString>) {
		self.vendor = Cow::Owned(vendor.into());
	}

	pub fn model(&self) -> &CStr {
		self.model.as_ref()
	}

	pub fn set_model(&mut self, model: impl Into<CString>) {
		self.model = Cow::Owned(model.into());
	}

	pub fn kind(&self) -> &CStr {
		self.kind.as_ref()
	}

	pub fn set_kind(&mut self, kind: impl Into<CString>) {
		self.kind = Cow::Owned(kind.into());
	}
}

// }}}

// Devices {{{

pub struct Devices<'a> {
	devices: &'a *const crate::Device,
}

impl fmt::Debug for Devices<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

impl<'a> Devices<'a> {
	pub unsafe fn from_ptr(
		ptr: *const *const crate::Device,
	) -> Devices<'a> {
		Devices { devices: &*ptr }
	}

	pub fn iter(&self) -> DevicesIter<'a> {
		DevicesIter { devices: self.devices }
	}
}

impl<'a> IntoIterator for &Devices<'a> {
	type Item = Device<'a>;
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
	type Item = Device<'a>;

	fn next(&mut self) -> Option<Device<'a>> {
		Some(unsafe {
			let device_ptr: *const _ = self.devices.as_ref()?;
			self.devices = ref_array_next(self.devices);
			Device::from_ptr(device_ptr)
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
impl fmt::Debug for DevicesBuf {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.devices().fmt(f)
	}
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

	pub fn devices(&self) -> Devices {
		Devices { devices: &self.device_ptrs[0] }
	}

	pub fn as_ptr(&self) -> *const *const crate::Device {
		self.device_ptrs.as_ptr()
	}
}

// }}}

// OptionDescriptor {{{

#[derive(Copy, Clone)]
pub struct OptionDescriptor<'a> {
	name: &'a CStr,
	title: &'a CStr,
	description: &'a CStr,
	value_type: crate::ValueType,
	unit: crate::Unit,
	size: u32,
	capabilities: Capabilities,
	constraint: Constraint<'a>,
}

impl fmt::Debug for OptionDescriptor<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("OptionDescriptor")
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
}

impl<'a> OptionDescriptor<'a> {
	pub unsafe fn from_ptr(
		ptr: *const crate::OptionDescriptor,
	) -> OptionDescriptor<'a> {
		let raw = &*ptr;
		OptionDescriptor {
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
		}
	}
}

impl OptionDescriptor<'_> {
	pub fn name(&self) -> &CStr {
		self.name
	}

	pub fn title(&self) -> &CStr {
		self.title
	}

	pub fn description(&self) -> &CStr {
		self.description
	}

	pub fn value_type(&self) -> crate::ValueType {
		self.value_type
	}

	pub fn unit(&self) -> crate::Unit {
		self.unit
	}

	pub fn size(&self) -> usize {
		self.size as usize
	}

	pub fn capabilities(&self) -> Capabilities {
		self.capabilities
	}

	pub fn constraint(&self) -> Constraint {
		self.constraint
	}
}

//}}}

// OptionDescriptorBuf {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct OptionDescriptorBuf {
	raw: Box<crate::OptionDescriptor>,
	strings: Vec<CString>,
	constraint_range: Option<Box<crate::Range>>,
	constraint_word_list: Vec<crate::Word>,
	constraint_string_list: Vec<crate::StringConst>,
}

#[cfg(any(doc, feature = "alloc"))]
impl OptionDescriptorBuf {
	fn new(name: &CStr, title: &CStr, desc: &CStr) -> OptionDescriptorBuf {
		let mut strings = Vec::new();

		let cstr_empty_ptr = crate::StringConst::from_c_str(CSTR_EMPTY);
		let mut take_cstr = |cstr: &CStr| -> crate::StringConst {
			if cstr.is_empty() {
				cstr_empty_ptr
			} else {
				let owned = CString::from(cstr);
				let ptr = crate::StringConst::from_c_str(&owned);
				strings.push(owned);
				ptr
			}
		};

		let mut raw = Box::new(crate::OptionDescriptor::new());
		raw.name = take_cstr(name);
		raw.title = take_cstr(title);
		raw.desc = take_cstr(desc);

		OptionDescriptorBuf {
			raw,
			strings,
			constraint_range: None,
			constraint_word_list: Vec::new(),
			constraint_string_list: Vec::new(),
		}
	}

	pub fn option_descriptor<'a>(&'a self) -> OptionDescriptor<'a> {
		unsafe { OptionDescriptor::from_ptr(self.as_ptr()) }
	}

	pub fn as_ptr(&self) -> *const crate::OptionDescriptor {
		Box::as_ref(&self.raw)
	}
}

// }}}

// BoolOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct BoolOptionBuilder<'a> {
	name: &'a CStr,
	title: &'a CStr,
	description: &'a CStr,
	capabilities: Capabilities,
}

#[cfg(any(doc, feature = "alloc"))]
impl<'a> BoolOptionBuilder<'a> {
	pub fn new(name: &'a CStr) -> Self {
		Self {
			name,
			title: CSTR_EMPTY,
			description: CSTR_EMPTY,
			capabilities: Capabilities::new(),
		}
	}

	pub fn title(&mut self, title: &'a CStr) -> &mut Self {
		self.title = title;
		self
	}

	pub fn description(&mut self, description: &'a CStr) -> &mut Self {
		self.description = description;
		self
	}

	pub fn capabilities(&mut self, capabilities: Capabilities) -> &mut Self {
		self.capabilities = capabilities;
		self
	}

	pub fn build(&self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			self.name,
			self.title,
			self.description,
		);
		buf.raw.r#type = crate::ValueType::BOOL;
		buf.raw.size = crate::Int::new(size_of::<crate::Bool>() as i32);
		buf.raw.cap = self.capabilities.as_int();
		buf
	}
}

// }}}

// IntOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct IntOptionBuilder<'a> {
	name: &'a CStr,
	title: &'a CStr,
	description: &'a CStr,
	unit: crate::Unit,
	capabilities: Capabilities,
	size: i32,
	range: Option<crate::Range>,
	values: Option<&'a [i32]>,
}

#[cfg(any(doc, feature = "alloc"))]
impl<'a> IntOptionBuilder<'a> {
	pub fn new(name: &'a CStr) -> Self {
		Self {
			name,
			title: CSTR_EMPTY,
			description: CSTR_EMPTY,
			unit: crate::Unit::NONE,
			capabilities: Capabilities::new(),
			size: size_of::<crate::Int>() as i32,
			range: None,
			values: None,
		}
	}

	pub fn title(&mut self, title: &'a CStr) -> &mut Self {
		self.title = title;
		self
	}

	pub fn description(&mut self, description: &'a CStr) -> &mut Self {
		self.description = description;
		self
	}

	pub fn unit(&mut self, unit: crate::Unit) -> &mut Self {
		self.unit = unit;
		self
	}

	pub fn capabilities(&mut self, capabilities: Capabilities) -> &mut Self {
		self.capabilities = capabilities;
		self
	}

	pub fn count(&mut self, count: usize) -> &mut Self {
		// FIXME: assert count > 0 ?
		// FIXME: assert count*sizeof(Int) <= i32::MAX ?
		self.size = (count * size_of::<crate::Int>()) as i32;
		self
	}

	pub fn range(&mut self, min: i32, max: i32, quant: i32) -> &mut Self {
		let mut range = crate::Range::new();
		range.min = crate::Int::new(min).as_word();
		range.max = crate::Int::new(max).as_word();
		range.quant = crate::Int::new(quant).as_word();
		self.range = Some(range);
		self.values = None;
		self
	}

	pub fn values(&mut self, values: &'a [i32]) -> &mut Self {
		self.values = Some(values);
		self.range = None;
		self
	}

	pub fn build(&self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			self.name,
			self.title,
			self.description,
		);
		buf.raw.r#type = crate::ValueType::INT;
		buf.raw.size = crate::Int::new(self.size);
		buf.raw.unit = self.unit;
		buf.raw.cap = self.capabilities.as_int();

		if let Some(range) = self.range {
			let range_box = Box::new(range);
			let range_ptr: *const crate::Range = range_box.as_ref();
			buf.constraint_range = Some(range_box);
			buf.raw.constraint_type = crate::ConstraintType::RANGE;
			buf.raw.constraint = range_ptr.cast();
		} else if let Some(values) = self.values {
			let word_list = &mut buf.constraint_word_list;
			word_list.push(crate::Word::new(
				values.len() as u32,
			));
			for value in values {
				word_list.push(crate::Int::new(*value).as_word());
			}
			buf.raw.constraint_type = crate::ConstraintType::WORD_LIST;
			buf.raw.constraint = word_list.as_ptr().cast();
		}

		buf
	}
}

// }}}

// FixedOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct FixedOptionBuilder<'a> {
	name: &'a CStr,
	title: &'a CStr,
	description: &'a CStr,
	unit: crate::Unit,
	capabilities: Capabilities,
	size: i32,
	range: Option<crate::Range>,
	values: Option<&'a [crate::Fixed]>,
}

#[cfg(any(doc, feature = "alloc"))]
impl<'a> FixedOptionBuilder<'a> {
	pub fn new(name: &'a CStr) -> Self {
		Self {
			name,
			title: CSTR_EMPTY,
			description: CSTR_EMPTY,
			unit: crate::Unit::NONE,
			capabilities: Capabilities::new(),
			size: size_of::<crate::Fixed>() as i32,
			range: None,
			values: None,
		}
	}

	pub fn title(&mut self, title: &'a CStr) -> &mut Self {
		self.title = title;
		self
	}

	pub fn description(&mut self, description: &'a CStr) -> &mut Self {
		self.description = description;
		self
	}

	pub fn unit(&mut self, unit: crate::Unit) -> &mut Self {
		self.unit = unit;
		self
	}

	pub fn capabilities(&mut self, capabilities: Capabilities) -> &mut Self {
		self.capabilities = capabilities;
		self
	}

	pub fn count(&mut self, count: usize) -> &mut Self {
		// FIXME: assert count > 0 ?
		// FIXME: assert count*sizeof(Int) <= i32::MAX ?
		self.size = (count * size_of::<crate::Fixed>()) as i32;
		self
	}

	pub fn range(
		&mut self,
		min: crate::Fixed,
		max: crate::Fixed,
		quant: crate::Fixed,
	) -> &mut Self {
		let mut range = crate::Range::new();
		range.min = min.as_word();
		range.max = max.as_word();
		range.quant = quant.as_word();
		self.range = Some(range);
		self.values = None;
		self
	}

	pub fn values(&mut self, values: &'a [crate::Fixed]) -> &mut Self {
		self.values = Some(values);
		self.range = None;
		self
	}

	pub fn build(&self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			self.name,
			self.title,
			self.description,
		);
		buf.raw.r#type = crate::ValueType::FIXED;
		buf.raw.size = crate::Int::new(self.size);
		buf.raw.unit = self.unit;
		buf.raw.cap = self.capabilities.as_int();

		if let Some(range) = self.range {
			let range_box = Box::new(range);
			let range_ptr: *const crate::Range = range_box.as_ref();
			buf.constraint_range = Some(range_box);
			buf.raw.constraint_type = crate::ConstraintType::RANGE;
			buf.raw.constraint = range_ptr.cast();
		} else if let Some(values) = self.values {
			let word_list = &mut buf.constraint_word_list;
			word_list.push(crate::Word::new(
				values.len() as u32,
			));
			for value in values {
				word_list.push(value.as_word());
			}
			buf.raw.constraint_type = crate::ConstraintType::WORD_LIST;
			buf.raw.constraint = word_list.as_ptr().cast();
		}

		buf
	}
}

// }}}

// StringOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct StringOptionBuilder<'a> {
	name: &'a CStr,
	title: &'a CStr,
	description: &'a CStr,
	unit: crate::Unit,
	capabilities: Capabilities,
	size: i32,
	values: Option<&'a [&'a CStr]>,
}

#[cfg(any(doc, feature = "alloc"))]
impl<'a> StringOptionBuilder<'a> {
	pub fn new(name: &'a CStr, size: usize) -> Self {
		// FIXME: assert size <= i32::MAX
		Self {
			name,
			size: size as i32,
			title: CSTR_EMPTY,
			description: CSTR_EMPTY,
			unit: crate::Unit::NONE,
			capabilities: Capabilities::new(),
			values: None,
		}
	}

	pub fn title(&mut self, title: &'a CStr) -> &mut Self {
		self.title = title;
		self
	}

	pub fn description(&mut self, description: &'a CStr) -> &mut Self {
		self.description = description;
		self
	}

	pub fn unit(&mut self, unit: crate::Unit) -> &mut Self {
		self.unit = unit;
		self
	}

	pub fn capabilities(&mut self, capabilities: Capabilities) -> &mut Self {
		self.capabilities = capabilities;
		self
	}

	pub fn values(&mut self, values: &'a [&'a CStr]) -> &mut Self {
		self.values = Some(values);
		self
	}

	pub fn build(&self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			self.name,
			self.title,
			self.description,
		);
		buf.raw.r#type = crate::ValueType::STRING;
		buf.raw.size = crate::Int::new(self.size);
		buf.raw.unit = self.unit;
		buf.raw.cap = self.capabilities.as_int();

		if let Some(values) = self.values {
			let string_list = &mut buf.constraint_string_list;
			for value in values {
				let owned = CString::from(*value);
				string_list.push(crate::StringConst::from_c_str(&owned));
				buf.strings.push(owned);
			}
			string_list.push(crate::StringConst::null());
			buf.raw.constraint_type = crate::ConstraintType::STRING_LIST;
			buf.raw.constraint = string_list.as_ptr().cast();
		}

		buf
	}
}

// }}}

// ButtonOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct ButtonOptionBuilder<'a> {
	name: &'a CStr,
	title: &'a CStr,
	description: &'a CStr,
	capabilities: Capabilities,
}

#[cfg(any(doc, feature = "alloc"))]
impl<'a> ButtonOptionBuilder<'a> {
	pub fn new(name: &'a CStr) -> Self {
		Self {
			name,
			title: CSTR_EMPTY,
			description: CSTR_EMPTY,
			capabilities: Capabilities::new(),
		}
	}

	pub fn title(&mut self, title: &'a CStr) -> &mut Self {
		self.title = title;
		self
	}

	pub fn description(&mut self, description: &'a CStr) -> &mut Self {
		self.description = description;
		self
	}

	pub fn capabilities(&mut self, capabilities: Capabilities) -> &mut Self {
		self.capabilities = capabilities;
		self
	}

	pub fn build(&self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			self.name,
			self.title,
			self.description,
		);
		buf.raw.r#type = crate::ValueType::BUTTON;
		buf.raw.size = crate::Int::new(0);
		buf.raw.cap = self.capabilities.as_int();
		buf
	}
}

// }}}

// GroupOptionBuilder {{{

#[cfg(any(doc, feature = "alloc"))]
pub struct GroupOptionBuilder<'a> {
	title: &'a CStr,
	description: &'a CStr,
}

#[cfg(any(doc, feature = "alloc"))]
impl<'a> GroupOptionBuilder<'a> {
	pub fn new() -> Self {
		Self {
			title: CSTR_EMPTY,
			description: CSTR_EMPTY,
		}
	}

	pub fn title(&mut self, title: &'a CStr) -> &mut Self {
		self.title = title;
		self
	}

	pub fn description(&mut self, description: &'a CStr) -> &mut Self {
		self.description = description;
		self
	}

	pub fn build(&self) -> OptionDescriptorBuf {
		let mut buf = OptionDescriptorBuf::new(
			CSTR_EMPTY,
			self.title,
			self.description,
		);
		buf.raw.r#type = crate::ValueType::GROUP;
		buf.raw.size = crate::Int::new(0);
		buf
	}
}

// }}}

// Capabilities {{{

#[derive(Clone, Copy)]
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
	pub const SOFT_SELECT: Capabilities = Capabilities {
		bits: crate::CAP_SOFT_SELECT | crate::CAP_SOFT_DETECT,
	};

	pub const HARD_SELECT: Capabilities = Capabilities {
		bits: crate::CAP_HARD_SELECT,
	};

	pub fn new() -> Capabilities {
		Self { bits: 0 }
	}

	#[allow(dead_code)]
	fn as_int(self) -> crate::Int {
		crate::Int::new(self.bits as i32)
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
#[derive(Copy, Clone)]
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
