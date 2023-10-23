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

use std::ffi::{
	CStr,
	CString,
};

use sane::{
	Bool,
	Fixed,
	Int,
	Word,
};
use sane::net::{
	ByteOrder,
	ProcedureNumber,
};
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

macro_rules! decode_ok {
	($bytes:expr) => {{
		let mut cursor = std::io::Cursor::new($bytes.to_vec());
		let mut reader = sane::net::io::Codec::BINARY_V3.reader(&mut cursor);

		use sane::net::io::Decode;
		Decode::decode(&mut reader).unwrap()
	}};
}

macro_rules! decode_err {
	($type:ty, $bytes:expr) => {{
		let mut cursor = std::io::Cursor::new($bytes.to_vec());
		let mut reader = sane::net::io::Codec::BINARY_V3.reader(&mut cursor);

		use sane::net::io::Decode;
		<$type>::decode(&mut reader).unwrap_err()
	}};
}

macro_rules! assert_decode_eq {
	($expect_value:expr, $bytes:expr) => {{
		let value = decode_ok!($bytes);
		let expect_value = $expect_value;
		fn unify_types<T>(_: &T, _: &T) {}
		unify_types(&value, &expect_value);
		assert_eq!(value, expect_value);
	}};
}

macro_rules! encode_ok {
	($value:expr) => {{
		let mut bytes = Vec::new();
		let mut cursor = std::io::Cursor::new(&mut bytes);
		let mut writer = sane::net::io::Codec::BINARY_V3.writer(&mut cursor);

		use sane::net::io::Encode;
		$value.encode(&mut writer).unwrap();
		bytes
	}};
}

macro_rules! assert_encode_eq {
	($value:expr, $expect_bytes:expr) => {
		let bytes = encode_ok!($value);
		assert_eq!(bytes, $expect_bytes);
	};
}

// https://github.com/rust-lang/rust/issues/87555
macro_rules! concat_bytes_ {
	($( $chunk:expr ),+ $( , )?) => {{
		struct Chunk<T>(T);
		#[allow(dead_code)]
		impl<const N: usize> Chunk<[u8; N]> {
			const fn get(&self) -> &[u8] { &self.0 }
		}
		#[allow(dead_code)]
		impl<const N: usize> Chunk<&[u8; N]> {
			const fn get(&self) -> &[u8] { self.0 }
		}
		#[allow(dead_code)]
		impl Chunk<&[u8]> {
			const fn get(&self) -> &[u8] { self.0 }
		}
		const fn bytes_len(chunks: &[&[u8]]) -> usize {
			let mut len = 0;
			let mut ii = 0;
			while ii < chunks.len() {
				len += chunks[ii].len();
				ii += 1;
			}
			len
		}
		const fn chunks_concat<const N: usize>(chunks: &[&[u8]]) -> [u8; N] {
			let mut buf = [0u8; N];
			let mut ii = 0;
			let mut buf_idx = 0;
			while ii < chunks.len() {
				let mut jj = 0;
				while jj < chunks[ii].len() {
					buf[buf_idx] = chunks[ii][jj];
					jj += 1;
					buf_idx += 1;
				}
				ii += 1;
			}
			buf
		}
		const CHUNKS: &[&[u8]] = &[$( Chunk($chunk).get() ),+];
		const BYTES_LEN: usize = bytes_len(CHUNKS);
		const BYTES: [u8; BYTES_LEN] = chunks_concat(CHUNKS);
		BYTES
	}};
}

#[test]
fn sane_net_byte_order() {
	assert_eq!(
		format!("{:?}", ByteOrder::LITTLE_ENDIAN),
		"SANE_NET_LITTLE_ENDIAN",
	);
	assert_eq!(
		format!("{:?}", ByteOrder::BIG_ENDIAN),
		"SANE_NET_BIG_ENDIAN",
	);
	assert_eq!(
		format!("{:?}", ByteOrder::from_word(Word::new(0x12345678))),
		"SANE_Net_Byte_Order(0x12345678)",
	);
}

