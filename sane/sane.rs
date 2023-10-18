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

#![no_std]

use core::ffi;
use core::fmt;
use core::mem;
use core::mem::size_of;
use core::ptr;

pub mod util;

type OutPtr<T> = *mut mem::MaybeUninit<T>;

// [4.1] Version Control {{{

/// `SANE_CURRENT_MAJOR`
pub const CURRENT_MAJOR: u8 = 1;

/// `SANE_CURRENT_MINOR`
pub const CURRENT_MINOR: u8 = 0;

/// `SANE_VERSION_CODE`
pub const fn version_code(major: u8, minor: u8, build: u16) -> u32 {
	let major = major as u32;
	let minor = minor as u32;
	let build = build as u32;
	(major << 24) | (minor << 16) | build
}

/// `SANE_VERSION_MAJOR`
pub const fn version_major(version_code: u32) -> u8 {
	((version_code >> 24) & 0xFF) as u8
}

/// `SANE_VERSION_MINOR`
pub const fn version_minor(version_code: u32) -> u8 {
	((version_code >> 16) & 0xFF) as u8
}

/// `SANE_VERSION_BUILD`
pub const fn version_build(version_code: u32) -> u16 {
	(version_code & 0xFFFF) as u16
}

// }}}

// [4.2.1] Base Types {{{

/// `SANE_Word`
#[repr(transparent)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Word(ffi::c_uint);

const _: () = assert!(size_of::<Word>() >= size_of::<u32>());

impl fmt::Debug for Word {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SANE_Word({:?})", self.0)
	}
}

impl From<u32> for Word {
	fn from(value: u32) -> Word {
		Word::new(value)
	}
}

impl Word {
	pub const fn new(value: u32) -> Word {
		Word(value as ffi::c_uint)
	}

	pub const fn as_u32(self) -> u32 {
		self.0 as u32
	}
}

// }}}

// [4.2.2] Boolean Type {{{

/// `SANE_Bool`
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Bool(ffi::c_uint);

const _: () = assert!(size_of::<Bool>() == size_of::<Word>());

impl Bool {
	/// `SANE_FALSE`
	pub const FALSE: Bool = Bool(0);

	/// `SANE_TRUE`
	pub const TRUE: Bool = Bool(1);
}

impl From<bool> for Bool {
	fn from(value: bool) -> Bool {
		Bool::new(value)
	}
}

impl fmt::Debug for Bool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::FALSE => f.write_str("SANE_FALSE"),
			Self::TRUE => f.write_str("SANE_TRUE"),
			_ => write!(f, "SANE_Bool({:#X})", self.0),
		}
	}
}

impl Bool {
	pub const fn new(value: bool) -> Bool {
		if value { Self::TRUE } else { Self::FALSE }
	}

	pub const fn as_word(self) -> Word {
		Word(self.0)
	}
}

// }}}

// [4.2.3] Integer Type {{{

/// `SANE_Int`
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(transparent)]
pub struct Int(ffi::c_int);

const _: () = assert!(size_of::<Int>() >= size_of::<i32>());
const _: () = assert!(size_of::<Int>() == size_of::<Word>());

impl fmt::Debug for Int {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SANE_Int({:?})", self.0)
	}
}

impl From<i32> for Int {
	fn from(value: i32) -> Int {
		Int::new(value)
	}
}

impl Int {
	pub const fn new(value: i32) -> Int {
		Int(value as ffi::c_int)
	}

	pub const fn as_i32(self) -> i32 {
		self.0 as i32
	}

	pub const fn as_word(self) -> Word {
		Word(self.0 as ffi::c_uint)
	}
}

// }}}

// [4.2.4] Fixed-point Type {{{

/// `SANE_Fixed`
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Fixed(ffi::c_int);

const _: () = assert!(size_of::<Fixed>() == size_of::<Word>());

impl fmt::Debug for Fixed {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let x = (self.0 >> Self::SCALE_SHIFT) as i16;
		let y = (self.0 & 0xFFFF) as u16;

