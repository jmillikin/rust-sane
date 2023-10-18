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

use core::ffi::CStr;
use core::fmt;

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

// }}}

// Devices {{{

pub struct Devices<'a> {
	devices: &'a *const crate::Device,
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
	pub fn new() -> Capabilities {
		Self { bits: 0 }
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
