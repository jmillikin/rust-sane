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
use core::mem::size_of;
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

const CSTR_OPT_NAME: &CStr = cstr(b"option-name\x00");
const CSTR_OPT_TITLE: &CStr = cstr(b"option-title\x00");
const CSTR_OPT_DESC: &CStr = cstr(b"option-description\x00");

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

	assert_eq!(
		format!("{:#?}", devices),
		concat!(
			"[\n",
			"    Device {\n",
			"        name: \"device-name\",\n",
			"        vendor: \"device-vendor\",\n",
			"        model: \"device-model\",\n",
			"        kind: \"device-type\",\n",
			"    },\n",
			"]",
		),
	);

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

	assert_eq!(
		format!("{:#?}", buf),
		concat!(
			"[\n",
			"    Device {\n",
			"        name: \"device-name\",\n",
			"        vendor: \"device-vendor\",\n",
			"        model: \"device-model\",\n",
			"        kind: \"device-type\",\n",
			"    },\n",
			"    Device {\n",
			"        name: \"device-name-2\",\n",
			"        vendor: \"\",\n",
			"        model: \"\",\n",
			"        kind: \"\",\n",
			"    },\n",
			"]",
		),
	);

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
fn util_option_descriptor() {
	let mut raw = sane::OptionDescriptor::new();
	raw.name = StringConst::from_c_str(CSTR_OPT_NAME);
	raw.title = StringConst::from_c_str(CSTR_OPT_TITLE);
	raw.desc = StringConst::from_c_str(CSTR_OPT_DESC);
	raw.r#type = sane::ValueType::INT;
	raw.unit = sane::Unit::PIXEL;
	raw.size = sane::Int::new(size_of::<sane::Int>() as i32);

	let option = unsafe { util::OptionDescriptor::from_ptr(&raw) };

	assert_eq!(option.name(), CSTR_OPT_NAME);
	assert_eq!(option.title(), CSTR_OPT_TITLE);
	assert_eq!(option.description(), CSTR_OPT_DESC);
	assert_eq!(option.value_type(), sane::ValueType::INT);
	assert_eq!(option.unit(), sane::Unit::PIXEL);
	assert_eq!(option.size(), size_of::<sane::Int>());
	//assert_eq!(option.capabilities(), util::Capabilities::new());
	assert!(matches!(option.constraint(), util::Constraint::None));

	assert_eq!(
		format!("{:#?}", option),
		concat!(
			"OptionDescriptor {\n",
			"    name: \"option-name\",\n",
			"    title: \"option-title\",\n",
			"    description: \"option-description\",\n",
			"    value_type: SANE_TYPE_INT,\n",
			"    unit: SANE_UNIT_PIXEL,\n",
			"    size: 4,\n",
			"    capabilities: Capabilities {},\n",
			"    constraint: None,\n",
			"}",
		),
	);
}

#[test]
fn bool_option_builder() {
	let buf = util::BoolOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.capabilities({
			let mut caps = util::Capabilities::new();
			caps.set_emulated(true);
			caps
		})
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.name(), CSTR_OPT_NAME);
	assert_eq!(option.title(), CSTR_OPT_TITLE);
	assert_eq!(option.description(), CSTR_OPT_DESC);
	assert_eq!(option.value_type(), sane::ValueType::BOOL);
	assert_eq!(option.size(), size_of::<sane::Bool>());

	assert_eq!(
		format!("{:#?}", option),
		concat!(
			"OptionDescriptor {\n",
			"    name: \"option-name\",\n",
			"    title: \"option-title\",\n",
			"    description: \"option-description\",\n",
			"    value_type: SANE_TYPE_BOOL,\n",
			"    unit: SANE_UNIT_NONE,\n",
			"    size: 4,\n",
			"    capabilities: Capabilities {\n",
			"        SANE_CAP_EMULATED,\n",
			"    },\n",
			"    constraint: None,\n",
			"}",
		),
	);
}

