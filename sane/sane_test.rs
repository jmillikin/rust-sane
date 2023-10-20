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

use std::ffi::CStr;
use std::ptr;

use sane::{
	Action,
	Bool,
	ConstraintType,
	Fixed,
	Frame,
	Handle,
	Int,
	Status,
	StringConst,
	Unit,
	ValueType,
	Word,
};

#[test]
fn version_code() {
	assert_eq!(
		sane::version_code(0x12, 0x34, 0x5678),
		0x12345678,
	);

	assert_eq!(sane::version_major(0x12345678), 0x12);
	assert_eq!(sane::version_minor(0x12345678), 0x34);
	assert_eq!(sane::version_build(0x12345678), 0x5678);
}

#[test]
fn sane_word() {
	let word_0 = Word::new(0);
	assert_eq!(word_0.as_u32(), 0u32);
	assert_eq!(format!("{:?}", word_0), "SANE_Word(0)");

	let word_max = Word::new(u32::MAX);
	assert_eq!(word_max.as_u32(), u32::MAX);
	assert_eq!(format!("{:?}", word_max), "SANE_Word(4294967295)");

	assert_eq!(Word::from(u32::MAX), Word::new(u32::MAX));
}

#[test]
fn sane_bool() {
	assert_eq!(Bool::from(false), Bool::FALSE);
	assert_eq!(Bool::from(true), Bool::TRUE);

	assert_eq!(format!("{:?}", Bool::FALSE), "SANE_FALSE");
	assert_eq!(format!("{:?}", Bool::TRUE), "SANE_TRUE");

	assert_eq!(Bool::TRUE.as_word(), Word::new(1));

	let bool_invalid: Bool = unsafe {
		core::mem::transmute(Word::new(0x12345678))
	};
	assert_eq!(format!("{:?}", bool_invalid), "SANE_Bool(0x12345678)");
	assert_eq!(bool_invalid.as_word(), Word::new(0x12345678));
}

#[test]
fn sane_int() {
	assert_eq!(format!("{:?}", Int::new(0)), "SANE_Int(0)");
	assert_eq!(format!("{:?}", Int::new(-1)), "SANE_Int(-1)");

	assert_eq!(Int::from(i32::MAX), Int::new(i32::MAX));
	assert_eq!(Int::from(i32::MAX).as_i32(), i32::MAX);
	assert_eq!(Int::from(i32::MAX).as_word(), Word::new(i32::MAX as u32));
}

#[test]
fn sane_fixed() {
	assert_eq!( 0.0_f64, Fixed::new( 0, 0).as_f64());
	assert_eq!( 1.0_f64, Fixed::new( 1, 0).as_f64());
	assert_eq!(-1.0_f64, Fixed::new(-1, 0).as_f64());

	assert_eq!(Fixed::new(0, 0).trunc(), 0.0);
	assert_eq!(Fixed::new(1, 0).trunc(), 1.0);
	assert_eq!(Fixed::new(-1, 0).trunc(), -1.0);

	assert_eq!(Fixed::new(0, 1).fract(), 0.0000152587890625);
	assert_eq!(Fixed::new(1, 32768).fract(), 0.5);

	assert_eq!(
		Fixed::from_f64(1.0_f64 / 65536_f64),
		Fixed::new(0, 1),
	);
	assert_eq!(
		Fixed::from_f64(1.5_f64),
		Fixed::new(1, 32768),
	);

	assert_eq!(
		1.0_f64 / 65536_f64,
		Fixed::new(0, 1).as_f64(),
	);
	assert_eq!(
		1.5_f64,
		Fixed::new(1, 32768).as_f64(),
	);

	assert_eq!(format!("{:?}", Fixed::new(0, 0)), "SANE_Fixed(0.0)");
	assert_eq!(format!("{:?}", Fixed::new(1, 0)), "SANE_Fixed(1.0)");
	assert_eq!(format!("{:?}", Fixed::new(-1, 0)), "SANE_Fixed(-1.0)");

	assert_eq!(
		format!("{:?}", Fixed::new(0, 1)),
		"SANE_Fixed(0.0000152587890625)", // FIXME: clamp digit count
	);
	assert_eq!(
		format!("{:?}", Fixed::new(0, 1)),
		"SANE_Fixed(0.0000152587890625)", // FIXME: clamp digit count
	);
	assert_eq!(
		format!("{:?}", Fixed::new(1, 32768)),
		"SANE_Fixed(1.5)",
	);

	assert_eq!(
		Fixed::new(1, 32768).as_word(),
		Word::new((1 << 16) | 32768),
	);
}

