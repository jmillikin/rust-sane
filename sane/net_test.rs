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
	self,
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
	($expect_value:expr, $bytes:expr $(,)?) => {{
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
	($value:expr, $expect_bytes:expr $(,)?) => {
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

	// Strings are terminated by an embedded NUL, to match behavior of
	// libsane.so and the existing application ecosystem.
	assert_decode_eq!(
		CString::from(cstr(b"abc\x00")),
		b"\x00\x00\x00\x06abc\x00d\x00",
	);

	// missing NUL
	let err = decode_err!(CString, b"\x00\x00\x00\x01a");
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

	let decoded: util::DeviceBuf = decode_ok!(bytes);
	assert_eq!(device_buf, decoded);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
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
	assert_eq!(option_buf, decoded_buf);
}

#[test]
fn init_request() {
	let mut request_buf = net::InitRequestBuf::new();
	request_buf.set_version_code(0x11223344);
	request_buf.set_username(cstr(b"aaa\x00"));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"InitRequest {\n",
			"    version_code: 287454020,\n",
			"    username: \"aaa\",\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 0],             // SANE_NET_INIT
		[0x11, 0x22, 0x33, 0x44], // version_code
		[0, 0, 0, 4],             // username.len
		b"aaa\x00",
	));

	let decoded: net::InitRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn init_reply() {
	let mut reply_buf = net::InitReplyBuf::new();
	reply_buf.set_status(sane::Status::ACCESS_DENIED);
	reply_buf.set_version_code(0x11223344);
	let reply = reply_buf.as_ref();

	assert_eq!(
		format!("{:#?}", reply),
		concat!(
			"InitReply {\n",
			"    status: SANE_STATUS_ACCESS_DENIED,\n",
			"    version_code: 287454020,\n",
			"}",
		),
	);

	let bytes = encode_ok!(&reply);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 11],            // ACCESS_DENIED
		[0x11, 0x22, 0x33, 0x44], // version_code
	));

	let decoded: net::InitReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn get_devices_request() {
	let request_buf = net::GetDevicesRequestBuf::new();
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		"GetDevicesRequest",
	);

	let bytes = encode_ok!(request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 1], // SANE_NET_GET_DEVICES
	));

	let decoded: net::GetDevicesRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn get_devices_reply() {
	const CSTR_DEV_NAME_2: &CStr = cstr(b"device-name-2\x00");

	let device_1 = {
		let mut dev = util::DeviceBuf::new(CSTR_DEV_NAME);
		dev.set_vendor(CSTR_DEV_VENDOR);
		dev.set_model(CSTR_DEV_MODEL);
		dev.set_kind(CSTR_DEV_TYPE);
		dev
	};
	let device_2 = util::DeviceBuf::new(CSTR_DEV_NAME_2);

	let mut reply_buf = net::GetDevicesReplyBuf::new();
	reply_buf.set_status(sane::Status::ACCESS_DENIED);
	reply_buf.set_devices([device_1, device_2]);
	let reply = reply_buf.as_ref();

	assert_eq!(
		format!("{:#?}", reply),
		concat!(
			"GetDevicesReply {\n",
			"    status: SANE_STATUS_ACCESS_DENIED,\n",
			"    devices: [\n",
			"        Device {\n",
			"            name: \"device-name\",\n",
			"            vendor: \"device-vendor\",\n",
			"            model: \"device-model\",\n",
			"            kind: \"device-type\",\n",
			"        },\n",
			"        Device {\n",
			"            name: \"device-name-2\",\n",
			"            vendor: \"\",\n",
			"            model: \"\",\n",
			"            kind: \"\",\n",
			"        },\n",
			"    ],\n",
			"}",
		),
	);

	let bytes = encode_ok!(reply);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 11], // Status::ACCESS_DENIED

		[0, 0, 0, 3], // device_list.len() + 1

		[0, 0, 0, 0], // device_list[0].is_null()
		[0, 0, 0, 12],
		b"device-name\x00",
		[0, 0, 0, 14],
		b"device-vendor\x00",
		[0, 0, 0, 13],
		b"device-model\x00",
		[0, 0, 0, 12],
		b"device-type\x00",

		[0, 0, 0, 0], // device_list[1].is_null()
		[0, 0, 0, 14],
		b"device-name-2\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		b"\x00",
		[0, 0, 0, 1],
		b"\x00",

		[0, 0, 0, 1], // device_list[2].is_null()
	));

	let decoded: net::GetDevicesReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn open_request() {
	let mut request_buf = net::OpenRequestBuf::new();
	request_buf.set_device_name(CSTR_DEV_NAME);
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"OpenRequest {\n",
			"    device_name: \"device-name\",\n",
			"}",
		),
	);

	let bytes = encode_ok!(request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 2],       // SANE_NET_OPEN
		[0, 0, 0, 12],      // device_name.len
		b"device-name\x00",
	));

	let decoded: net::OpenRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn open_reply() {
	let mut reply_buf = net::OpenReplyBuf::new();
	reply_buf.set_status(sane::Status::ACCESS_DENIED);
	reply_buf.set_handle(net::Handle(0x11223344));
	reply_buf.set_resource(cstr(b"open-resource\x00"));
	let reply = reply_buf.as_ref();

	assert_eq!(
		format!("{:#?}", reply),
		concat!(
			"OpenReply {\n",
			"    status: SANE_STATUS_ACCESS_DENIED,\n",
			"    handle: Handle(287454020),\n",
			"    resource: \"open-resource\",\n",
			"}",
		),
	);

	let bytes = encode_ok!(&reply);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 11], // Status::ACCESS_DENIED
		[0x11, 0x22, 0x33, 0x44], // handle
		[0, 0, 0, 14],
		b"open-resource\x00",
	));

	let decoded: net::OpenReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn close_request() {
	let mut request_buf = net::CloseRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"CloseRequest {\n",
			"    handle: Handle(287454020),\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 3],             // SANE_NET_CLOSE
		[0x11, 0x22, 0x33, 0x44], // handle
	));

	let decoded: net::CloseRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn close_reply() {
	let reply_buf = net::CloseReplyBuf::new();
	let reply = reply_buf.as_ref();

	assert_eq!(format!("{:#?}", reply), "CloseReply");

	let bytes = encode_ok!(reply);
	assert_eq!(bytes, &[0, 0, 0, 0]);

	let decoded: net::CloseReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn get_option_descriptors_request() {
	let mut request_buf = net::GetOptionDescriptorsRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"GetOptionDescriptorsRequest {\n",
			"    handle: Handle(287454020),\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 4],             // SANE_NET_GET_OPTION_DESCRIPTORS
		[0x11, 0x22, 0x33, 0x44], // handle
	));

	let decoded: net::GetOptionDescriptorsRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn get_option_descriptors_reply() {
	const CSTR_OPT_NAME_2: &CStr = cstr(b"option-name-2\x00");

	let mut options = Vec::new();
	options.push({
		util::ButtonOptionBuilder::new(CSTR_OPT_NAME)
			.title(CSTR_OPT_TITLE)
			.description(CSTR_OPT_DESC)
			.build()
	});
	options.push(util::ButtonOptionBuilder::new(CSTR_OPT_NAME_2).build());

	let mut reply_buf = net::GetOptionDescriptorsReplyBuf::new();
	reply_buf.set_option_descriptors(options);
	let reply = reply_buf.as_ref();

	assert_eq!(
		format!("{:#?}", reply),
		concat!(
			"GetOptionDescriptorsReply {\n",
			"    option_descriptors: [\n",
			"        OptionDescriptor {\n",
			"            name: \"option-name\",\n",
			"            title: \"option-title\",\n",
			"            description: \"option-description\",\n",
			"            value_type: SANE_TYPE_BUTTON,\n",
			"            unit: SANE_UNIT_NONE,\n",
			"            size: 0,\n",
			"            capabilities: Capabilities {},\n",
			"            constraint: None,\n",
			"        },\n",
			"        OptionDescriptor {\n",
			"            name: \"option-name-2\",\n",
			"            title: \"\",\n",
			"            description: \"\",\n",
			"            value_type: SANE_TYPE_BUTTON,\n",
			"            unit: SANE_UNIT_NONE,\n",
			"            size: 0,\n",
			"            capabilities: Capabilities {},\n",
			"            constraint: None,\n",
			"        },\n",
			"    ],\n",
			"}",
		),
	);

	let bytes = encode_ok!(reply);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 3],  // options_list.len() + 1

		[0, 0, 0, 0],  // options_list[0].is_null()
		[0, 0, 0, 12], // CSTR_OPT_NAME.len()
		b"option-name\x00",
		[0, 0, 0, 13], // CSTR_OPT_TITLE.len()
		b"option-title\x00",
		[0, 0, 0, 19], // CSTR_OPT_DESC.len()
		b"option-description\x00",
		[0, 0, 0, 4], // ValueType::BUTTON
		[0, 0, 0, 0], // Unit::NONE
		[0, 0, 0, 0], // size (ignored for ValueType::BUTTON)
		[0, 0, 0, 0], // CAP_SOFT_SELECT | CAP_SOFT_DETECT
		[0, 0, 0, 0], // ConstraintType::NONE

		[0, 0, 0, 0],  // options_list[1].is_null()
		[0, 0, 0, 14], // CSTR_OPT_NAME_2.len()
		b"option-name-2\x00",
		[0, 0, 0, 1], // b"\x00".len()
		b"\x00",
		[0, 0, 0, 1], // b"\x00".len()
		b"\x00",
		[0, 0, 0, 4], // ValueType::BUTTON
		[0, 0, 0, 0],
		[0, 0, 0, 0],
		[0, 0, 0, 0],
		[0, 0, 0, 0],

		[0, 0, 0, 1],  // (NULL).is_null()
	));

	let decoded: net::GetOptionDescriptorsReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn control_option_request_set_int() {
	let mut request_buf = net::ControlOptionRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	request_buf.set_option(0x55555555);
	request_buf.set_action(sane::Action::SET_VALUE);
	request_buf.set_value(net::OptionValueBuf::from_i32(0x66778899));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"ControlOptionRequest {\n",
			"    handle: Handle(287454020),\n",
			"    option: 1431655765,\n",
			"    action: SANE_ACTION_SET_VALUE,\n",
			"    value_type: SANE_TYPE_INT,\n",
			"    value: [\n",
			"        102,\n",
			"        119,\n",
			"        136,\n",
			"        153,\n",
			"    ],\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 5],             // SANE_NET_CONTROL_OPTION
		[0x11, 0x22, 0x33, 0x44], // handle
		[0x55, 0x55, 0x55, 0x55], // option
		[0, 0, 0, 1],             // SANE_ACTION_SET_VALUE
		[0, 0, 0, 1],             // value_type: INT
		[0, 0, 0, 4],             // value size
		[0, 0, 0, 1],             // value[-1]: word list length
		[0x66, 0x77, 0x88, 0x99], // value[0]: Int::new(0x66778899)
	));

	let decoded: net::ControlOptionRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn control_option_request_set_string() {
	let mut request_buf = net::ControlOptionRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	request_buf.set_option(0x55555555);
	request_buf.set_action(sane::Action::SET_VALUE);
	request_buf.set_value(net::OptionValueBuf::from_cstring(cstr(b"abcd\x00")));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"ControlOptionRequest {\n",
			"    handle: Handle(287454020),\n",
			"    option: 1431655765,\n",
			"    action: SANE_ACTION_SET_VALUE,\n",
			"    value_type: SANE_TYPE_STRING,\n",
			"    value: [\n",
			"        97,\n",
			"        98,\n",
			"        99,\n",
			"        100,\n",
			"        0,\n",
			"    ],\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 5],             // SANE_NET_CONTROL_OPTION
		[0x11, 0x22, 0x33, 0x44], // handle
		[0x55, 0x55, 0x55, 0x55], // option
		[0, 0, 0, 1],             // SANE_ACTION_SET_VALUE
		[0, 0, 0, 3],             // value_type: STRING
		[0, 0, 0, 5],             // value size
		b"abcd\x00",
	));

	let decoded: net::ControlOptionRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn control_option_request_set_auto() {
	let mut request_buf = net::ControlOptionRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	request_buf.set_option(0x55555555);
	request_buf.set_action(sane::Action::SET_AUTO);
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"ControlOptionRequest {\n",
			"    handle: Handle(287454020),\n",
			"    option: 1431655765,\n",
			"    action: SANE_ACTION_SET_AUTO,\n",
			"    value_type: SANE_TYPE_BOOL,\n",
			"    value: [],\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 5],             // SANE_NET_CONTROL_OPTION
		[0x11, 0x22, 0x33, 0x44], // handle
		[0x55, 0x55, 0x55, 0x55], // option
		[0, 0, 0, 2],             // SANE_ACTION_SET_AUTO
	));

	let decoded: net::ControlOptionRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn control_option_reply() {
	let mut reply_buf = net::ControlOptionReplyBuf::new();
	reply_buf.set_status(sane::Status::ACCESS_DENIED);
	reply_buf.set_info(0x55555555);
	reply_buf.set_value(net::OptionValueBuf::from_i32(0x66778899));
	reply_buf.set_resource(cstr(b"set-value-resource\x00"));
	let reply = reply_buf.as_ref();

	assert_eq!(
		format!("{:#?}", reply),
		concat!(
			"ControlOptionReply {\n",
			"    status: SANE_STATUS_ACCESS_DENIED,\n",
			"    info: 1431655765,\n",
			"    value_type: SANE_TYPE_INT,\n",
			"    value: [\n",
			"        102,\n",
			"        119,\n",
			"        136,\n",
			"        153,\n",
			"    ],\n",
			"    resource: \"set-value-resource\",\n",
			"}",
		),
	);

	let bytes = encode_ok!(&reply);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 11],             // ACCESS_DENIED
		[0x55, 0x55, 0x55, 0x55],  // info

		[0, 0, 0, 1],             // value_type: INT
		[0, 0, 0, 4],             // value size
		[0, 0, 0, 1],             // value[-1]: word list length
		[0x66, 0x77, 0x88, 0x99], // value[0]: Int::new(0x66778899)

		[0, 0, 0, 19],
		b"set-value-resource\x00"
	));

	let decoded: net::ControlOptionReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

