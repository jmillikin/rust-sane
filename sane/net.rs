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

use core::fmt;

use crate::Word;

// ByteOrder {{{

/// `SANE_Net_Byte_Order`
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ByteOrder(u32);

impl ByteOrder {
	/// `SANE_NET_LITTLE_ENDIAN`
	pub const LITTLE_ENDIAN: ByteOrder = ByteOrder(0x1234);

	/// `SANE_NET_BIG_ENDIAN`
	pub const BIG_ENDIAN: ByteOrder = ByteOrder(0x4321);

	pub const fn from_word(word: Word) -> ByteOrder {
		ByteOrder(word.as_u32())
	}

	pub const fn as_word(self) -> Word {
		Word::new(self.0)
	}
}

impl fmt::Debug for ByteOrder {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::BIG_ENDIAN => f.write_str("SANE_NET_BIG_ENDIAN"),
			Self::LITTLE_ENDIAN => f.write_str("SANE_NET_LITTLE_ENDIAN"),
			_ => write!(f, "SANE_Net_Byte_Order({:#X})", self.0),
		}
	}
}

// }}}

// ProcedureNumber {{{

/// `SANE_Net_Procedure_Number`
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ProcedureNumber(u32);

const PROCEDURE_NUMBER_STR: [&str; 11] = [
	/* 0 */ "SANE_NET_INIT",
	/* 1 */ "SANE_NET_GET_DEVICES",
	/* 2 */ "SANE_NET_OPEN",
	/* 3 */ "SANE_NET_CLOSE",
	/* 4 */ "SANE_NET_GET_OPTION_DESCRIPTORS",
	/* 5 */ "SANE_NET_CONTROL_OPTION",
	/* 6 */ "SANE_NET_GET_PARAMETERS",
	/* 7 */ "SANE_NET_START",
	/* 8 */ "SANE_NET_CANCEL",
	/* 9 */ "SANE_NET_AUTHORIZE",
	/* 10 */ "SANE_NET_EXIT",
];

impl ProcedureNumber {
	/// `SANE_NET_INIT`
	pub const INIT: ProcedureNumber = ProcedureNumber(0);

	/// `SANE_NET_GET_DEVICES`
	pub const GET_DEVICES: ProcedureNumber = ProcedureNumber(1);

	/// `SANE_NET_OPEN`
	pub const OPEN: ProcedureNumber = ProcedureNumber(2);

	/// `SANE_NET_CLOSE`
	pub const CLOSE: ProcedureNumber = ProcedureNumber(3);

	/// `SANE_NET_GET_OPTION_DESCRIPTORS`
	pub const GET_OPTION_DESCRIPTORS: ProcedureNumber = ProcedureNumber(4);

	/// `SANE_NET_CONTROL_OPTION`
	pub const CONTROL_OPTION: ProcedureNumber = ProcedureNumber(5);

	/// `SANE_NET_GET_PARAMETERS`
	pub const GET_PARAMETERS: ProcedureNumber = ProcedureNumber(6);

	/// `SANE_NET_START`
	pub const START: ProcedureNumber = ProcedureNumber(7);

	/// `SANE_NET_CANCEL`
	pub const CANCEL: ProcedureNumber = ProcedureNumber(8);

	/// `SANE_NET_AUTHORIZE`
	pub const AUTHORIZE: ProcedureNumber = ProcedureNumber(9);

	/// `SANE_NET_EXIT`
	pub const EXIT: ProcedureNumber = ProcedureNumber(10);

	pub const fn from_word(word: Word) -> ProcedureNumber {
		ProcedureNumber(word.as_u32())
	}

	pub const fn as_word(self) -> Word {
		Word::new(self.0)
	}
}

impl fmt::Debug for ProcedureNumber {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match PROCEDURE_NUMBER_STR.get(self.0 as usize) {
			Some(s) => f.write_str(s),
			None => write!(f, "SANE_Net_Procedure_Number({:#X})", self.0),
		}
	}
}

// }}}