#[test]
fn sane_net_procedure_number() {
	assert_eq!(
		format!("{:?}", ProcedureNumber::INIT),
		"SANE_NET_INIT",
	);
	assert_eq!(
		format!("{:?}", ProcedureNumber::from_word(Word::new(0x12345678))),
		"SANE_Net_Procedure_Number(0x12345678)",
	);
}

#[test]
fn sane_net_handle() {
	assert_decode_eq!(sane::net::Handle(0x12345678), b"\x12\x34\x56\x78");
	assert_encode_eq!(sane::net::Handle(0x12345678), b"\x12\x34\x56\x78");
}

#[test]
fn sane_word() {
	assert_decode_eq!(Word::new(0x12345678), b"\x12\x34\x56\x78");
	assert_encode_eq!(Word::new(0x12345678), b"\x12\x34\x56\x78");
}

#[test]
fn sane_bool() {
	assert_decode_eq!(Bool::FALSE, b"\x00\x00\x00\x00");
	assert_encode_eq!(Bool::FALSE, b"\x00\x00\x00\x00");

	assert_decode_eq!(Bool::TRUE, b"\x00\x00\x00\x01");
	assert_encode_eq!(Bool::TRUE, b"\x00\x00\x00\x01");

	let err = decode_err!(Bool, b"\x00\x00\x00\x02");
	assert!(format!("{:?}", err).contains("InvalidBool"));
}

#[test]
fn sane_int() {
	assert_decode_eq!(Int::new(-1), b"\xFF\xFF\xFF\xFF");
	assert_encode_eq!(Int::new(-1), b"\xFF\xFF\xFF\xFF");

	assert_decode_eq!(Int::new(1), b"\x00\x00\x00\x01");
	assert_encode_eq!(Int::new(1), b"\x00\x00\x00\x01");
}

#[test]
fn sane_fixed() {
	assert_decode_eq!(Fixed::new(-1, 0), b"\xFF\xFF\x00\x00");
	assert_encode_eq!(Fixed::new(-1, 0), b"\xFF\xFF\x00\x00");

	assert_decode_eq!(Fixed::new(1, 0), b"\x00\x01\x00\x00");
	assert_encode_eq!(Fixed::new(1, 0), b"\x00\x01\x00\x00");
}

#[test]
fn sane_enums() {
	assert_decode_eq!(sane::Status::GOOD, b"\x00\x00\x00\x00");
	assert_encode_eq!(sane::Status::GOOD, b"\x00\x00\x00\x00");

	assert_decode_eq!(sane::ValueType::BOOL, b"\x00\x00\x00\x00");
	assert_encode_eq!(sane::ValueType::BOOL, b"\x00\x00\x00\x00");

	assert_decode_eq!(sane::Unit::NONE, b"\x00\x00\x00\x00");
	assert_encode_eq!(sane::Unit::NONE, b"\x00\x00\x00\x00");

	assert_decode_eq!(sane::ConstraintType::NONE, b"\x00\x00\x00\x00");
	assert_encode_eq!(sane::ConstraintType::NONE, b"\x00\x00\x00\x00");

	assert_decode_eq!(sane::Action::GET_VALUE, b"\x00\x00\x00\x00");
	assert_encode_eq!(sane::Action::GET_VALUE, b"\x00\x00\x00\x00");

	assert_decode_eq!(sane::Frame::GRAY, b"\x00\x00\x00\x00");
	assert_encode_eq!(sane::Frame::GRAY, b"\x00\x00\x00\x00");
}

#[test]
fn sane_net_enums() {
	assert_decode_eq!(sane::net::ByteOrder::LITTLE_ENDIAN, b"\x00\x00\x12\x34");
	assert_encode_eq!(sane::net::ByteOrder::LITTLE_ENDIAN, b"\x00\x00\x12\x34");

	assert_decode_eq!(sane::net::ByteOrder::BIG_ENDIAN, b"\x00\x00\x43\x21");
	assert_encode_eq!(sane::net::ByteOrder::BIG_ENDIAN, b"\x00\x00\x43\x21");

	assert_decode_eq!(sane::net::ProcedureNumber::INIT, b"\x00\x00\x00\x00");
	assert_encode_eq!(sane::net::ProcedureNumber::INIT, b"\x00\x00\x00\x00");
}