#[test]
fn sane_string_const() {
	let null = StringConst::null();
	assert!(null.is_null());
	assert_eq!(format!("{:?}", null), "SANE_String_Const(0x0)");
	assert_eq!(unsafe { null.to_c_str() }, None);

	let x: core::ffi::c_char = 0x00;
	let x_ptr: *const core::ffi::c_char = &x;
	let x_str = StringConst::new(x_ptr);
	assert!(!x_str.is_null());
	assert_eq!(
		format!("{:?}", x_str),
		format!("SANE_String_Const({:?})", x_ptr),
	);

	let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(b"a\x00") };
	let cstr_str = StringConst::from_c_str(cstr);
	assert_eq!(cstr_str.as_ptr(), cstr.as_ptr());
	assert_eq!(unsafe { cstr_str.to_c_str() }, Some(cstr));

	assert_eq!(
		StringConst::from_c_str(cstr),
		StringConst::from_c_str(cstr),
	);
}

#[test]
fn sane_handle() {
	let handle_0 = Handle::new(ptr::null_mut());
	assert_eq!(format!("{:?}", handle_0), "SANE_Handle(0x0)");

	let mut x: i32 = 0;
	let x_ptr: *mut () = (&mut x as *mut i32).cast();
	let x_handle = Handle::new(x_ptr);
	assert_eq!(
		format!("{:?}", x_handle),
		format!("SANE_Handle({:?})", x_ptr),
	);

	assert_eq!(
		format!("{:#?}", x_handle),
		format!("SANE_Handle({:#?})", x_ptr),
	);

	assert_eq!(x_handle.as_ptr(), x_ptr);
	assert_eq!(x_handle, Handle::new(x_ptr));
}

#[test]
fn sane_status() {
	assert_eq!(Status::GOOD, Status::GOOD);
	assert_eq!(format!("{:?}", Status::GOOD), "SANE_STATUS_GOOD");

	let status_unknown_word = Word::new(0x12345678);
	let status_unknown = Status::from_word(status_unknown_word);
	assert_eq!(status_unknown.as_word(), status_unknown_word);
	assert_eq!(format!("{:?}", status_unknown), "SANE_Status(0x12345678)");
}

#[test]
fn sane_device() {
	let _ = sane::Device::new().clone();

	let null: *const () = ptr::null();
	assert_eq!(
		format!("{:#?}", sane::Device::new()),
		format!(concat!(
			"SANE_Device {{\n",
			"    name: SANE_String_Const({null_str}),\n",
			"    vendor: SANE_String_Const({null_str}),\n",
			"    model: SANE_String_Const({null_str}),\n",
			"    type: SANE_String_Const({null_str}),\n",
			"}}",
		), null_str = format!("{:#?}", null)),
	);
}

#[test]
fn sane_option_descriptor() {
	let _ = sane::OptionDescriptor::new().clone();

	let null: *const () = ptr::null();
	assert_eq!(
		format!("{:#?}", sane::OptionDescriptor::new()),
		format!(concat!(
			"SANE_Option_Descriptor {{\n",
			"    name: SANE_String_Const({null_str}),\n",
			"    title: SANE_String_Const({null_str}),\n",
			"    desc: SANE_String_Const({null_str}),\n",
			"    type: SANE_TYPE_BOOL,\n",
			"    unit: SANE_UNIT_NONE,\n",
			"    size: SANE_Int(4),\n",
			"    cap: SANE_Int(0),\n",
			"    constraint_type: SANE_CONSTRAINT_NONE,\n",
			"    constraint: {null_str},\n",
			"}}",
		), null_str = format!("{:#?}", null)),
	);
}