#[test]
fn int_option_builder() {
	let buf = util::IntOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.unit(sane::Unit::PIXEL)
		.capabilities({
			let mut caps = util::Capabilities::new();
			caps.set_emulated(true);
			caps
		})
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.name(), CSTR_OPT_NAME);
	assert_eq!(option.title(), CSTR_OPT_TITLE);
	assert_eq!(option.description(), CSTR_OPT_DESC);
	assert_eq!(option.value_type(), sane::ValueType::INT);
	assert_eq!(option.unit(), sane::Unit::PIXEL);
	assert_eq!(option.size(), size_of::<sane::Int>());

	assert_eq!(
		format!("{:#?}", option),
		concat!(
			"OptionDescriptor {\n",
			"    name: \"option-name\",\n",
			"    title: \"option-title\",\n",
			"    description: \"option-description\",\n",
			"    value_type: SANE_TYPE_INT,\n",
			"    unit: SANE_UNIT_PIXEL,\n",
			"    size: 4,\n",
			"    capabilities: Capabilities {\n",
			"        SANE_CAP_EMULATED,\n",
			"    },\n",
			"    constraint: None,\n",
			"}",
		),
	);
}

#[test]
fn int_option_builder_count() {
	let buf = util::IntOptionBuilder::new(CSTR_EMPTY)
		.count(123)
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.size(), 123 * size_of::<sane::Int>());
}

#[test]
fn int_option_builder_range() {
	let buf = util::IntOptionBuilder::new(CSTR_EMPTY)
		.range(0, 100, 1)
		.build();

	let option = buf.option_descriptor();

	assert_eq!(
		format!("{:#?}", option.constraint()),
		concat!(
			"Range {\n",
			"    min: SANE_Int(0),\n",
			"    max: SANE_Int(100),\n",
			"    quant: SANE_Int(1),\n",
			"}",
		),
	);
}

#[test]
fn int_option_builder_values() {
	let buf = util::IntOptionBuilder::new(CSTR_EMPTY)
		.values(&[1, 2, 3])
		.build();

	let option = buf.option_descriptor();

	assert_eq!(
		format!("{:#?}", option.constraint()),
		concat!(
			"[\n",
			"    SANE_Int(1),\n",
			"    SANE_Int(2),\n",
			"    SANE_Int(3),\n",
			"]",
		),
	);
}

#[test]
fn fixed_option_builder() {
	let buf = util::FixedOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.unit(sane::Unit::PIXEL)
		.capabilities({
			let mut caps = util::Capabilities::new();
			caps.set_emulated(true);
			caps
		})
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.name(), CSTR_OPT_NAME);
	assert_eq!(option.title(), CSTR_OPT_TITLE);
	assert_eq!(option.description(), CSTR_OPT_DESC);
	assert_eq!(option.value_type(), sane::ValueType::FIXED);
	assert_eq!(option.unit(), sane::Unit::PIXEL);
	assert_eq!(option.size(), size_of::<sane::Fixed>());

	assert_eq!(
		format!("{:#?}", option),
		concat!(
			"OptionDescriptor {\n",
			"    name: \"option-name\",\n",
			"    title: \"option-title\",\n",
			"    description: \"option-description\",\n",
			"    value_type: SANE_TYPE_FIXED,\n",
			"    unit: SANE_UNIT_PIXEL,\n",
			"    size: 4,\n",
			"    capabilities: Capabilities {\n",
			"        SANE_CAP_EMULATED,\n",
			"    },\n",
			"    constraint: None,\n",
			"}",
		),
	);
}

#[test]
fn fixed_option_builder_count() {
	let buf = util::FixedOptionBuilder::new(CSTR_EMPTY)
		.count(123)
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.size(), 123 * size_of::<sane::Fixed>());
}

#[test]
fn fixed_option_builder_range() {
	let buf = util::FixedOptionBuilder::new(CSTR_EMPTY)
		.range(
			sane::Fixed::new(0, 0),
			sane::Fixed::new(100, 0),
			sane::Fixed::new(1, 0),
		)
		.build();

	let option = buf.option_descriptor();

	assert_eq!(
		format!("{:#?}", option.constraint()),
		concat!(
			"Range {\n",
			"    min: SANE_Fixed(0.0),\n",
			"    max: SANE_Fixed(100.0),\n",
			"    quant: SANE_Fixed(1.0),\n",
			"}",
		),
	);
}