fn encode_option_value(value: &net::OptionValueBuf) -> Vec<u8> {
	let mut request_buf = net::ControlOptionRequestBuf::new();
	request_buf.set_value(value.clone());

	let request_bytes = encode_ok!(&request_buf);
	Vec::from(&request_bytes[16..])
}

fn decode_option_value(mut value_bytes: Vec<u8>) -> net::OptionValueBuf {
	let mut request_bytes = vec![0u8; 16];
	request_bytes.append(&mut value_bytes);

	let request_buf: net::ControlOptionRequestBuf = decode_ok!(request_bytes);
	request_buf.value().into()
}

#[test]
fn option_value_bool() {
	let value = net::OptionValueBuf::from_bool(true);
	assert_eq!(value.as_bytes(), &[0, 0, 0, 1]);

	let bytes = encode_option_value(&value);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 0], // BOOL
		[0, 0, 0, 4], // value_size
		[0, 0, 0, 1], // values[-1]: word list length
		[0, 0, 0, 1], // values[0]: TRUE
	));

	let decoded = decode_option_value(bytes);
	assert_eq!(value, decoded);
}

#[test]
fn option_value_int() {
	let value = net::OptionValueBuf::from_i32(0x11223344);
	assert_eq!(value.as_bytes(), &[0x11, 0x22, 0x33, 0x44]);

	let bytes = encode_option_value(&value);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 1], // INT
		[0, 0, 0, 4], // value_size
		[0, 0, 0, 1], // values[-1]: word list length
		[0x11, 0x22, 0x33, 0x44], // values[0]: Int::new(0x11223344)
	));

	let decoded = decode_option_value(bytes);
	assert_eq!(value, decoded);
}

