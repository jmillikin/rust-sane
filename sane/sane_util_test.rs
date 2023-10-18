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
use core::ptr;

use sane::StringConst;
use sane::util;

const fn cstr(bytes: &[u8]) -> &CStr {
	unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
}

const CSTR_DEV_NAME: &CStr = cstr(b"device-name\x00");
const CSTR_DEV_VENDOR: &CStr = cstr(b"device-vendor\x00");
const CSTR_DEV_MODEL: &CStr = cstr(b"device-model\x00");
const CSTR_DEV_TYPE: &CStr = cstr(b"device-type\x00");

#[test]
fn util_device() {
	let mut raw = sane::Device::new();
	raw.name = StringConst::from_c_str(CSTR_DEV_NAME);
	raw.vendor = StringConst::from_c_str(CSTR_DEV_VENDOR);
	raw.model = StringConst::from_c_str(CSTR_DEV_MODEL);
	raw.r#type = StringConst::from_c_str(CSTR_DEV_TYPE);

	let device = unsafe { util::Device::from_ptr(&raw) };

	assert_eq!(device.name(), CSTR_DEV_NAME);
	assert_eq!(device.vendor(), CSTR_DEV_VENDOR);
	assert_eq!(device.model(), CSTR_DEV_MODEL);
	assert_eq!(device.kind(), CSTR_DEV_TYPE);

	assert_eq!(
		format!("{:#?}", device),
		concat!(
			"Device {\n",
			"    name: \"device-name\",\n",
			"    vendor: \"device-vendor\",\n",
			"    model: \"device-model\",\n",
			"    kind: \"device-type\",\n",
			"}",
		),
	);
}

#[test]
fn util_devices_iter() {
	let mut raw = sane::Device::new();
	raw.name = StringConst::from_c_str(CSTR_DEV_NAME);
	raw.vendor = StringConst::from_c_str(CSTR_DEV_VENDOR);
	raw.model = StringConst::from_c_str(CSTR_DEV_MODEL);
	raw.r#type = StringConst::from_c_str(CSTR_DEV_TYPE);

	let raw_devices: &[*const _] = &[&raw, ptr::null()];
	let devices = unsafe { util::Devices::from_ptr(raw_devices.as_ptr()) };

	let devices_vec: Vec<_> = devices.into_iter().collect();
	assert_eq!(devices_vec.len(), 1);

	assert_eq!(
		format!("{:#?}", devices_vec[0]),
		concat!(
			"Device {\n",
			"    name: \"device-name\",\n",
			"    vendor: \"device-vendor\",\n",
			"    model: \"device-model\",\n",
			"    kind: \"device-type\",\n",
			"}",
		),
	);
}