#[test]
fn fixed_option_builder_values() {
	let buf = util::FixedOptionBuilder::new(CSTR_EMPTY)
		.values(&[
			sane::Fixed::new(1, 0),
			sane::Fixed::new(2, 0),
			sane::Fixed::new(3, 0),
		])
		.build();

	let option = buf.option_descriptor();

	assert_eq!(
		format!("{:#?}", option.constraint()),
		concat!(
			"[\n",
			"    SANE_Fixed(1.0),\n",
			"    SANE_Fixed(2.0),\n",
			"    SANE_Fixed(3.0),\n",
			"]",
		),
	);
}

#[test]
fn string_option_builder() {
	let buf = util::StringOptionBuilder::new(CSTR_OPT_NAME, 1234)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.unit(sane::Unit::PIXEL)
		.capabilities({
			let mut caps = util::Capabilities::new();
			caps.set_emulated(true);
			caps
		})
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.name(), CSTR_OPT_NAME);
	assert_eq!(option.title(), CSTR_OPT_TITLE);
	assert_eq!(option.description(), CSTR_OPT_DESC);
	assert_eq!(option.value_type(), sane::ValueType::STRING);
	assert_eq!(option.unit(), sane::Unit::PIXEL);
	assert_eq!(option.size(), 1234);

	assert_eq!(
		format!("{:#?}", option),
		concat!(
			"OptionDescriptor {\n",
			"    name: \"option-name\",\n",
			"    title: \"option-title\",\n",
			"    description: \"option-description\",\n",
			"    value_type: SANE_TYPE_STRING,\n",
			"    unit: SANE_UNIT_PIXEL,\n",
			"    size: 1234,\n",
			"    capabilities: Capabilities {\n",
			"        SANE_CAP_EMULATED,\n",
			"    },\n",
			"    constraint: None,\n",
			"}",
		),
	);
}

#[test]
fn string_option_builder_values() {
	let buf = util::StringOptionBuilder::new(CSTR_EMPTY, 1234)
		.values(&[
			cstr(b"aaa\x00"),
			cstr(b"bbb\x00"),
			cstr(b"ccc\x00"),
		])
		.build();

	let option = buf.option_descriptor();

	assert_eq!(
		format!("{:#?}", option.constraint()),
		concat!(
			"[\n",
			"    \"aaa\",\n",
			"    \"bbb\",\n",
			"    \"ccc\",\n",
			"]",
		),
	);
}

#[test]
fn button_option_builder() {
	let buf = util::ButtonOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.capabilities({
			let mut caps = util::Capabilities::new();
			caps.set_emulated(true);
			caps
		})
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.name(), CSTR_OPT_NAME);
	assert_eq!(option.title(), CSTR_OPT_TITLE);
	assert_eq!(option.description(), CSTR_OPT_DESC);
	assert_eq!(option.value_type(), sane::ValueType::BUTTON);
	assert_eq!(option.unit(), sane::Unit::NONE);
	assert_eq!(option.size(), 0);

	assert_eq!(
		format!("{:#?}", option),
		concat!(
			"OptionDescriptor {\n",
			"    name: \"option-name\",\n",
			"    title: \"option-title\",\n",
			"    description: \"option-description\",\n",
			"    value_type: SANE_TYPE_BUTTON,\n",
			"    unit: SANE_UNIT_NONE,\n",
			"    size: 0,\n",
			"    capabilities: Capabilities {\n",
			"        SANE_CAP_EMULATED,\n",
			"    },\n",
			"    constraint: None,\n",
			"}",
		),
	);
}

#[test]
fn group_option_builder() {
	let buf = util::GroupOptionBuilder::new()
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.build();

	let option = buf.option_descriptor();

	assert_eq!(option.name(), CSTR_EMPTY);
	assert_eq!(option.title(), CSTR_OPT_TITLE);
	assert_eq!(option.description(), CSTR_OPT_DESC);
	assert_eq!(option.value_type(), sane::ValueType::GROUP);
	assert_eq!(option.unit(), sane::Unit::NONE);
	assert_eq!(option.size(), 0);

	assert_eq!(
		format!("{:#?}", option),
		concat!(
			"OptionDescriptor {\n",
			"    name: \"\",\n",
			"    title: \"option-title\",\n",
			"    description: \"option-description\",\n",
			"    value_type: SANE_TYPE_GROUP,\n",
			"    unit: SANE_UNIT_NONE,\n",
			"    size: 0,\n",
			"    capabilities: Capabilities {},\n",
			"    constraint: None,\n",
			"}",
		),
	);
}