#[test]
fn option_value_int_list() {
	let value = net::OptionValueBuf::from_i32_list(&[
		0x11223344,
		0x55667788,
	]);
	assert_eq!(value.as_bytes(), concat_bytes_!(
		[0x11, 0x22, 0x33, 0x44],
		[0x55, 0x66, 0x77, 0x88],
	));

	let bytes = encode_option_value(&value);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 1], // INT
		[0, 0, 0, 8], // value_size
		[0, 0, 0, 2], // values[-1]: word list length
		[0x11, 0x22, 0x33, 0x44], // values[0]: Int::new(0x11223344)
		[0x55, 0x66, 0x77, 0x88], // values[1]: Int::new(0x55667788)
	));

	let decoded = decode_option_value(bytes);
	assert_eq!(value, decoded);
}

#[test]
fn option_value_fixed() {
	let value = net::OptionValueBuf::from_fixed(Fixed::new(0x1122, 0x3344));
	assert_eq!(value.as_bytes(), &[0x11, 0x22, 0x33, 0x44]);

	let bytes = encode_option_value(&value);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 2], // FIXED
		[0, 0, 0, 4], // value_size
		[0, 0, 0, 1], // values[-1]: word list length
		[0x11, 0x22, 0x33, 0x44], // values[0]: Fixed::new(0x1122, 0x3344)
	));

	let decoded = decode_option_value(bytes);
	assert_eq!(value, decoded);
}