		if y == 0 {
			return write!(f, "SANE_Fixed({}.0)", x);
		}

		use fmt::Write;

		let frac_65536: u64 = 152587890625;
		let frac_big_dec = frac_65536 * u64::from(y);
		let mut frac_buf = [0u8; 100];
		let mut bufwriter = crate::util::BufWriter::new(&mut frac_buf);

		write!(&mut bufwriter, "{:016}", frac_big_dec).unwrap();
		let frac_bytes = bufwriter.into_bytes();
		let mut frac_str = core::str::from_utf8(frac_bytes).unwrap();
		assert!(frac_str.len() > 0);
		frac_str = frac_str.trim_end_matches('0');
		write!(f, "SANE_Fixed({}.{})", x, frac_str)
	}
}

impl Fixed {
	const SCALE_SHIFT: usize = 16;
	const SCALE_SHIFT_F64: f64 = (1 << Self::SCALE_SHIFT) as f64;

	pub const fn new(whole: i16, fract_65536s: u16) -> Fixed {
		let x = (whole as u16) as u32;
		let y = fract_65536s as u32;
		Fixed(((x << Self::SCALE_SHIFT) | y) as ffi::c_int)
	}

	pub fn trunc(self) -> f64 {
		((self.0 >> Self::SCALE_SHIFT) as i16) as f64
	}

	pub fn fract(self) -> f64 {
		let frac_65536: f64 = 0.0000152587890625;
		frac_65536 * ((self.0 & 0xFFFF) as u16) as f64
	}

	pub fn from_f64(value: f64) -> Fixed {
		Fixed((value * Self::SCALE_SHIFT_F64) as i32)
	}

	pub fn as_f64(self) -> f64 {
		(self.0 as ffi::c_int as f64) / Self::SCALE_SHIFT_F64
	}

	pub const fn as_word(self) -> Word {
		Word(self.0 as ffi::c_uint)
	}
}

// }}}

// [4.2.5.2] String Type {{{

/// `SANE_String_Const`
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct StringConst(*const ffi::c_char);

impl fmt::Debug for StringConst {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "SANE_String_Const({:#?})", self.0)
		} else {
			write!(f, "SANE_String_Const({:?})", self.0)
		}
	}
}

impl StringConst {
	pub const fn new(ptr: *const ffi::c_char) -> StringConst {
		StringConst(ptr)
	}

	pub const fn null() -> StringConst {
		StringConst(ptr::null())
	}

	pub const fn as_ptr(self) -> *const ffi::c_char {
		self.0
	}

	pub fn is_null(self) -> bool {
		self.0.is_null()
	}

	pub const fn from_c_str(cstr: &ffi::CStr) -> StringConst {
		StringConst(cstr.as_ptr())
	}

	pub unsafe fn to_c_str<'a>(self) -> Option<&'a ffi::CStr> {
		if self.0.is_null() {
			return None;
		}
		Some(ffi::CStr::from_ptr(self.0))
	}
}

// }}}

// [4.2.6] Scanner Handle Type {{{

/// `SANE_Handle`
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Handle(*mut ());

impl fmt::Debug for Handle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "SANE_Handle({:#?})", self.0)
		} else {
			write!(f, "SANE_Handle({:?})", self.0)
		}
	}
}

impl Handle {
	pub fn new(ptr: *mut ()) -> Handle {
		Handle(ptr)
	}

	pub fn as_ptr(self) -> *mut () {
		self.0
	}
}

// }}}

// [4.2.7] Status Type {{{

/// `SANE_Status`
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Status(ffi::c_uint);

