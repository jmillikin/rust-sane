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