#[test]
fn strings() {
	fn cstring_empty() -> CString {
		CString::from(CSTR_EMPTY)
	}

	// (char*)(NULL) encodes as len=0
	assert_encode_eq!(Option::<&CStr>::None, b"\x00\x00\x00\x00");

	// len=0 strings can be decoded to an Option (preserving NULL)
	assert_decode_eq!(Option::<CString>::None, b"\x00\x00\x00\x00");

	// len=0 strings can be decoded to a CString (NULL -> "")
	assert_decode_eq!(cstring_empty(), b"\x00\x00\x00\x00");

	// (char*)("") encodes as len=1 data="\x00"
	assert_encode_eq!(Some(CSTR_EMPTY), b"\x00\x00\x00\x01\x00");
	assert_encode_eq!(CSTR_EMPTY, b"\x00\x00\x00\x01\x00");
	assert_encode_eq!(Some(cstring_empty()), b"\x00\x00\x00\x01\x00");
	assert_encode_eq!(cstring_empty(), b"\x00\x00\x00\x01\x00");

	assert_decode_eq!(cstring_empty(), b"\x00\x00\x00\x01\x00");
	assert_decode_eq!(Some(cstring_empty()), b"\x00\x00\x00\x01\x00");

	// (char*)("abc") encodes as len=4 data="abc\x00"
	assert_encode_eq!(cstr(b"abc\x00"), b"\x00\x00\x00\x04abc\x00");

	// missing NUL
	let err = decode_err!(CString, b"\x00\x00\x00\x01a");
	assert!(format!("{:?}", err).contains("InvalidString"));

	// NUL before final byte
	let err = decode_err!(CString, b"\x00\x00\x00\x02\x00\x00");
	assert!(format!("{:?}", err).contains("InvalidString"));
}

#[test]
fn sane_range() {
	let mut range = sane::Range::new();
	range.min = Word::new(0x11111111);
	range.max = Word::new(0x22222222);
	range.quant = Word::new(0x33333333);

	let bytes = encode_ok!(&range);
	assert_eq!(bytes, concat_bytes_!(
		[0x11, 0x11, 0x11, 0x11], // min
		[0x22, 0x22, 0x22, 0x22], // max
		[0x33, 0x33, 0x33, 0x33], // quant
	));

	let decoded: sane::Range = decode_ok!(bytes);
	assert_eq!(range, decoded);
}

#[test]
fn sane_parameters() {
	let mut params = sane::Parameters::new();
	params.format = sane::Frame::BLUE;
	params.last_frame = Bool::TRUE;
	params.bytes_per_line = Int::new(0x11111111);
	params.pixels_per_line = Int::new(0x22222222);
	params.lines = Int::new(0x33333333);
	params.depth = Int::new(0x44444444);

	let bytes = encode_ok!(&params);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 4],             // format
		[0, 0, 0, 1],             // last_frame
		[0x11, 0x11, 0x11, 0x11], // bytes_per_line
		[0x22, 0x22, 0x22, 0x22], // pixels_per_line
		[0x33, 0x33, 0x33, 0x33], // lines
		[0x44, 0x44, 0x44, 0x44], // depth
	));

	let decoded: sane::Parameters = decode_ok!(bytes);
	assert_eq!(params, decoded);
}

#[test]
fn util_device() {
	let mut device_buf = util::DeviceBuf::new(CSTR_DEV_NAME);
	device_buf.set_vendor(CSTR_DEV_VENDOR);
	device_buf.set_model(CSTR_DEV_MODEL);
	device_buf.set_kind(CSTR_DEV_TYPE);

	let bytes = encode_ok!(&device_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"device-name\x00",
		[0, 0, 0, 14],
		b"device-vendor\x00",
		[0, 0, 0, 13],
		b"device-model\x00",
		[0, 0, 0, 12],
		b"device-type\x00",
	));

	let decoded_buf: util::DeviceBuf = decode_ok!(bytes);
	assert_eq!(device_buf.to_device(), decoded_buf.to_device());
}