#[test]
fn option_value_fixed_list() {
	let value = net::OptionValueBuf::from_fixed_list(&[
		Fixed::new(0x1122, 0x3344),
		Fixed::new(0x5566, 0x7788),
	]);
	assert_eq!(value.as_bytes(), concat_bytes_!(
		[0x11, 0x22, 0x33, 0x44],
		[0x55, 0x66, 0x77, 0x88],
	));

	let bytes = encode_option_value(&value);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 2], // FIXED
		[0, 0, 0, 8], // value_size
		[0, 0, 0, 2], // values[-1]: word list length
		[0x11, 0x22, 0x33, 0x44], // values[0]: Fixed::new(0x1122, 0x3344)
		[0x55, 0x66, 0x77, 0x88], // values[1]: Fixed::new(0x5566, 0x7788)
	));

	let decoded = decode_option_value(bytes);
	assert_eq!(value, decoded);
}

#[test]
fn option_value_string() {
	let value = net::OptionValueBuf::from_cstring(cstr(b"abcde\x00"));
	assert_eq!(value.as_bytes(), b"abcde\x00");

	let bytes = encode_option_value(&value);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 3], // STRING
		[0, 0, 0, 6], // value_size
		b"abcde\x00",
	));

	let decoded = decode_option_value(bytes);
	assert_eq!(value, decoded);
}