const STATUS_STR: [&str; 12] = [
	/* 0 */ "SANE_STATUS_GOOD",
	/* 1 */ "SANE_STATUS_UNSUPPORTED",
	/* 2 */ "SANE_STATUS_CANCELLED",
	/* 3 */ "SANE_STATUS_DEVICE_BUSY",
	/* 4 */ "SANE_STATUS_INVAL",
	/* 5 */ "SANE_STATUS_EOF",
	/* 6 */ "SANE_STATUS_JAMMED",
	/* 7 */ "SANE_STATUS_NO_DOCS",
	/* 8 */ "SANE_STATUS_COVER_OPEN",
	/* 9 */ "SANE_STATUS_IO_ERROR",
	/* 10 */ "SANE_STATUS_NO_MEM",
	/* 11 */ "SANE_STATUS_ACCESS_DENIED",
];

impl Status {
	/// `SANE_STATUS_GOOD`
	pub const GOOD: Status = Status(0);

	/// `SANE_STATUS_UNSUPPORTED`
	pub const UNSUPPORTED: Status = Status(1);

	/// `SANE_STATUS_CANCELLED`
	pub const CANCELLED: Status = Status(2);

	/// `SANE_STATUS_DEVICE_BUSY`
	pub const DEVICE_BUSY: Status = Status(3);

	/// `SANE_STATUS_INVAL`
	pub const INVAL: Status = Status(4);

	/// `SANE_STATUS_EOF`
	pub const EOF: Status = Status(5);

	/// `SANE_STATUS_JAMMED`
	pub const JAMMED: Status = Status(6);

	/// `SANE_STATUS_NO_DOCS`
	pub const NO_DOCS: Status = Status(7);

	/// `SANE_STATUS_COVER_OPEN`
	pub const COVER_OPEN: Status = Status(8);

	/// `SANE_STATUS_IO_ERROR`
	pub const IO_ERROR: Status = Status(9);

	/// `SANE_STATUS_NO_MEM`
	pub const NO_MEM: Status = Status(10);

	/// `SANE_STATUS_ACCESS_DENIED`
	pub const ACCESS_DENIED: Status = Status(11);
}

impl fmt::Debug for Status {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match STATUS_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			None => write!(f, "SANE_Status({:#X})", self.0),
		}
	}
}

// }}}

// [4.2.8] Device Descriptor Type {{{

/// `SANE_Device`
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct Device {
	pub name: StringConst,
	pub vendor: StringConst,
	pub model: StringConst,
	pub r#type: StringConst,
}

impl fmt::Debug for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SANE_Device")
			.field("name", &self.name)
			.field("vendor", &self.vendor)
			.field("model", &self.model)
			.field("type", &self.r#type)
			.finish()
	}
}

impl Device {
	pub fn new() -> Device {
		Device {
			name: StringConst::null(),
			vendor: StringConst::null(),
			model: StringConst::null(),
			r#type: StringConst::null(),
		}
	}
}

// }}}

// [4.2.9] Option Descriptor Type {{{

/// `SANE_Option_Descriptor`
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct OptionDescriptor {
	pub name: StringConst,
	pub title: StringConst,
	pub desc: StringConst,
	pub r#type: ValueType,
	pub unit: Unit,
	pub size: Int,
	pub cap: Int,
	pub constraint_type: ConstraintType,
	pub constraint: *const (),
}

impl fmt::Debug for OptionDescriptor {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SANE_Option_Descriptor")
			.field("name", &self.name)
			.field("title", &self.title)
			.field("desc", &self.desc)
			.field("type", &self.r#type)
			.field("unit", &self.unit)
			.field("size", &self.size)
			.field("cap", &self.cap)
			.field("constraint_type", &self.constraint_type)
			.field("constraint", &self.constraint)
			.finish()
	}
}

impl OptionDescriptor {
	pub fn new() -> OptionDescriptor {
		OptionDescriptor {
			name: StringConst::null(),
			title: StringConst::null(),
			desc: StringConst::null(),
			r#type: ValueType::BOOL,
			unit: Unit::NONE,
			size: Int::new(size_of::<Bool>() as i32),
			cap: Int::new(0),
			constraint_type: ConstraintType::NONE,
			constraint: ptr::null(),
		}
	}
}

// }}}

// [4.2.9.4] Option Value Type {{{