#[test]
fn util_capabilities() {
	assert!(util::Capabilities::SOFT_SELECT.can_soft_select());
	assert!(util::Capabilities::SOFT_SELECT.can_soft_detect());

	assert!(util::Capabilities::HARD_SELECT.can_hard_select());

	// Interactions between SOFT_SELECT, HARD_SELECT, and SOFT_DETECT
	{
		let mut caps;

		caps = util::Capabilities::SOFT_SELECT;
		assert!(caps.can_soft_detect());
		caps.set_soft_detect(false); // ignored for SOFT_SELECT
		assert!(caps.can_soft_detect());

		caps = util::Capabilities::HARD_SELECT;
		assert!(!caps.can_soft_detect());
		caps.set_soft_detect(true);
		assert!(caps.can_soft_detect());
		caps.set_soft_detect(false);
		assert!(!caps.can_soft_detect());

		caps = util::Capabilities::new();
		assert!(!caps.can_soft_detect());
		caps.set_soft_detect(true);
		assert!(caps.can_soft_detect());
		caps.set_soft_detect(false);
		assert!(!caps.can_soft_detect());
	}

	// Bits are set appropriately
	{
		let mut caps;

		caps = util::Capabilities::new();
		assert!(!caps.is_emulated());
		caps.set_emulated(true);
		assert!(caps.is_emulated());

		caps = util::Capabilities::new();
		assert!(!caps.is_automatic());
		caps.set_automatic(true);
		assert!(caps.is_automatic());

		caps = util::Capabilities::new();
		assert!(caps.is_active());
		caps.set_active(false);
		assert!(!caps.is_active());

		caps = util::Capabilities::new();
		assert!(!caps.is_advanced());
		caps.set_advanced(true);
		assert!(caps.is_advanced());
	}

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
fn constraint_none() {
	let constraint = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::INT,
			sane::ConstraintType::NONE,
			ptr::null()
		).unwrap()
	};
	assert!(matches!(constraint, util::Constraint::None));

	assert_eq!(format!("{:?}", constraint), "None");
}

#[test]
fn constraint_int_range() {
	let mut raw = sane::Range::new();
	raw.min = sane::Int::new(-10).as_word();
	raw.max = sane::Int::new(10).as_word();
	raw.quant = sane::Int::new(1).as_word();

	let constraint = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::INT,
			sane::ConstraintType::RANGE,
			(&raw as *const sane::Range).cast(),
		).unwrap()
	};

	assert!(matches!(constraint, util::Constraint::IntRange(_)));
	let range = match constraint {
		util::Constraint::IntRange(range) => range,
		_ => unreachable!(),
	};
	assert_eq!(range as *const sane::Range, &raw);

	assert_eq!(
		format!("{:#?}", constraint),
		concat!(
			"Range {\n",
			"    min: SANE_Int(-10),\n",
			"    max: SANE_Int(10),\n",
			"    quant: SANE_Int(1),\n",
			"}",
		),
	);
}

#[test]
fn constraint_fixed_range() {
	let mut raw = sane::Range::new();
	raw.min = sane::Fixed::new(-10, 0).as_word();
	raw.max = sane::Fixed::new(10, 0).as_word();
	raw.quant = sane::Fixed::new(0, 32768).as_word();

	let constraint = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::FIXED,
			sane::ConstraintType::RANGE,
			(&raw as *const sane::Range).cast(),
		).unwrap()
	};

	assert!(matches!(constraint, util::Constraint::FixedRange(_)));
	let range = match constraint {
		util::Constraint::FixedRange(range) => range,
		_ => unreachable!(),
	};
	assert_eq!(range as *const sane::Range, &raw);

	assert_eq!(
		format!("{:#?}", constraint),
		concat!(
			"Range {\n",
			"    min: SANE_Fixed(-10.0),\n",
			"    max: SANE_Fixed(10.0),\n",
			"    quant: SANE_Fixed(0.5),\n",
			"}",
		),
	);
}

