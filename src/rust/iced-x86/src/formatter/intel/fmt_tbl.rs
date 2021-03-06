/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::data_reader::DataReader;
use super::super::super::iced_constants::IcedConstants;
use super::super::pseudo_ops::get_pseudo_ops;
use super::super::strings_tbl::get_strings_table;
use super::enums::*;
use super::fmt_data::FORMATTER_TBL_DATA;
use super::info::*;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::mem;

lazy_static! {
	pub(super) static ref ALL_INFOS: Vec<Box<InstrInfo + Sync + Send>> = { read() };
}

fn add_prefix(s: &str, c: char) -> String {
	let mut res = String::with_capacity(s.len() + 1);
	res.push(c);
	res.push_str(s);
	res
}

fn read() -> Vec<Box<InstrInfo + Sync + Send>> {
	let mut infos: Vec<Box<InstrInfo + Sync + Send>> = Vec::with_capacity(IcedConstants::NUMBER_OF_CODE_VALUES);
	let mut reader = DataReader::new(FORMATTER_TBL_DATA);
	let strings = get_strings_table();
	let mut prev_index = -1isize;
	for i in 0..IcedConstants::NUMBER_OF_CODE_VALUES {
		let f = reader.read_u8();
		let mut ctor_kind: CtorKind = unsafe { mem::transmute((f & 0x7F) as u8) };
		let current_index;
		if ctor_kind == CtorKind::Previous {
			current_index = reader.index() as isize;
			reader.set_index(prev_index as usize);
			ctor_kind = unsafe { mem::transmute((reader.read_u8() & 0x7F) as u8) };
		} else {
			current_index = -1;
			prev_index = reader.index() as isize - 1;
		}
		let s = if (f & 0x80) != 0 {
			add_prefix(&strings[reader.read_compressed_u32() as usize], 'v')
		} else {
			strings[reader.read_compressed_u32() as usize].clone()
		};

		let v;
		let v2;
		let info: Box<InstrInfo + Sync + Send> = match ctor_kind {
			CtorKind::Previous => unreachable!(),
			CtorKind::Normal_1 => Box::new(SimpleInstrInfo::with_mnemonic(s)),

			CtorKind::Normal_2 => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo::new(s, v))
			}

			CtorKind::asz => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_as::new(v, s))
			}

			CtorKind::AX => Box::new(SimpleInstrInfo_AX::new(s)),
			CtorKind::AY => Box::new(SimpleInstrInfo_AY::new(s)),

			CtorKind::bcst => {
				v = reader.read_compressed_u32();
				v2 = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_bcst::new(s, v, v2))
			}

			CtorKind::bnd_1 => Box::new(SimpleInstrInfo_bnd::with_mnemonic(s)),

			CtorKind::bnd_2 => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_bnd::new(s, v))
			}

			CtorKind::DeclareData => Box::new(SimpleInstrInfo_DeclareData::new(unsafe { mem::transmute(i as u16) }, s)),
			CtorKind::fpu_ST_STi => Box::new(SimpleInstrInfo_fpu_ST_STi::new(s)),
			CtorKind::fpu_STi_ST => Box::new(SimpleInstrInfo_fpu_STi_ST::new(s)),
			CtorKind::imul => Box::new(SimpleInstrInfo_imul::new(s)),
			CtorKind::k1 => Box::new(SimpleInstrInfo_k1::new(s)),
			CtorKind::k2 => Box::new(SimpleInstrInfo_k2::new(s)),

			CtorKind::maskmovq => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_maskmovq::new(s, v))
			}

			CtorKind::memsize => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_memsize::new(v, s))
			}

			CtorKind::movabs => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_movabs::new(v, s))
			}

			CtorKind::nop => {
				v = reader.read_compressed_u32();
				v2 = reader.read_u8() as u32;
				Box::new(SimpleInstrInfo_nop::new(v, s, unsafe { mem::transmute(v2 as u8) }))
			}

			CtorKind::nop0F1F => {
				v = reader.read_u8() as u32;
				v2 = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_nop0F1F::new(unsafe { mem::transmute(v as u8) }, s, v2))
			}

			CtorKind::os2 => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_os::with_mnemonic(v, s))
			}

			CtorKind::os3 => {
				v = reader.read_compressed_u32();
				v2 = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_os::new(v, s, v2))
			}

			CtorKind::os_bnd => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_os_bnd::new(v, s))
			}

			CtorKind::os_jcc_2 => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_os_jcc::with_mnemonic(v, s))
			}

			CtorKind::os_jcc_3 => {
				v = reader.read_compressed_u32();
				v2 = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_os_jcc::new(v, s, v2))
			}

			CtorKind::os_loop => {
				v = reader.read_compressed_u32();
				v2 = reader.read_u8() as u32;
				Box::new(SimpleInstrInfo_os_loop::new(v, unsafe { mem::transmute(v2 as u8) }, s))
			}

			CtorKind::os_mem => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_os_mem::new(v, s))
			}

			CtorKind::pclmulqdq => {
				v = reader.read_u8() as u32;
				Box::new(SimpleInstrInfo_pclmulqdq::new(s, get_pseudo_ops(unsafe { mem::transmute(v as u8) })))
			}

			CtorKind::pops => {
				v = reader.read_u8() as u32;
				Box::new(SimpleInstrInfo_pops::new(s, get_pseudo_ops(unsafe { mem::transmute(v as u8) })))
			}

			CtorKind::reg => {
				v = reader.read_u8() as u32;
				Box::new(SimpleInstrInfo_reg::new(s, unsafe { mem::transmute(v as u8) }))
			}

			CtorKind::Reg16 => Box::new(SimpleInstrInfo_Reg16::new(s)),
			CtorKind::ST_STi => Box::new(SimpleInstrInfo_ST_STi::new(s)),

			CtorKind::ST1_2 => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_ST1::with_mnemonic(s, v))
			}

			CtorKind::ST1_3 => {
				v = reader.read_compressed_u32();
				v2 = reader.read_u8() as u32;
				if v2 > 1 {
					panic!();
				}
				Box::new(SimpleInstrInfo_ST1::new(s, v, v2 != 0))
			}

			CtorKind::ST2 => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_ST2::new(s, v))
			}

			CtorKind::STi_ST => Box::new(SimpleInstrInfo_STi_ST::new(s)),

			CtorKind::xbegin => {
				v = reader.read_compressed_u32();
				Box::new(SimpleInstrInfo_xbegin::new(v, s))
			}

			CtorKind::YA => Box::new(SimpleInstrInfo_YA::new(s)),
		};

		infos.push(info);
		if current_index >= 0 {
			reader.set_index(current_index as usize);
		}
	}
	if reader.can_read() {
		panic!();
	}
	infos
}