/// `SANE_Value_Type`
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ValueType(ffi::c_uint);

impl ValueType {
	/// `SANE_TYPE_BOOL`
	pub const BOOL: ValueType = ValueType(0);

	/// `SANE_TYPE_INT`
	pub const INT: ValueType = ValueType(1);

	/// `SANE_TYPE_FIXED`
	pub const FIXED: ValueType = ValueType(2);

	/// `SANE_TYPE_STRING`
	pub const STRING: ValueType = ValueType(3);

	/// `SANE_TYPE_BUTTON`
	pub const BUTTON: ValueType = ValueType(4);

	/// `SANE_TYPE_GROUP`
	pub const GROUP: ValueType = ValueType(5);
}

const VALUE_TYPE_STR: [&str; 6] = [
	/* 0 */ "SANE_TYPE_BOOL",
	/* 1 */ "SANE_TYPE_INT",
	/* 2 */ "SANE_TYPE_FIXED",
	/* 3 */ "SANE_TYPE_STRING",
	/* 4 */ "SANE_TYPE_BUTTON",
	/* 5 */ "SANE_TYPE_GROUP",
];

impl fmt::Debug for ValueType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match VALUE_TYPE_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			_ => write!(f, "SANE_Value_Type({:#X})", self.0),
		}
	}
}

// }}}

// [4.2.9.5] Option Value Unit {{{

/// `SANE_Unit`
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Unit(ffi::c_uint);

impl Unit {
	/// `SANE_UNIT_NONE`
	pub const NONE: Unit = Unit(0);

	/// `SANE_UNIT_PIXEL`
	pub const PIXEL: Unit = Unit(1);

	/// `SANE_UNIT_BIT`
	pub const BIT: Unit = Unit(2);

	/// `SANE_UNIT_MM`
	pub const MM: Unit = Unit(3);

	/// `SANE_UNIT_DPI`
	pub const DPI: Unit = Unit(4);

	/// `SANE_UNIT_PERCENT`
	pub const PERCENT: Unit = Unit(5);

	/// `SANE_UNIT_MICROSECOND`
	pub const MICROSECOND: Unit = Unit(6);
}

const UNIT_STR: [&str; 7] = [
	/* 0 */ "SANE_UNIT_NONE",
	/* 1 */ "SANE_UNIT_PIXEL",
	/* 2 */ "SANE_UNIT_BIT",
	/* 3 */ "SANE_UNIT_MM",
	/* 4 */ "SANE_UNIT_DPI",
	/* 5 */ "SANE_UNIT_PERCENT",
	/* 6 */ "SANE_UNIT_MICROSECOND",
];

impl fmt::Debug for Unit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match UNIT_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			_ => write!(f, "SANE_Unit({:#X})", self.0),
		}
	}
}

// }}}

// [4.2.9.7] Option Capabilities {{{

/// `SANE_CAP_SOFT_SELECT`
pub const CAP_SOFT_SELECT: u32 = 1 << 0;

/// `SANE_CAP_HARD_SELECT`
pub const CAP_HARD_SELECT: u32 = 1 << 1;

/// `SANE_CAP_SOFT_DETECT`
pub const CAP_SOFT_DETECT: u32 = 1 << 2;

/// `SANE_CAP_EMULATED`
pub const CAP_EMULATED: u32 = 1 << 3;

/// `SANE_CAP_AUTOMATIC`
pub const CAP_AUTOMATIC: u32 = 1 << 4;

/// `SANE_CAP_INACTIVE`
pub const CAP_INACTIVE: u32 = 1 << 5;

/// `SANE_CAP_ADVANCED`
pub const CAP_ADVANCED: u32 = 1 << 6;

// }}}

// [4.2.9.8] Option Value Constraints {{{

/// `SANE_Constraint_Type`
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ConstraintType(ffi::c_uint);

impl ConstraintType {
	/// `SANE_CONSTRAINT_NONE`
	pub const NONE: ConstraintType = ConstraintType(0);