#[test]
#[should_panic]
fn option_value_string_size_too_short() {
	net::OptionValueBuf::from_cstring_with_size(cstr(b"abcde\x00"), 5);
}

#[test]
fn option_value_string_size_extend() {
	let value = net::OptionValueBuf::from_cstring_with_size(cstr(b"a\x00"), 6);
	assert_eq!(value.as_bytes(), b"a\x00\x00\x00\x00\x00");

	let bytes = encode_option_value(&value);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 3], // STRING
		[0, 0, 0, 6], // value_size
		b"a\x00\x00\x00\x00\x00",
	));

	let decoded = decode_option_value(bytes);
	assert_eq!(value, decoded);
}

#[test]
fn get_parameters_request() {
	let mut request_buf = net::GetParametersRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"GetParametersRequest {\n",
			"    handle: Handle(287454020),\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 6],             // SANE_NET_GET_PARAMETERS
		[0x11, 0x22, 0x33, 0x44], // handle
	));

	let decoded: net::GetParametersRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn get_parameters_reply() {
	let mut params = sane::Parameters::new();
	params.format = sane::Frame::BLUE;
	params.last_frame = sane::Bool::TRUE;
	params.bytes_per_line = sane::Int::new(0x11111111);
	params.pixels_per_line = sane::Int::new(0x22222222);
	params.lines = sane::Int::new(0x33333333);
	params.depth = sane::Int::new(0x44444444);

	let mut reply_buf = net::GetParametersReplyBuf::new();
	reply_buf.set_status(sane::Status::ACCESS_DENIED);
	reply_buf.set_parameters(params);
	let reply = reply_buf.as_ref();

	assert_eq!(
		format!("{:#?}", reply),
		concat!(
			"GetParametersReply {\n",
			"    status: SANE_STATUS_ACCESS_DENIED,\n",
			"    parameters: SANE_Parameters {\n",
			"        format: SANE_FRAME_BLUE,\n",
			"        last_frame: SANE_TRUE,\n",
			"        bytes_per_line: SANE_Int(286331153),\n",
			"        pixels_per_line: SANE_Int(572662306),\n",
			"        lines: SANE_Int(858993459),\n",
			"        depth: SANE_Int(1145324612),\n",
			"    },\n",
			"}",
		),
	);

	let bytes = encode_ok!(&reply);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 11],            // Status::ACCESS_DENIED

		[0, 0, 0, 4],             // params.format
		[0, 0, 0, 1],             // params.last_frame
		[0x11, 0x11, 0x11, 0x11], // params.bytes_per_line
		[0x22, 0x22, 0x22, 0x22], // params.pixels_per_line
		[0x33, 0x33, 0x33, 0x33], // params.lines
		[0x44, 0x44, 0x44, 0x44], // params.depth
	));

	let decoded: net::GetParametersReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn start_request() {
	let mut request_buf = net::StartRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"StartRequest {\n",
			"    handle: Handle(287454020),\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 7],             // SANE_NET_START
		[0x11, 0x22, 0x33, 0x44], // handle
	));

	let decoded: net::StartRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn start_reply() {
	let mut reply_buf = net::StartReplyBuf::new();
	reply_buf.set_status(sane::Status::ACCESS_DENIED);
	reply_buf.set_port(0x2233);
	reply_buf.set_byte_order(net::ByteOrder::LITTLE_ENDIAN);
	reply_buf.set_resource(cstr(b"start-resource\x00"));
	let reply = reply_buf.as_ref();

	let bytes = encode_ok!(&reply);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 11],      // Status::ACCESS_DENIED
		[0, 0, 0x22, 0x33], // port
		[0, 0, 0x12, 0x34], // ByteOrder::LITTLE_ENDIAN
		[0, 0, 0, 15],
		b"start-resource\x00",
	));

	let decoded: net::StartReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn cancel_request() {
	let mut request_buf = net::CancelRequestBuf::new();
	request_buf.set_handle(net::Handle(0x11223344));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"CancelRequest {\n",
			"    handle: Handle(287454020),\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 8],             // SANE_NET_CANCEL
		[0x11, 0x22, 0x33, 0x44], // handle
	));

	let decoded: net::CancelRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn cancel_reply() {
	let reply_buf = net::CancelReplyBuf::new();
	let reply = reply_buf.as_ref();

	assert_eq!(format!("{:#?}", reply), "CancelReply");

	let bytes = encode_ok!(reply);
	assert_eq!(bytes, &[0, 0, 0, 0]);

	let decoded: net::CancelReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}