#[test]
fn sane_value_type() {
	assert_eq!(ValueType::BOOL, ValueType::BOOL);
	assert_eq!(format!("{:?}", ValueType::BOOL), "SANE_TYPE_BOOL");

	let type_unknown_word = Word::new(0x12345678);
	let type_unknown = ValueType::from_word(type_unknown_word);
	assert_eq!(type_unknown.as_word(), type_unknown_word);
	assert_eq!(format!("{:?}", type_unknown), "SANE_Value_Type(0x12345678)");
}

#[test]
fn sane_unit() {
	assert_eq!(Unit::NONE, Unit::NONE);
	assert_eq!(format!("{:?}", Unit::NONE), "SANE_UNIT_NONE");

	let unit_unknown_word = Word::new(0x12345678);
	let unit_unknown = Unit::from_word(unit_unknown_word);
	assert_eq!(unit_unknown.as_word(), unit_unknown_word);
	assert_eq!(format!("{:?}", unit_unknown), "SANE_Unit(0x12345678)");
}

#[test]
fn sane_constraint_type() {
	assert_eq!(ConstraintType::NONE, ConstraintType::NONE);
	assert_eq!(format!("{:?}", ConstraintType::NONE), "SANE_CONSTRAINT_NONE");

	let type_unknown_word = Word::new(0x12345678);
	let type_unknown = ConstraintType::from_word(type_unknown_word);
	assert_eq!(type_unknown.as_word(), type_unknown_word);
	assert_eq!(
		format!("{:?}", type_unknown),
		"SANE_Constraint_Type(0x12345678)",
	);
}

#[test]
fn sane_range() {
	let mut range = sane::Range::new();
	range.min = Word::new(100);
	range.max = Word::new(200);
	range.quant = Word::new(10);
	assert_eq!(
		format!("{:#?}", range),
		concat!(
			"SANE_Range {\n",
			"    min: SANE_Word(100),\n",
			"    max: SANE_Word(200),\n",
			"    quant: SANE_Word(10),\n",
			"}",
		),
	);

	assert_eq!(range, range.clone());
}

#[test]
fn sane_action() {
	assert_eq!(Action::GET_VALUE, Action::GET_VALUE);
	assert_eq!(format!("{:?}", Action::GET_VALUE), "SANE_ACTION_GET_VALUE");

	let action_unknown_word = Word::new(0x12345678);
	let action_unknown = Action::from_word(action_unknown_word);
	assert_eq!(action_unknown.as_word(), action_unknown_word);
	assert_eq!(format!("{:?}", action_unknown), "SANE_Action(0x12345678)");
}

#[test]
fn sane_parameters() {
	let _ = sane::Parameters::new().clone();

	assert_eq!(
		format!("{:#?}", sane::Parameters::new()),
		concat!(
			"SANE_Parameters {\n",
			"    format: SANE_FRAME_GRAY,\n",
			"    last_frame: SANE_FALSE,\n",
			"    bytes_per_line: SANE_Int(0),\n",
			"    pixels_per_line: SANE_Int(0),\n",
			"    lines: SANE_Int(0),\n",
			"    depth: SANE_Int(0),\n",
			"}",
		),
	);
}

#[test]
fn sane_frame() {
	assert_eq!(Frame::GRAY, Frame::GRAY);
	assert_eq!(format!("{:?}", Frame::GRAY), "SANE_FRAME_GRAY");

	let frame_unknown_word = Word::new(0x12345678);
	let frame_unknown = Frame::from_word(frame_unknown_word);
	assert_eq!(frame_unknown.as_word(), frame_unknown_word);
	assert_eq!(format!("{:?}", frame_unknown), "SANE_Frame(0x12345678)");
}