	/// `SANE_CONSTRAINT_RANGE`
	pub const RANGE: ConstraintType = ConstraintType(1);

	/// `SANE_CONSTRAINT_WORD_LIST`
	pub const WORD_LIST: ConstraintType = ConstraintType(2);

	/// `SANE_CONSTRAINT_STRING_LIST`
	pub const STRING_LIST: ConstraintType = ConstraintType(3);
}

const CONSTRAINT_TYPE_STR: [&str; 4] = [
	/* 0 */ "SANE_CONSTRAINT_NONE",
	/* 1 */ "SANE_CONSTRAINT_RANGE",
	/* 2 */ "SANE_CONSTRAINT_WORD_LIST",
	/* 3 */ "SANE_CONSTRAINT_STRING_LIST",
];

impl fmt::Debug for ConstraintType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match CONSTRAINT_TYPE_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			_ => write!(f, "SANE_Constraint_Type({:#X})", self.0),
		}
	}
}

/// `SANE_Range`
#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct Range {
	pub min: Word,
	pub max: Word,
	pub quant: Word,
}

impl fmt::Debug for Range {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SANE_Range")
			.field("min", &self.min)
			.field("max", &self.max)
			.field("quant", &self.quant)
			.finish()
	}
}

impl Range {
	pub const fn new() -> Range {
		Range {
			min: Word(0),
			max: Word(0),
			quant: Word(0),
		}
	}
}

// }}}

// [4.3.1] sane_init() {{{

/// `sane_init()`
pub type InitFn = unsafe extern "C" fn(
	version_code: OutPtr<Int>,
	authorize: Option<AuthCallback>,
) -> Status;

/// `SANE_MAX_USERNAME_LEN`
pub const MAX_USERNAME_LEN: usize = 128;

/// `SANE_MAX_PASSWORD_LEN`
pub const MAX_PASSWORD_LEN: usize = 128;

/// `SANE_Auth_Callback`
pub type AuthCallback = extern "C" fn(
	resource: StringConst,
	username: *mut mem::MaybeUninit<[ffi::c_char; MAX_USERNAME_LEN]>,
	password: *mut mem::MaybeUninit<[ffi::c_char; MAX_PASSWORD_LEN]>,
);

// }}}

// [4.3.2] sane_exit() {{{

/// `sane_exit()`
pub type ExitFn = unsafe extern "C" fn();

// }}}

// [4.3.3] sane_get_devices() {{{

/// `sane_get_devices()`
pub type GetDevicesFn = unsafe extern "C" fn(
	device_list: OutPtr<*const *const Device>,
	local_only: Bool,
) -> Status;

// }}}

// [4.3.4] sane_open() {{{

/// `sane_open()`
pub type OpenFn = unsafe extern "C" fn(
	device_name: StringConst,
	handle: OutPtr<Handle>,
) -> Status;

// }}}

// [4.3.5] sane_close() {{{

/// `sane_close()`
pub type CloseFn = unsafe extern "C" fn(
	handle: Handle,
);

// }}}

// [4.3.6] sane_get_option_descriptor() {{{

/// `sane_get_option_descriptor()`
pub type GetOptionDescriptorFn = unsafe extern "C" fn(
	handle: Handle,
	option: Int,
) -> *const OptionDescriptor;

// }}}

// [4.3.7] sane_control_option() {{{

/// `sane_control_option()`
pub type ControlOptionFn = unsafe extern "C" fn(
	handle: Handle,
	option: Int,
	action: Action,
	value: *mut mem::MaybeUninit<()>,
	info: OutPtr<Int>,
) -> Status;

/// `SANE_Action`
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Action(ffi::c_uint);

impl Action {
	/// `SANE_ACTION_GET_VALUE`
	pub const GET_VALUE: Action = Action(0);

	/// `SANE_ACTION_SET_VALUE`
	pub const SET_VALUE: Action = Action(1);

	/// `SANE_ACTION_SET_AUTO`
	pub const SET_AUTO: Action = Action(2);
}

