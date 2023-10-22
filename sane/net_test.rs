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
use sane::net::io::{
	DecodeError,
};

macro_rules! decode_ok {
	($bytes:expr) => {{
		let mut cursor = std::io::Cursor::new($bytes.to_vec());
		let mut decoder = sane::net::io::Codec::BINARY_V3.decoder(&mut cursor);

		use sane::net::io::Decode;
		Decode::decode(&mut decoder).unwrap()
	}};
}

macro_rules! decode_err {
	($type:ty, $bytes:expr) => {{
		let mut cursor = std::io::Cursor::new($bytes.to_vec());
		let mut decoder = sane::net::io::Codec::BINARY_V3.decoder(&mut cursor);

		use sane::net::io::Decode;
		<$type>::decode(&mut decoder).unwrap_err()
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
		let mut encoder = sane::net::io::Codec::BINARY_V3.encoder(&mut cursor);

		use sane::net::io::Encode;
		$value.encode(&mut encoder).unwrap();
		bytes
	}}
}

macro_rules! assert_encode_eq {
	($value:expr, $expect_bytes:expr) => {
		let bytes = encode_ok!($value);
		assert_eq!(bytes, $expect_bytes);
	};
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
fn sane_word() {
	assert_decode_eq!(Word::new(0x12345678), b"\x12\x34\x56\x78");
	assert_encode_eq!(Word::new(0x12345678), b"\x12\x34\x56\x78");
}

#[test]
fn sane_bool() {
	assert_decode_eq!(Bool::FALSE, b"\x00\x00\x00\x00");
	assert_encode_eq!(Bool::FALSE,  b"\x00\x00\x00\x00");

	assert_decode_eq!(Bool::TRUE, b"\x00\x00\x00\x01");
	assert_encode_eq!(Bool::TRUE, b"\x00\x00\x00\x01");

	let err = decode_err!(Bool, b"\x00\x00\x00\x02");
	assert!(matches!(err, DecodeError::InvalidBool(_)));
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