#[test]
fn constraint_int_list() {
	let raw = [
		sane::Word::new(3),
		sane::Word::new(10),
		sane::Word::new(20),
		sane::Word::new(30),
	];
	let raw_ptr: *const sane::Word = raw.as_ptr();

	let constraint = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::INT,
			sane::ConstraintType::WORD_LIST,
			raw_ptr.cast(),
		).unwrap()
	};

	assert!(matches!(constraint, util::Constraint::IntList(_)));
	let words = match constraint {
		util::Constraint::IntList(words) => words,
		_ => unreachable!(),
	};
	assert_eq!(words.as_ptr(), raw_ptr);

	assert_eq!(
		format!("{:?}", constraint),
		"[SANE_Int(10), SANE_Int(20), SANE_Int(30)]",
	);
}

#[test]
fn constraint_fixed_list() {
	let raw = [
		sane::Word::new(3),
		sane::Fixed::new(-10, 0).as_word(),
		sane::Fixed::new(0, 0).as_word(),
		sane::Fixed::new(10, 0).as_word(),
	];
	let raw_ptr: *const sane::Word = raw.as_ptr();

	let constraint = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::FIXED,
			sane::ConstraintType::WORD_LIST,
			raw_ptr.cast(),
		).unwrap()
	};

	assert!(matches!(constraint, util::Constraint::FixedList(_)));
	let words = match constraint {
		util::Constraint::FixedList(words) => words,
		_ => unreachable!(),
	};
	assert_eq!(words.as_ptr(), raw_ptr);

	assert_eq!(
		format!("{:?}", constraint),
		"[SANE_Fixed(-10.0), SANE_Fixed(0.0), SANE_Fixed(10.0)]",
	);
}

#[test]
fn constraint_string_list() {
	let raw = [
		cstr(b"aaa\x00").as_ptr(),
		cstr(b"bbb\x00").as_ptr(),
		cstr(b"ccc\x00").as_ptr(),
		ptr::null(),
	];
	let raw_ptr: *const sane::StringConst = raw.as_ptr().cast();

	let constraint = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::STRING,
			sane::ConstraintType::STRING_LIST,
			raw_ptr.cast(),
		).unwrap()
	};

	assert!(matches!(constraint, util::Constraint::StringList(_)));
	let strings = match constraint {
		util::Constraint::StringList(strings) => strings,
		_ => unreachable!(),
	};
	assert_eq!(strings.as_ptr(), raw_ptr);

	assert_eq!(
		format!("{:?}", constraint),
		r#"["aaa", "bbb", "ccc"]"#,
	);
}

#[test]
fn constraint_invalid() {
	let err = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::BOOL,
			sane::ConstraintType::RANGE,
			ptr::null()
		).unwrap_err()
	};
	assert!(matches!(err, util::ConstraintError::TypeMismatch(
		sane::ValueType::BOOL,
		sane::ConstraintType::RANGE,
	)));

	let err = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::BOOL,
			sane::ConstraintType::WORD_LIST,
			ptr::null()
		).unwrap_err()
	};
	assert!(matches!(err, util::ConstraintError::TypeMismatch(
		sane::ValueType::BOOL,
		sane::ConstraintType::WORD_LIST,
	)));

	let err = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::BOOL,
			sane::ConstraintType::STRING_LIST,
			ptr::null()
		).unwrap_err()
	};
	assert!(matches!(err, util::ConstraintError::TypeMismatch(
		sane::ValueType::BOOL,
		sane::ConstraintType::STRING_LIST,
	)));

	const INVALID: sane::ConstraintType = unsafe {
		core::mem::transmute(u32::MAX)
	};

	let err = unsafe {
		util::Constraint::from_ptr(
			sane::ValueType::BOOL,
			INVALID,
			ptr::null()
		).unwrap_err()
	};
	assert!(matches!(err, util::ConstraintError::InvalidType(INVALID)));
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