const ACTION_STR: [&str; 3] = [
	/* 0 */ "SANE_ACTION_GET_VALUE",
	/* 1 */ "SANE_ACTION_SET_VALUE",
	/* 2 */ "SANE_ACTION_SET_AUTO",
];

impl fmt::Debug for Action {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match ACTION_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			_ => write!(f, "SANE_Action({:#X})", self.0),
		}
	}
}

/// `SANE_INFO_INEXACT`
pub const INFO_INEXACT: u32 = 1 << 0;

/// `SANE_INFO_RELOAD_OPTIONS`
pub const INFO_RELOAD_OPTIONS: u32 = 1 << 1;

/// `SANE_INFO_RELOAD_PARAMS`
pub const INFO_RELOAD_PARAMS: u32 = 1 << 2;

// }}}

// [4.3.8] sane_get_parameters() {{{

/// `sane_get_parameters()`
pub type GetParametersFn = unsafe extern "C" fn(
	handle: Handle,
	params: OutPtr<Parameters>,
) -> Status;

/// `SANE_Parameters`
#[derive(Clone, Copy)]
#[repr(C)]
#[non_exhaustive]
pub struct Parameters {
	pub format: Frame,
	pub last_frame: Bool,
	pub bytes_per_line: Int,
	pub pixels_per_line: Int,
	pub lines: Int,
	pub depth: Int,
}

impl fmt::Debug for Parameters {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SANE_Parameters")
			.field("format", &self.format)
			.field("last_frame", &self.last_frame)
			.field("bytes_per_line", &self.bytes_per_line)
			.field("pixels_per_line", &self.pixels_per_line)
			.field("lines", &self.lines)
			.field("depth", &self.depth)
			.finish()
	}
}

impl Parameters {
	pub fn new() -> Parameters {
		Parameters {
			format: Frame::GRAY,
			last_frame: Bool::FALSE,
			bytes_per_line: Int(0),
			pixels_per_line: Int(0),
			lines: Int(0),
			depth: Int(0),
		}
	}
}

/// `SANE_Frame`
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct Frame(ffi::c_uint);

impl Frame {
	pub const GRAY: Frame = Frame(0);
	pub const RGB: Frame = Frame(1);
	pub const RED: Frame = Frame(2);
	pub const GREEN: Frame = Frame(3);
	pub const BLUE: Frame = Frame(4);
}

const FRAME_STR: [&str; 5] = [
	/* 0 */ "SANE_FRAME_GRAY",
	/* 1 */ "SANE_FRAME_RGB",
	/* 2 */ "SANE_FRAME_RED",
	/* 3 */ "SANE_FRAME_GREEN",
	/* 4 */ "SANE_FRAME_BLUE",
];

impl fmt::Debug for Frame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match FRAME_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			_ => write!(f, "SANE_Frame({:#X})", self.0),
		}
	}
}

// }}}

// [4.3.9] sane_start() {{{

/// `sane_start()`
pub type StartFn = unsafe extern "C" fn(
	handle: Handle,
) -> Status;

// }}}

// [4.3.10] sane_read() {{{

/// `sane_read()`
pub type ReadFn = unsafe extern "C" fn(
	handle: Handle,
	data: OutPtr<u8>,
	max_length: Int,
	length: OutPtr<Int>,
) -> Status;

// }}}

// [4.3.11] sane_cancel() {{{

/// `sane_cancel()`
pub type CancelFn = unsafe extern "C" fn(
	handle: Handle,
);

// }}}

// [4.3.12] sane_set_io_mode() {{{

/// `sane_set_io_mode()`
pub type SetIoModeFn = unsafe extern "C" fn(
	handle: Handle,
	non_blocking: Bool,
) -> Status;

// }}}

// [4.3.13] sane_get_select_fd() {{{

/// `sane_get_select_fd()`
pub type GetSelectFdFn = unsafe extern "C" fn(
	handle: Handle,
	fd: OutPtr<Int>,
) -> Status;

// }}}
