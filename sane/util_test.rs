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

const CSTR_EMPTY: &CStr = cstr(b"\x00");

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

#[test]
fn util_devices_buf() {
	let mut buf = util::DevicesBuf::new();

	buf.add(CSTR_DEV_NAME, |dev| {
		dev.set_vendor(CSTR_DEV_VENDOR);
		dev.set_model(CSTR_DEV_MODEL);
		dev.set_kind(CSTR_DEV_TYPE);
	});

	const CSTR_DEV_NAME_2: &CStr = cstr(b"device-name-2\x00");
	buf.add(CSTR_DEV_NAME_2, |_dev| {});

	let devices = unsafe { util::Devices::from_ptr(buf.as_ptr()) };
	let devices_vec: Vec<_> = devices.into_iter().collect();

	assert_eq!(devices_vec.len(), 2);

	assert_eq!(devices_vec[0].name(), CSTR_DEV_NAME);
	assert_eq!(devices_vec[0].vendor(), CSTR_DEV_VENDOR);
	assert_eq!(devices_vec[0].model(), CSTR_DEV_MODEL);
	assert_eq!(devices_vec[0].kind(), CSTR_DEV_TYPE);

	assert_eq!(devices_vec[1].name(), CSTR_DEV_NAME_2);
	assert_eq!(devices_vec[1].vendor(), CSTR_EMPTY);
	assert_eq!(devices_vec[1].model(), CSTR_EMPTY);
	assert_eq!(devices_vec[1].kind(), CSTR_EMPTY);

	let devices_vec_2: Vec<_> = buf.devices().into_iter().collect();
	assert_eq!(devices_vec_2.len(), 2);
}

#[test]
fn util_capabilities() {
	let _ = util::Capabilities::new().clone();

	fn cap(bit: u32) -> util::Capabilities {
		unsafe { core::mem::transmute(bit) }
	}

	assert_eq!(format!("{:?}", cap(0)), "Capabilities {}");

	assert_eq!(
		format!("{:?}", cap(sane::CAP_SOFT_SELECT)),
		"Capabilities {SANE_CAP_SOFT_SELECT}",
	);
	assert_eq!(
		format!("{:?}", cap(sane::CAP_HARD_SELECT)),
		"Capabilities {SANE_CAP_HARD_SELECT}",
	);
	assert_eq!(
		format!("{:?}", cap(sane::CAP_SOFT_DETECT)),
		"Capabilities {SANE_CAP_SOFT_DETECT}",
	);
	assert_eq!(
		format!("{:?}", cap(sane::CAP_EMULATED)),
		"Capabilities {SANE_CAP_EMULATED}",
	);
	assert_eq!(
		format!("{:?}", cap(sane::CAP_AUTOMATIC)),
		"Capabilities {SANE_CAP_AUTOMATIC}",
	);
	assert_eq!(
		format!("{:?}", cap(sane::CAP_INACTIVE)),
		"Capabilities {SANE_CAP_INACTIVE}",
	);
	assert_eq!(
		format!("{:?}", cap(sane::CAP_ADVANCED)),
		"Capabilities {SANE_CAP_ADVANCED}",
	);
	assert_eq!(
		format!("{:?}", cap(1u32 << 31)),
		"Capabilities {0x80000000}",
	);
}

#[test]
fn util_word_list() {
	let raw = [
		sane::Word::new(3),
		sane::Word::new(10),
		sane::Word::new(20),
		sane::Word::new(30),
	];
	let raw_ptr: *const sane::Word = raw.as_ptr();

	let word_list = unsafe { util::WordList::from_ptr(raw_ptr) };
	let words: Vec<_> = word_list.into_iter().collect();

	assert_eq!(words, vec![
		sane::Word::new(10),
		sane::Word::new(20),
		sane::Word::new(30),
	]);
}

#[test]
fn util_string_list() {
	let raw = [
		cstr(b"aaa\x00").as_ptr(),
		cstr(b"bbb\x00").as_ptr(),
		cstr(b"ccc\x00").as_ptr(),
		ptr::null(),
	];
	let raw_ptr: *const sane::StringConst = raw.as_ptr().cast();

	let string_list = unsafe { util::StringList::from_ptr(raw_ptr) };
	let strings: Vec<_> = string_list.into_iter().collect();

	assert_eq!(strings, vec![
		cstr(b"aaa\x00"),
		cstr(b"bbb\x00"),
		cstr(b"ccc\x00"),
	]);
}