#[test]
fn util_option_descriptor_bool() {
	let option_buf = util::BoolOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.capabilities(util::Capabilities::SOFT_SELECT)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 13],
		b"option-title\x00",
		[0, 0, 0, 19],
		b"option-description\x00",

		[0, 0, 0, 0],     // ValueType::BOOL
		[0, 0, 0, 0],     // Unit::NONE
		[0, 0, 0, 4],     // size_of::<Bool>()
		[0, 0, 0, 0b101], // CAP_SOFT_SELECT | CAP_SOFT_DETECT
		[0, 0, 0, 0],     // ConstraintType::NONE
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_int() {
	let option_buf = util::IntOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.unit(sane::Unit::PIXEL)
		.capabilities(util::Capabilities::SOFT_SELECT)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 13],
		b"option-title\x00",
		[0, 0, 0, 19],
		b"option-description\x00",

		[0, 0, 0, 1],     // ValueType::INT
		[0, 0, 0, 1],     // Unit::PIXEL
		[0, 0, 0, 4],     // size_of::<Int>()
		[0, 0, 0, 0b101], // CAP_SOFT_SELECT | CAP_SOFT_DETECT
		[0, 0, 0, 0],     // ConstraintType::NONE
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_int_range() {
	let option_buf = util::IntOptionBuilder::new(CSTR_OPT_NAME)
		.range(0x11111111, 0x22222222, 0x33333333)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		[0, 0, 0, 0],
		[0, 0, 0, 4],
		[0, 0, 0, 0],

		[0, 0, 0, 1],             // ConstraintType::RANGE
		[0, 0, 0, 0],             // is_null
		[0x11, 0x11, 0x11, 0x11], // range.min
		[0x22, 0x22, 0x22, 0x22], // range.max
		[0x33, 0x33, 0x33, 0x33], // range.quant
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_int_enum() {
	let values = [
		0x11111111,
		0x22222222,
		0x33333333,
	];
	let option_buf = util::IntOptionBuilder::new(CSTR_OPT_NAME)
		.values(values)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		[0, 0, 0, 0],
		[0, 0, 0, 4],
		[0, 0, 0, 0],

		[0, 0, 0, 2],             // ConstraintType::WORD_LIST
		[0, 0, 0, 4],             // words.len() + 1
		[0, 0, 0, 3],             // words.len()
		[0x11, 0x11, 0x11, 0x11], // words[0]
		[0x22, 0x22, 0x22, 0x22], // words[1]
		[0x33, 0x33, 0x33, 0x33], // words[2]
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_fixed() {
	let option_buf = util::FixedOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.unit(sane::Unit::PIXEL)
		.capabilities(util::Capabilities::SOFT_SELECT)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 13],
		b"option-title\x00",
		[0, 0, 0, 19],
		b"option-description\x00",

		[0, 0, 0, 2],     // ValueType::FIXED
		[0, 0, 0, 1],     // Unit::PIXEL
		[0, 0, 0, 4],     // size_of::<Fixed>()
		[0, 0, 0, 0b101], // CAP_SOFT_SELECT | CAP_SOFT_DETECT
		[0, 0, 0, 0],     // ConstraintType::NONE
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_fixed_range() {
	let range_min = sane::Fixed::new(0x11, 0x22);
	let range_max = sane::Fixed::new(0x33, 0x44);
	let range_quant = sane::Fixed::new(0x55, 0x66);
	let option_buf = util::FixedOptionBuilder::new(CSTR_OPT_NAME)
		.range(range_min, range_max, range_quant)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 2],
		[0, 0, 0, 0],
		[0, 0, 0, 4],
		[0, 0, 0, 0],

		[0, 0, 0, 1],       // ConstraintType::RANGE
		[0, 0, 0, 0],       // is_null
		[0, 0x11, 0, 0x22], // range.min
		[0, 0x33, 0, 0x44], // range.max
		[0, 0x55, 0, 0x66], // range.quant
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_fixed_enum() {
	let values = [
		sane::Fixed::new(0x11, 0x22),
		sane::Fixed::new(0x33, 0x44),
		sane::Fixed::new(0x55, 0x66),
	];
	let option_buf = util::FixedOptionBuilder::new(CSTR_OPT_NAME)
		.values(&values)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 2],
		[0, 0, 0, 0],
		[0, 0, 0, 4],
		[0, 0, 0, 0],

		[0, 0, 0, 2],       // ConstraintType::WORD_LIST
		[0, 0, 0, 4],       // words.len() + 1
		[0, 0, 0, 3],       // words.len()
		[0, 0x11, 0, 0x22], // words[0]
		[0, 0x33, 0, 0x44], // words[1]
		[0, 0x55, 0, 0x66], // words[2]
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_string() {
	let option_buf = util::StringOptionBuilder::new(CSTR_OPT_NAME, 123)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.unit(sane::Unit::PIXEL)
		.capabilities(util::Capabilities::SOFT_SELECT)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 13],
		b"option-title\x00",
		[0, 0, 0, 19],
		b"option-description\x00",

		[0, 0, 0, 3],     // ValueType::STRING
		[0, 0, 0, 1],     // Unit::PIXEL
		[0, 0, 0, 123],   // size=123 (passed to `new()`)
		[0, 0, 0, 0b101], // CAP_SOFT_SELECT | CAP_SOFT_DETECT
		[0, 0, 0, 0],     // ConstraintType::NONE
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_string_enum() {
	let option_buf = util::StringOptionBuilder::new(CSTR_OPT_NAME, 123)
		.values(vec![
			CString::from(cstr(b"aaa\x00")),
			CString::from(cstr(b"bbb\x00")),
			CString::from(cstr(b"ccc\x00")),
		])
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12],
		b"option-name\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 3],
		[0, 0, 0, 0],
		[0, 0, 0, 123],
		[0, 0, 0, 0],

		[0, 0, 0, 3], // ConstraintType::STRING_LIST
		[0, 0, 0, 4], // strings.len() + 1
		[0, 0, 0, 4], // strings[0].len
		b"aaa\x00",
		[0, 0, 0, 4], // strings[1].len
		b"bbb\x00",
		[0, 0, 0, 4], // strings[2].len
		b"ccc\x00",
		[0, 0, 0, 0], // NULL
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_button() {
	let option_buf = util::ButtonOptionBuilder::new(CSTR_OPT_NAME)
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.capabilities(util::Capabilities::SOFT_SELECT)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 12], // CSTR_OPT_NAME.len()
		b"option-name\x00",
		[0, 0, 0, 13], // CSTR_OPT_TITLE.len()
		b"option-title\x00",
		[0, 0, 0, 19], // CSTR_OPT_DESC.len()
		b"option-description\x00",
		[0, 0, 0, 4], // ValueType::BUTTON
		[0, 0, 0, 0], // Unit::NONE
		[0, 0, 0, 0], // size (ignored for ValueType::BUTTON)
		[0, 0, 0, 0b101], // CAP_SOFT_SELECT | CAP_SOFT_DETECT
		[0, 0, 0, 0], // ConstraintType::NONE
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}

#[test]
fn util_option_descriptor_group() {
	let option_buf = util::GroupOptionBuilder::new()
		.title(CSTR_OPT_TITLE)
		.description(CSTR_OPT_DESC)
		.build();

	let bytes = encode_ok!(&option_buf);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 1], // no name for groups
		b"\x00",
		[0, 0, 0, 13], // CSTR_OPT_TITLE.len()
		b"option-title\x00",
		[0, 0, 0, 19], // CSTR_OPT_DESC.len()
		b"option-description\x00",
		[0, 0, 0, 5], // ValueType::GROUP
		[0, 0, 0, 0], // Unit::NONE
		[0, 0, 0, 0], // size (ignored for ValueType::GROUP)
		[0, 0, 0, 0], // capabilities (ignored for ValueType::GROUP)
		[0, 0, 0, 0], // ConstraintType::NONE
	));

	let decoded_buf: util::OptionDescriptorBuf = decode_ok!(bytes);
	assert_eq!(
		option_buf.option_descriptor(),
		decoded_buf.option_descriptor(),
	);
}