#[test]
fn authorize_request() {
	let mut request_buf = net::AuthorizeRequestBuf::new();
	request_buf.set_resource(cstr(b"auth-resource\x00"));
	request_buf.set_username(cstr(b"auth-username\x00"));
	request_buf.set_password(cstr(b"auth-password\x00"));
	let request = request_buf.as_ref();

	assert_eq!(
		format!("{:#?}", request),
		concat!(
			"AuthorizeRequest {\n",
			"    resource: \"auth-resource\",\n",
			"    username: \"auth-username\",\n",
			"    password: \"auth-password\",\n",
			"}",
		),
	);

	let bytes = encode_ok!(&request);
	assert_eq!(bytes, concat_bytes_!(
		[0, 0, 0, 9], // SANE_NET_AUTHORIZE

		[0, 0, 0, 14], // resource.len
		b"auth-resource\x00",
		[0, 0, 0, 14], // username.len
		b"auth-username\x00",
		[0, 0, 0, 14], // password.len
		b"auth-password\x00",
	));

	let decoded: net::AuthorizeRequestBuf = decode_ok!(bytes);
	assert_eq!(request_buf, decoded);
}

#[test]
fn authorize_reply() {
	let reply_buf = net::AuthorizeReplyBuf::new();
	let reply = reply_buf.as_ref();

	assert_eq!(format!("{:#?}", reply), "AuthorizeReply");

	let bytes = encode_ok!(reply);
	assert_eq!(bytes, &[0, 0, 0, 0]);

	let decoded: net::AuthorizeReplyBuf = decode_ok!(bytes);
	assert_eq!(reply_buf, decoded);
}
