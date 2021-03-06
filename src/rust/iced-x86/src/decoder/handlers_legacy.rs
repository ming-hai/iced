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

#![cfg_attr(feature = "cargo-clippy", allow(clippy::useless_let_if_seq))]

use super::enums::*;
use super::handlers::*;
use super::*;
use core::u32;

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_Reg {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_Reg::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, this.reg);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_RegIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_RegIb {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_RegIb::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, this.reg);
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_IbReg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_IbReg {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_IbReg::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register(instruction, this.reg);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_AL_DX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_AL_DX {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_AL_DX::decode, has_modrm: false, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, Register::AL);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register(instruction, Register::DX);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_DX_AL {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_DX_AL {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_DX_AL::decode, has_modrm: false, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, Register::DX);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register(instruction, Register::AL);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Ib {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Ib::decode, has_modrm: false, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ib3 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Ib3 {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Ib3::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_MandatoryPrefix {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) handlers: [&'static OpCodeHandler; 4],
}

impl OpCodeHandler_MandatoryPrefix {
	pub fn new(
		has_modrm: bool, handler: *const OpCodeHandler, handler_66: *const OpCodeHandler, handler_f3: *const OpCodeHandler,
		handler_f2: *const OpCodeHandler,
	) -> Self {
		const_assert_eq!(0, MandatoryPrefixByte::None as u32);
		const_assert_eq!(1, MandatoryPrefixByte::P66 as u32);
		const_assert_eq!(2, MandatoryPrefixByte::PF3 as u32);
		const_assert_eq!(3, MandatoryPrefixByte::PF2 as u32);
		assert!(!is_null_instance_handler(handler));
		assert!(!is_null_instance_handler(handler_66));
		assert!(!is_null_instance_handler(handler_f3));
		assert!(!is_null_instance_handler(handler_f2));
		let handlers = unsafe { [&*handler, &*handler_66, &*handler_f3, &*handler_f2] };
		debug_assert_eq!(has_modrm, handlers[0].has_modrm);
		debug_assert_eq!(has_modrm, handlers[1].has_modrm);
		debug_assert_eq!(has_modrm, handlers[2].has_modrm);
		debug_assert_eq!(has_modrm, handlers[3].has_modrm);
		Self { decode: OpCodeHandler_MandatoryPrefix::decode, has_modrm, handlers: unsafe { [&*handler, &*handler_66, &*handler_f3, &*handler_f2] } }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		decoder.clear_mandatory_prefix(instruction);
		let handler = unsafe { *this.handlers.get_unchecked(decoder.state.mandatory_prefix as usize) };
		(handler.decode)(handler, decoder, instruction);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_MandatoryPrefix3 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) handlers_reg: [(&'static OpCodeHandler, bool); 4],
	pub(crate) handlers_mem: [(&'static OpCodeHandler, bool); 4],
}

impl OpCodeHandler_MandatoryPrefix3 {
	#[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
	pub(crate) fn new(
		handler_reg: *const OpCodeHandler, handler_mem: *const OpCodeHandler, handler66_reg: *const OpCodeHandler,
		handler66_mem: *const OpCodeHandler, handlerf3_reg: *const OpCodeHandler, handlerf3_mem: *const OpCodeHandler,
		handlerf2_reg: *const OpCodeHandler, handlerf2_mem: *const OpCodeHandler, flags: u32,
	) -> Self {
		assert!(!is_null_instance_handler(handler_reg));
		assert!(!is_null_instance_handler(handler_mem));
		assert!(!is_null_instance_handler(handler66_reg));
		assert!(!is_null_instance_handler(handler66_mem));
		assert!(!is_null_instance_handler(handlerf3_reg));
		assert!(!is_null_instance_handler(handlerf3_mem));
		assert!(!is_null_instance_handler(handlerf2_reg));
		assert!(!is_null_instance_handler(handlerf2_mem));
		let handlers_reg = unsafe {
			[
				(&*handler_reg, (flags & LegacyHandlerFlags::HANDLER_REG) == 0),
				(&*handler66_reg, (flags & LegacyHandlerFlags::HANDLER_66_REG) == 0),
				(&*handlerf3_reg, (flags & LegacyHandlerFlags::HANDLER_F3_REG) == 0),
				(&*handlerf2_reg, (flags & LegacyHandlerFlags::HANDLER_F2_REG) == 0),
			]
		};
		let handlers_mem = unsafe {
			[
				(&*handler_mem, (flags & LegacyHandlerFlags::HANDLER_MEM) == 0),
				(&*handler66_mem, (flags & LegacyHandlerFlags::HANDLER_66_MEM) == 0),
				(&*handlerf3_mem, (flags & LegacyHandlerFlags::HANDLER_F3_MEM) == 0),
				(&*handlerf2_mem, (flags & LegacyHandlerFlags::HANDLER_F2_MEM) == 0),
			]
		};
		debug_assert!(handlers_reg[0].0.has_modrm);
		debug_assert!(handlers_reg[1].0.has_modrm);
		debug_assert!(handlers_reg[2].0.has_modrm);
		debug_assert!(handlers_reg[3].0.has_modrm);
		debug_assert!(handlers_mem[0].0.has_modrm);
		debug_assert!(handlers_mem[1].0.has_modrm);
		debug_assert!(handlers_mem[2].0.has_modrm);
		debug_assert!(handlers_mem[3].0.has_modrm);
		Self { decode: OpCodeHandler_MandatoryPrefix3::decode, has_modrm: true, handlers_reg, handlers_mem }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let (handler, mandatory_prefix) = unsafe {
			if decoder.state.mod_ == 3 {
				*this.handlers_reg.get_unchecked(decoder.state.mandatory_prefix as usize)
			} else {
				*this.handlers_mem.get_unchecked(decoder.state.mandatory_prefix as usize)
			}
		};
		if mandatory_prefix {
			decoder.clear_mandatory_prefix(instruction);
		}
		(handler.decode)(handler, decoder, instruction);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_MandatoryPrefix_F3_F2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) handler_normal: &'static OpCodeHandler,
	pub(crate) handler_f3: &'static OpCodeHandler,
	pub(crate) handler_f2: &'static OpCodeHandler,
	pub(crate) clear_f3: bool,
	pub(crate) clear_f2: bool,
}

impl OpCodeHandler_MandatoryPrefix_F3_F2 {
	pub(crate) fn new(
		handler_normal: *const OpCodeHandler, handler_f3: *const OpCodeHandler, clear_f3: bool, handler_f2: *const OpCodeHandler, clear_f2: bool,
	) -> Self {
		assert!(!is_null_instance_handler(handler_normal));
		assert!(!is_null_instance_handler(handler_f3));
		assert!(!is_null_instance_handler(handler_f2));
		Self {
			decode: OpCodeHandler_MandatoryPrefix_F3_F2::decode,
			has_modrm: false,
			handler_normal: unsafe { &*handler_normal },
			handler_f3: unsafe { &*handler_f3 },
			clear_f3,
			handler_f2: unsafe { &*handler_f2 },
			clear_f2,
		}
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let handler;
		let prefix = decoder.state.mandatory_prefix;
		if prefix == MandatoryPrefixByte::PF3 as u32 {
			if this.clear_f3 {
				decoder.clear_mandatory_prefix_f3(instruction);
			}
			handler = this.handler_f3;
		} else if prefix == MandatoryPrefixByte::PF2 as u32 {
			if this.clear_f2 {
				decoder.clear_mandatory_prefix_f2(instruction);
			}
			handler = this.handler_f2;
		} else {
			debug_assert!(prefix == MandatoryPrefixByte::None as u32 || prefix == MandatoryPrefixByte::P66 as u32);
			handler = this.handler_normal;
		}
		if handler.has_modrm {
			decoder.read_modrm();
		}
		(handler.decode)(handler, decoder, instruction);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_NIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_NIb {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_NIb::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_ReservedNop {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) reserved_nop_handler: &'static OpCodeHandler,
	pub(crate) other_handler: &'static OpCodeHandler,
}

impl OpCodeHandler_ReservedNop {
	pub(crate) fn new(reserved_nop_handler: *const OpCodeHandler, other_handler: *const OpCodeHandler) -> Self {
		assert!(!is_null_instance_handler(reserved_nop_handler));
		assert!(!is_null_instance_handler(other_handler));
		Self {
			decode: OpCodeHandler_ReservedNop::decode,
			has_modrm: true,
			reserved_nop_handler: unsafe { &*reserved_nop_handler },
			other_handler: unsafe { &*other_handler },
		}
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let handler = if (decoder.options & DecoderOptions::FORCE_RESERVED_NOP) != 0 { this.reserved_nop_handler } else { this.other_handler };
		(handler.decode)(handler, decoder, instruction);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Iz {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Iz {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Iz::decode, has_modrm: true, code16, code32, code64, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate32);
			instruction.set_immediate32(decoder.read_u32() as u32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate32to64);
			instruction.set_immediate32(decoder.read_u32() as u32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate16);
			super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Ib {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Ib::decode, has_modrm: true, code16, code32, code64, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8to32);
			super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8to64);
			super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8to16);
			super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Ib2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Ib2 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Ib2::decode, has_modrm: true, code16, code32, code64, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_1 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_1 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_1::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, 1);
		decoder.state.flags |= StateFlags::NO_IMM;
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_CL {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_CL {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_CL::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, Register::CL as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Ev::decode, has_modrm: true, code16, code32, code64, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Rv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Rv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Rv::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::AX as u32,
			);
		}
		debug_assert_eq!(3, decoder.state.mod_);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Rv_32_64 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Rv_32_64 {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Rv_32_64::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let base_reg;
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			base_reg = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			base_reg = Register::EAX;
		}
		debug_assert_eq!(3, decoder.state.mod_);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.rm + decoder.state.extra_base_register_base + base_reg as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_REXW {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) disallow_reg: u32,
	pub(crate) disallow_mem: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_REXW {
	pub(crate) fn new(code32: u32, code64: u32, allow_reg: bool, allow_mem: bool) -> Self {
		Self {
			decode: OpCodeHandler_Ev_REXW::decode,
			has_modrm: true,
			code32,
			code64,
			disallow_reg: if allow_reg { 0 } else { u32::MAX },
			disallow_mem: if allow_mem { 0 } else { u32::MAX },
		}
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			if (decoder.state.flags & StateFlags::W) != 0 {
				super::instruction_internal::internal_set_op0_register_u32(
					instruction,
					decoder.state.rm + decoder.state.extra_base_register_base + Register::RAX as u32,
				);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(
					instruction,
					decoder.state.rm + decoder.state.extra_base_register_base + Register::EAX as u32,
				);
			}
			if (this.disallow_reg & decoder.invalid_check_mask) != 0 {
				decoder.set_invalid_instruction();
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.disallow_mem & decoder.invalid_check_mask) != 0 {
				decoder.set_invalid_instruction();
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Evj {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Evj {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Evj::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if (decoder.options & DecoderOptions::AMD_BRANCHES) == 0 || decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
			if decoder.state.mod_ == 3 {
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
				if (decoder.options & DecoderOptions::AMD_BRANCHES) == 0 || decoder.state.operand_size == OpSize::Size32 {
					super::instruction_internal::internal_set_op0_register_u32(
						instruction,
						decoder.state.rm + decoder.state.extra_base_register_base + Register::RAX as u32,
					);
				} else {
					super::instruction_internal::internal_set_op0_register_u32(
						instruction,
						decoder.state.rm + decoder.state.extra_base_register_base + Register::AX as u32,
					);
				}
			} else {
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
				decoder.read_op_mem(instruction);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
			if decoder.state.mod_ == 3 {
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
				if decoder.state.operand_size == OpSize::Size32 {
					super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + Register::EAX as u32);
				} else {
					super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + Register::AX as u32);
				}
			} else {
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
				decoder.read_op_mem(instruction);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ep {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ep {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ep::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Evw {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Evw {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Evw::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ew {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ew {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ew::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ms {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ms {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ms::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		debug_assert_ne!(3, decoder.state.mod_);
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
		decoder.read_op_mem(instruction);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ev {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ev::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_M_as {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_M_as {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_M_as::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gdq_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gdq_Ev {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gdq_Ev::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev3 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ev3 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ev3::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ev2 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ev2::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::EAX as u32);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_R_C {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_R_C {
	pub(crate) fn new(code32: u32, code64: u32, base_reg: Register) -> Self {
		Self { decode: OpCodeHandler_R_C::decode, has_modrm: true, code32, code64, base_reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::EAX as u32,
			);
		}
		// LOCK MOV CR0 is supported by some AMD CPUs
		if this.base_reg == Register::CR0
			&& decoder.state.reg == 0
			&& decoder.state.extra_register_base == 0
			&& instruction.has_lock_prefix()
			&& (decoder.options & DecoderOptions::NO_LOCK_MOV_CR0) == 0
		{
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, Register::CR8 as u32);
			super::instruction_internal::internal_clear_has_lock_prefix(instruction);
			decoder.state.flags &= !StateFlags::LOCK;
		} else {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let reg = decoder.state.reg + decoder.state.extra_register_base;
			if decoder.invalid_check_mask != 0 {
				if this.base_reg == Register::CR0 {
					if reg == 1 || (reg != 8 && reg >= 5) {
						decoder.set_invalid_instruction();
					}
				} else if this.base_reg == Register::DR0 {
					if reg > 7 {
						decoder.set_invalid_instruction();
					}
				} else {
					debug_assert_eq!(Register::TR0, this.base_reg);
				}
			}
			super::instruction_internal::internal_set_op1_register_u32(instruction, reg + this.base_reg as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_C_R {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_C_R {
	pub(crate) fn new(code32: u32, code64: u32, base_reg: Register) -> Self {
		Self { decode: OpCodeHandler_C_R::decode, has_modrm: true, code32, code64, base_reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::EAX as u32,
			);
		}
		// LOCK MOV CR0 is supported by some AMD CPUs
		if this.base_reg == Register::CR0
			&& decoder.state.reg == 0
			&& decoder.state.extra_register_base == 0
			&& instruction.has_lock_prefix()
			&& (decoder.options & DecoderOptions::NO_LOCK_MOV_CR0) == 0
		{
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::CR8);
			super::instruction_internal::internal_clear_has_lock_prefix(instruction);
			decoder.state.flags &= !StateFlags::LOCK;
		} else {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let reg = decoder.state.reg + decoder.state.extra_register_base;
			if decoder.invalid_check_mask != 0 {
				if this.base_reg == Register::CR0 {
					if reg == 1 || (reg != 8 && reg >= 5) {
						decoder.set_invalid_instruction();
					}
				} else if this.base_reg == Register::DR0 {
					if reg > 7 {
						decoder.set_invalid_instruction();
					}
				} else {
					debug_assert_eq!(Register::TR0, this.base_reg);
				}
			}
			super::instruction_internal::internal_set_op0_register_u32(instruction, reg + this.base_reg as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Jb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Jb {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Jb::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		decoder.state.flags |= StateFlags::BRANCH_IMM8;
		if decoder.is64_mode {
			if (decoder.options & DecoderOptions::AMD_BRANCHES) == 0 || decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch64);
				instruction.set_near_branch64((decoder.read_u8() as i8 as u64).wrapping_add(decoder.current_ip64()));
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u8() as i8 as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			}
		} else {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch32);
				instruction.set_near_branch32((decoder.read_u8() as i8 as u32).wrapping_add(decoder.current_ip32()));
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u8() as i8 as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Jx {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Jx {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Jx::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		decoder.state.flags |= StateFlags::XBEGIN;
		if decoder.is64_mode {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch64);
				instruction.set_near_branch64((decoder.read_u32() as i32 as u64).wrapping_add(decoder.current_ip64()));
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch64);
				instruction.set_near_branch64((decoder.read_u32() as i32 as u64).wrapping_add(decoder.current_ip64()));
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch64);
				instruction.set_near_branch64((decoder.read_u16() as i16 as u64).wrapping_add(decoder.current_ip64()));
			}
		} else if decoder.default_code_size == CodeSize::Code32 {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch32);
				instruction.set_near_branch32((decoder.read_u32() as u32).wrapping_add(decoder.current_ip32()));
			} else {
				debug_assert!(decoder.state.operand_size == OpSize::Size16);
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch32);
				instruction.set_near_branch32((decoder.read_u16() as i16 as u32).wrapping_add(decoder.current_ip32()));
			}
		} else {
			debug_assert!(decoder.default_code_size == CodeSize::Code16);
			if decoder.state.operand_size == OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u16() as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			} else {
				debug_assert!(decoder.state.operand_size == OpSize::Size32);
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u32() as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Jz {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Jz {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Jz::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if (decoder.options & DecoderOptions::AMD_BRANCHES) == 0 || decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch64);
				instruction.set_near_branch64((decoder.read_u32() as i32 as u64).wrapping_add(decoder.current_ip64()));
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u16() as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			}
		} else {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch32);
				instruction.set_near_branch32((decoder.read_u32() as u32).wrapping_add(decoder.current_ip32()));
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u16() as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Jb2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16_16: u32,
	pub(crate) code16_32: u32,
	pub(crate) code16_64: u32,
	pub(crate) code32_16: u32,
	pub(crate) code32_32: u32,
	pub(crate) code64_32: u32,
	pub(crate) code64_64: u32,
}

impl OpCodeHandler_Jb2 {
	pub(crate) fn new(code16_16: u32, code16_32: u32, code16_64: u32, code32_16: u32, code32_32: u32, code64_32: u32, code64_64: u32) -> Self {
		Self { decode: OpCodeHandler_Jb2::decode, has_modrm: false, code16_16, code16_32, code16_64, code32_16, code32_32, code64_32, code64_64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		decoder.state.flags |= StateFlags::BRANCH_IMM8;
		if decoder.is64_mode {
			if (decoder.options & DecoderOptions::AMD_BRANCHES) == 0 || decoder.state.operand_size == OpSize::Size32 {
				if decoder.state.address_size == OpSize::Size64 {
					super::instruction_internal::internal_set_code_u32(instruction, this.code64_64);
				} else {
					super::instruction_internal::internal_set_code_u32(instruction, this.code64_32);
				}
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch64);
				instruction.set_near_branch64((decoder.read_u8() as i8 as u64).wrapping_add(decoder.current_ip64()));
			} else {
				if decoder.state.address_size == OpSize::Size64 {
					super::instruction_internal::internal_set_code_u32(instruction, this.code16_64);
				} else {
					super::instruction_internal::internal_set_code_u32(instruction, this.code16_32);
				}
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u8() as i8 as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				if decoder.state.address_size == OpSize::Size32 {
					super::instruction_internal::internal_set_code_u32(instruction, this.code32_32);
				} else {
					super::instruction_internal::internal_set_code_u32(instruction, this.code32_16);
				}
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch32);
				instruction.set_near_branch32((decoder.read_u8() as i8 as u32).wrapping_add(decoder.current_ip32()));
			} else {
				if decoder.state.address_size == OpSize::Size32 {
					super::instruction_internal::internal_set_code_u32(instruction, this.code16_32);
				} else {
					super::instruction_internal::internal_set_code_u32(instruction, this.code16_16);
				}
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
				super::instruction_internal::internal_set_near_branch16(
					instruction,
					(decoder.read_u8() as i8 as u32).wrapping_add(decoder.current_ip32()) as u16 as u32,
				);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Jdisp {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_Jdisp {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_Jdisp::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		debug_assert!(!decoder.is64_mode);
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch32);
			instruction.set_near_branch32(decoder.read_u32() as u32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::NearBranch16);
			super::instruction_internal::internal_set_near_branch16(instruction, decoder.read_u16() as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_PushOpSizeReg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_PushOpSizeReg {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_PushOpSizeReg::decode, has_modrm: false, code16, code32, code64, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, this.reg);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_PushEv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_PushEv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_PushEv::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.is64_mode {
				if decoder.state.operand_size != OpSize::Size16 {
					super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
				} else {
					super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
				}
			} else {
				if decoder.state.operand_size == OpSize::Size32 {
					super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
				} else {
					super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
				}
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Gv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Gv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Gv::decode, has_modrm: true, code16, code32, code64, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Gv_32_64 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Gv_32_64 {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Gv_32_64::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let base_reg;
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			base_reg = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			base_reg = Register::EAX;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + base_reg as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Gv_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Gv_Ib {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Gv_Ib::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Gv_CL {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Gv_CL {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Gv_CL::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register(instruction, Register::CL);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Mp {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Mp {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Mp::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Eb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Eb {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Eb::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			let mut index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ew {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ew {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ew::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::AX as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_PushSimple2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_PushSimple2 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_PushSimple2::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Simple2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Simple2 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Simple2::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Simple2Iw {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Simple2Iw {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Simple2Iw::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate16);
		super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Simple3 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Simple3 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Simple3::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Simple5 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Simple5 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Simple5::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Simple5_ModRM_as {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Simple5_ModRM_as {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Simple5_ModRM_as::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::RAX as u32,
			);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::EAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::AX as u32,
			);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Simple4 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Simple4 {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Simple4::decode, has_modrm: false, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_PushSimpleReg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) index: u32,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_PushSimpleReg {
	pub(crate) fn new(index: u32, code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_PushSimpleReg::decode, has_modrm: false, index, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
				super::instruction_internal::internal_set_op0_register_u32(
					instruction,
					this.index + decoder.state.extra_base_register_base + Register::RAX as u32,
				);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
				super::instruction_internal::internal_set_op0_register_u32(
					instruction,
					this.index + decoder.state.extra_base_register_base + Register::AX as u32,
				);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
				super::instruction_internal::internal_set_op0_register_u32(
					instruction,
					this.index + decoder.state.extra_base_register_base + Register::EAX as u32,
				);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
				super::instruction_internal::internal_set_op0_register_u32(
					instruction,
					this.index + decoder.state.extra_base_register_base + Register::AX as u32,
				);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_SimpleReg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) index: u32,
	pub(crate) code: u32,
}

impl OpCodeHandler_SimpleReg {
	pub(crate) fn new(code: u32, index: u32) -> Self {
		Self { decode: OpCodeHandler_SimpleReg::decode, has_modrm: false, code, index }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		const_assert_eq!(0, OpSize::Size16 as u32);
		const_assert_eq!(1, OpSize::Size32 as u32);
		const_assert_eq!(2, OpSize::Size64 as u32);
		let size_index = decoder.state.operand_size as u32;

		super::instruction_internal::internal_set_code_u32(instruction, size_index + this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		const_assert!(Register::AX as u32 + 16 == Register::EAX as u32);
		const_assert!(Register::AX as u32 + 32 == Register::RAX as u32);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			size_index * 16 + this.index + decoder.state.extra_base_register_base + Register::AX as u32,
		);
	}
}

static XCHG_REG_RAX_CODES: [u16; 3 * 16] = [
	Code::Nopw as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Xchg_r16_AX as u16,
	Code::Nopd as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Xchg_r32_EAX as u16,
	Code::Nopq as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
	Code::Xchg_r64_RAX as u16,
];

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Xchg_Reg_rAX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) index: u32,
}

impl OpCodeHandler_Xchg_Reg_rAX {
	pub(crate) fn new(index: u32) -> Self {
		Self { decode: OpCodeHandler_Xchg_Reg_rAX::decode, has_modrm: false, index }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());

		if this.index == 0 && decoder.state.mandatory_prefix == MandatoryPrefixByte::PF3 as u32 && (decoder.options & DecoderOptions::NO_PAUSE) == 0 {
			decoder.clear_mandatory_prefix_f3(instruction);
			super::instruction_internal::internal_set_code(instruction, Code::Pause);
		} else {
			const_assert_eq!(0, OpSize::Size16 as u32);
			const_assert_eq!(1, OpSize::Size32 as u32);
			const_assert_eq!(2, OpSize::Size64 as u32);
			let size_index = decoder.state.operand_size as u32;
			let code_index = this.index + decoder.state.extra_base_register_base;

			super::instruction_internal::internal_set_code_u32(instruction, unsafe {
				*XCHG_REG_RAX_CODES.get_unchecked((size_index * 16 + code_index) as usize) as u32
			});
			if code_index != 0 {
				const_assert!(Register::AX as u32 + 16 == Register::EAX as u32);
				const_assert!(Register::AX as u32 + 32 == Register::RAX as u32);
				let reg = size_index * 16 + code_index + Register::AX as u32;
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
				super::instruction_internal::internal_set_op0_register_u32(instruction, reg);
				const_assert_eq!(0, OpKind::Register as u32);
				//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
				super::instruction_internal::internal_set_op1_register_u32(instruction, size_index * 16 + Register::AX as u32);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Iz {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Reg_Iz {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Reg_Iz::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::EAX);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate32);
			instruction.set_immediate32(decoder.read_u32() as u32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::RAX);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate32to64);
			instruction.set_immediate32(decoder.read_u32() as u32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::AX);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate16);
			super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
		}
	}
}

static WITH_REX_PREFIX_MOV_REGISTERS: [Register; 16] = [
	Register::AL,
	Register::CL,
	Register::DL,
	Register::BL,
	Register::SPL,
	Register::BPL,
	Register::SIL,
	Register::DIL,
	Register::R8L,
	Register::R9L,
	Register::R10L,
	Register::R11L,
	Register::R12L,
	Register::R13L,
	Register::R14L,
	Register::R15L,
];

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_RegIb3 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) index: u32,
}

impl OpCodeHandler_RegIb3 {
	pub(crate) fn new(index: u32) -> Self {
		Self { decode: OpCodeHandler_RegIb3::decode, has_modrm: false, index }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let register;
		if (decoder.state.flags & StateFlags::HAS_REX) != 0 {
			register = unsafe { *WITH_REX_PREFIX_MOV_REGISTERS.get_unchecked((this.index + decoder.state.extra_base_register_base) as usize) };
		} else {
			register = unsafe { mem::transmute((this.index + Register::AL as u32) as u8) };
		}
		super::instruction_internal::internal_set_code(instruction, Code::Mov_r8_imm8);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, register);
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_RegIz2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) index: u32,
}

impl OpCodeHandler_RegIz2 {
	pub(crate) fn new(index: u32) -> Self {
		Self { decode: OpCodeHandler_RegIz2::decode, has_modrm: false, index }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code(instruction, Code::Mov_r32_imm32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				this.index + decoder.state.extra_base_register_base + Register::EAX as u32,
			);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate32);
			instruction.set_immediate32(decoder.read_u32() as u32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code(instruction, Code::Mov_r64_imm64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				this.index + decoder.state.extra_base_register_base + Register::RAX as u32,
			);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate64);
			super::instruction_internal::internal_set_immediate64_lo(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_immediate64_hi(instruction, decoder.read_u32() as u32);
		} else {
			super::instruction_internal::internal_set_code(instruction, Code::Mov_r16_imm16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				this.index + decoder.state.extra_base_register_base + Register::AX as u32,
			);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate16);
			super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_PushIb2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_PushIb2 {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_PushIb2::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8to64);
				super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8to16);
				super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8to32);
				super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8to16);
				super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_PushIz {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_PushIz {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_PushIz::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate32to64);
				instruction.set_immediate32(decoder.read_u32() as u32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate16);
				super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate32);
				instruction.set_immediate32(decoder.read_u32() as u32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
				super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate16);
				super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ma {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_Gv_Ma {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ma::decode, has_modrm: true, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		debug_assert_ne!(3, decoder.state.mod_);
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
		decoder.read_op_mem(instruction);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_RvMw_Gw {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_RvMw_Gw {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_RvMw_Gw::decode, has_modrm: true, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let base_reg;
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
			base_reg = Register::EAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
			base_reg = Register::AX;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ev_Ib {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ev_Ib::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8to32);
			super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8to64);
			super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		} else {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8to16);
			super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev_Ib_REX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_Gv_Ev_Ib_REX {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ev_Ib_REX::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		debug_assert_eq!(3, decoder.state.mod_);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
		);
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev_32_64 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) disallow_reg: u32,
	pub(crate) disallow_mem: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ev_32_64 {
	pub(crate) fn new(code32: u32, code64: u32, allow_reg: bool, allow_mem: bool) -> Self {
		Self {
			decode: OpCodeHandler_Gv_Ev_32_64::decode,
			has_modrm: true,
			code32,
			code64,
			disallow_mem: if allow_mem { 0 } else { u32::MAX },
			disallow_reg: if allow_reg { 0 } else { u32::MAX },
		}
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let base_reg;
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			base_reg = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			base_reg = Register::EAX;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + base_reg as u32,
			);
			if (this.disallow_reg & decoder.invalid_check_mask) != 0 {
				decoder.set_invalid_instruction();
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.disallow_mem & decoder.invalid_check_mask) != 0 {
				decoder.set_invalid_instruction();
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev_Iz {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ev_Iz {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ev_Iz::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate32);
			instruction.set_immediate32(decoder.read_u32() as u32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate32to64);
			instruction.set_immediate32(decoder.read_u32() as u32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate16);
			super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Yb_Reg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_Yb_Reg {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_Yb_Reg::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESEDI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESDI);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register(instruction, this.reg);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Yv_Reg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Yv_Reg {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Yv_Reg::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESEDI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESDI);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::EAX);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::RAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::AX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Yv_Reg2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_Yv_Reg2 {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_Yv_Reg2::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESEDI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESDI);
		}
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::DX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::DX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Xb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_Reg_Xb {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_Reg_Xb::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, this.reg);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegRSI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegESI);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegSI);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Xv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Reg_Xv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Reg_Xv::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegRSI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegESI);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegSI);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::EAX);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::RAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::AX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Xv2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_Reg_Xv2 {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_Reg_Xv2::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegRSI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegESI);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegSI);
		}
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::DX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::DX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Yb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_Reg_Yb {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_Reg_Yb::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, this.reg);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESEDI);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESDI);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Yv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Reg_Yv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Reg_Yv::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESEDI);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESDI);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::EAX);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::RAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::AX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Yb_Xb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Yb_Xb {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Yb_Xb::decode, has_modrm: false, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESRDI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegRSI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESEDI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegESI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESDI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegSI);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Yv_Xv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Yv_Xv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Yv_Xv::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESRDI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegRSI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESEDI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegESI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemoryESDI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemorySegSI);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Xb_Yb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Xb_Yb {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Xb_Yb::decode, has_modrm: false, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegRSI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegESI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESEDI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegSI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESDI);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Xv_Yv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Xv_Yv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Xv_Yv::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegRSI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegESI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESEDI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegSI);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::MemoryESDI);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Sw {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Sw {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Sw::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.read_op_seg_reg());
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_M {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_M {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_M::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Sw_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Sw_Ev {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Sw_Ev::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		let sreg = decoder.read_op_seg_reg();
		if decoder.invalid_check_mask != 0 && sreg == Register::CS as u32 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_op0_register_u32(instruction, sreg);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			let index = decoder.state.rm + decoder.state.extra_base_register_base;
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::EAX as u32);
			} else if decoder.state.operand_size == OpSize::Size64 {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::RAX as u32);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AX as u32);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ap {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_Ap {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_Ap::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
		}
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::FarBranch32);
			instruction.set_far_branch32(decoder.read_u32() as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::FarBranch16);
			super::instruction_internal::internal_set_far_branch16(instruction, decoder.read_u16() as u32);
		}
		super::instruction_internal::internal_set_far_branch_selector(instruction, decoder.read_u16() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Ob {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_Reg_Ob {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_Reg_Ob::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, this.reg);
		decoder.displ_index = decoder.data_ptr as usize;
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 4);
			decoder.state.flags |= StateFlags::ADDR64;
			super::instruction_internal::internal_set_memory_address64_lo(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_memory_address64_hi(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory64);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 3);
			instruction.set_memory_displacement(decoder.read_u32() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
		} else {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 2);
			instruction.set_memory_displacement(decoder.read_u16() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ob_Reg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) reg: Register,
}

impl OpCodeHandler_Ob_Reg {
	pub(crate) fn new(code: u32, reg: Register) -> Self {
		Self { decode: OpCodeHandler_Ob_Reg::decode, has_modrm: false, code, reg }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		decoder.displ_index = decoder.data_ptr as usize;
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 4);
			decoder.state.flags |= StateFlags::ADDR64;
			super::instruction_internal::internal_set_memory_address64_lo(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_memory_address64_hi(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory64);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 3);
			instruction.set_memory_displacement(decoder.read_u32() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
		} else {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 2);
			instruction.set_memory_displacement(decoder.read_u16() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register(instruction, this.reg);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Ov {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Reg_Ov {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Reg_Ov::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		decoder.displ_index = decoder.data_ptr as usize;
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 4);
			decoder.state.flags |= StateFlags::ADDR64;
			super::instruction_internal::internal_set_memory_address64_lo(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_memory_address64_hi(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory64);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 3);
			instruction.set_memory_displacement(decoder.read_u32() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
		} else {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 2);
			instruction.set_memory_displacement(decoder.read_u16() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::EAX);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::RAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::AX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ov_Reg {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ov_Reg {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ov_Reg::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		decoder.displ_index = decoder.data_ptr as usize;
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 4);
			decoder.state.flags |= StateFlags::ADDR64;
			super::instruction_internal::internal_set_memory_address64_lo(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_memory_address64_hi(instruction, decoder.read_u32() as u32);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory64);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 3);
			instruction.set_memory_displacement(decoder.read_u32() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
		} else {
			super::instruction_internal::internal_set_memory_displ_size(instruction, 2);
			instruction.set_memory_displacement(decoder.read_u16() as u32);
			//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
			//super::instruction_internal::internal_set_memory_base(instruction, Register::None);
			//super::instruction_internal::internal_set_memory_index(instruction, Register::None);
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::EAX);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::RAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::AX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_BranchIw {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_BranchIw {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_BranchIw::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if (decoder.options & DecoderOptions::AMD_BRANCHES) == 0 || decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		}
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate16);
		super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_BranchSimple {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_BranchSimple {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_BranchSimple::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if (decoder.options & DecoderOptions::AMD_BRANCHES) == 0 || decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Iw_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Iw_Ib {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Iw_Ib::decode, has_modrm: false, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.is64_mode {
			if decoder.state.operand_size != OpSize::Size16 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		} else {
			if decoder.state.operand_size == OpSize::Size32 {
				super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			} else {
				super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			}
		}
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate16);
		super::instruction_internal::internal_set_immediate16(instruction, decoder.read_u16() as u32);
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8_2nd);
		super::instruction_internal::internal_set_immediate8_2nd(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Reg_Ib2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_Reg_Ib2 {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_Reg_Ib2::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op0_register(instruction, Register::EAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op0_register(instruction, Register::AX);
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_IbReg2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_IbReg2 {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_IbReg2::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			super::instruction_internal::internal_set_op1_register(instruction, Register::EAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			super::instruction_internal::internal_set_op1_register(instruction, Register::AX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_eAX_DX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_eAX_DX {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_eAX_DX::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::EAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register(instruction, Register::AX);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register(instruction, Register::DX);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_DX_eAX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
}

impl OpCodeHandler_DX_eAX {
	pub(crate) fn new(code16: u32, code32: u32) -> Self {
		Self { decode: OpCodeHandler_DX_eAX::decode, has_modrm: false, code16, code32 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register(instruction, Register::DX);
		if decoder.state.operand_size != OpSize::Size16 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::EAX);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register(instruction, Register::AX);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Eb_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code: u32,
}

impl OpCodeHandler_Eb_Ib {
	pub(crate) fn new(code: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Eb_Ib::decode, has_modrm: true, code, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			let mut index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Eb_1 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Eb_1 {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Eb_1::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			let mut index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, 1);
		decoder.state.flags |= StateFlags::NO_IMM;
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Eb_CL {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Eb_CL {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Eb_CL::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			let mut index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, Register::CL as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Eb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code: u32,
}

impl OpCodeHandler_Eb {
	pub(crate) fn new(code: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Eb::decode, has_modrm: true, code, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			let mut index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Eb_Gb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags: u32,
	pub(crate) code: u32,
}

impl OpCodeHandler_Eb_Gb {
	pub(crate) fn new(code: u32, flags: u32) -> Self {
		Self { decode: OpCodeHandler_Eb_Gb::decode, has_modrm: true, code, flags }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		let mut index;
		if decoder.state.mod_ == 3 {
			index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if (this.flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, this.flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (this.flags & HandlerFlags::LOCK) << (13 - 3);
		}
		index = decoder.state.reg + decoder.state.extra_register_base;
		if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
			index += 4;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AL as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gb_Eb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Gb_Eb {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Gb_Eb::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		let mut index = decoder.state.reg + decoder.state.extra_register_base;
		if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
			index += 4;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, index + Register::AL as u32);

		if decoder.state.mod_ == 3 {
			index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_M {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
}

impl OpCodeHandler_M {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_M::decode, has_modrm: true, code_w0: code, code_w1: code }
	}

	pub(crate) fn new1(code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_M::decode, has_modrm: true, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w1);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w0);
		}
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_M_REXW {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) flags32: u32,
	pub(crate) flags64: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_M_REXW {
	pub(crate) fn new(code32: u32, code64: u32, flags32: u32, flags64: u32) -> Self {
		Self { decode: OpCodeHandler_M_REXW::decode, has_modrm: true, code32, code64, flags32, flags64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			let flags;
			if (decoder.state.flags & StateFlags::W) != 0 {
				flags = this.flags64;
			} else {
				flags = this.flags32;
			}
			decoder.read_op_mem(instruction);
			if (flags & (HandlerFlags::XACQUIRE | HandlerFlags::XRELEASE)) != 0 {
				decoder.set_xacquire_xrelease(instruction, flags);
			}
			const_assert_eq!(8, HandlerFlags::LOCK);
			const_assert_eq!(0x0000_2000, StateFlags::ALLOW_LOCK);
			decoder.state.flags |= (flags & HandlerFlags::LOCK) << (13 - 3);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_MemBx {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_MemBx {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_MemBx::decode, has_modrm: false, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		//instruction.set_memory_displacement(0);
		//super::instruction_internal::internal_set_memory_index_scale(instruction, 0);
		//super::instruction_internal::internal_set_memory_displ_size(instruction, 0);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_memory_base(instruction, Register::RBX);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_memory_base(instruction, Register::EBX);
		} else {
			super::instruction_internal::internal_set_memory_base(instruction, Register::BX);
		}
		super::instruction_internal::internal_set_memory_index(instruction, Register::AL);
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VW {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_r: u32,
	pub(crate) code_m: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VW {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VW::decode, has_modrm: true, base_reg, code_r: code, code_m: code }
	}

	pub(crate) fn new1(base_reg: Register, code_r: u32, code_m: u32) -> Self {
		Self { decode: OpCodeHandler_VW::decode, has_modrm: true, base_reg, code_r, code_m }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_r);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_m);
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
			if this.code_m == Code::INVALID as u32 {
				decoder.set_invalid_instruction();
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_WV {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_WV {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_WV::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_rDI_VX_RX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_rDI_VX_RX {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_rDI_VX_RX::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegEDI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegDI);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_rDI_P_N {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_rDI_P_N {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_rDI_P_N::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.address_size == OpSize::Size64 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegRDI);
		} else if decoder.state.address_size == OpSize::Size32 {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegEDI);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::MemorySegDI);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VM {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VM {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VM::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_MV {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_MV {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_MV::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VQ {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VQ {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VQ::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_P_Q {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_P_Q {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_P_Q::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Q_P {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_Q_P {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_Q_P::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_MP {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_MP {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_MP::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_P_Q_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_P_Q_Ib {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_P_Q_Ib::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_P_W {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_P_W {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_P_W::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_P_R {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_P_R {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_P_R::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_P_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_P_Ev {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_P_Ev::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_P_Ev_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_P_Ev_Ib {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_P_Ev_Ib::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_P {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_P {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_P::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.reg + Register::MM0 as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_W {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_Gv_W {
	pub(crate) fn new(base_reg: Register, code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_W::decode, has_modrm: true, base_reg, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w1);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w0);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_V_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_V_Ev {
	pub(crate) fn new(base_reg: Register, code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_V_Ev::decode, has_modrm: true, base_reg, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if decoder.state.operand_size != OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w0);
			gpr = Register::EAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w1);
			gpr = Register::RAX;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VWIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VWIb {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VWIb::decode, has_modrm: true, base_reg, code_w0: code, code_w1: code }
	}

	pub(crate) fn new1(base_reg: Register, code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_VWIb::decode, has_modrm: true, base_reg, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w1);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w0);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VRIbIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VRIbIb {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VRIbIb::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Immediate8_2nd);
		super::instruction_internal::internal_set_immediate8_2nd(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_RIbIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_RIbIb {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_RIbIb::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8_2nd);
		super::instruction_internal::internal_set_immediate8_2nd(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_RIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_RIb {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_RIb::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ed_V_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_Ed_V_Ib {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ed_V_Ib::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VX_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VX_Ev {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VX_Ev::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + Register::XMM0 as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_VX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_VX {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_VX::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + Register::XMM0 as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VX_E_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VX_E_Ib {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VX_E_Ib::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_RX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_Gv_RX {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_RX::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_B_MIB {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_B_MIB {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_B_MIB::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.reg > 3 || ((decoder.state.extra_register_base & decoder.invalid_check_mask) != 0) {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::BND0 as u32);
		debug_assert_ne!(3, decoder.state.mod_);
		super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
		decoder.read_op_mem_mpx(instruction);
		// It can't be EIP since if it's MPX + 64-bit, the address size is always 64-bit
		if decoder.invalid_check_mask != 0 && instruction.memory_base() == Register::RIP {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_MIB_B {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_MIB_B {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_MIB_B::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.reg > 3 || ((decoder.state.extra_register_base & decoder.invalid_check_mask) != 0) {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		debug_assert_ne!(3, decoder.state.mod_);
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
		decoder.read_op_mem_mpx(instruction);
		// It can't be EIP since if it's MPX + 64-bit, the address size is always 64-bit
		if decoder.invalid_check_mask != 0 && instruction.memory_base() == Register::RIP {
			decoder.set_invalid_instruction();
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.reg + Register::BND0 as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_B_BM {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_B_BM {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_B_BM::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.reg | decoder.state.rm) > 3
			|| (((decoder.state.extra_register_base | decoder.state.extra_base_register_base) & decoder.invalid_check_mask) != 0)
		{
			decoder.set_invalid_instruction();
		}
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::BND0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::BND0 as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem_mpx(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_BM_B {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_BM_B {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_BM_B::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.reg | decoder.state.rm) > 3
			|| (((decoder.state.extra_register_base | decoder.state.extra_base_register_base) & decoder.invalid_check_mask) != 0)
		{
			decoder.set_invalid_instruction();
		}
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + Register::BND0 as u32);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem_mpx(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.reg + Register::BND0 as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_B_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_B_Ev {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_B_Ev::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.reg > 3 || ((decoder.state.extra_register_base & decoder.invalid_check_mask) != 0) {
			decoder.set_invalid_instruction();
		}
		let base_reg;
		if decoder.is64_mode {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			base_reg = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			base_reg = Register::EAX;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::BND0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem_mpx(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Mv_Gv_REXW {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Mv_Gv_REXW {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Mv_Gv_REXW::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_N_Ib_REX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_N_Ib_REX {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_N_Ib_REX::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_N {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_N {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_N::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VN {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VN {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VN::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::MM0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Mv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Mv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Mv::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Mv_Gv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code16: u32,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Mv_Gv {
	pub(crate) fn new(code16: u32, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Mv_Gv::decode, has_modrm: true, code16, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		if decoder.state.operand_size == OpSize::Size32 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		} else if decoder.state.operand_size == OpSize::Size64 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code16);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::AX as u32,
			);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Eb_REX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Eb_REX {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Eb_REX::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			let mut index = decoder.state.rm + decoder.state.extra_base_register_base;
			if (decoder.state.flags & StateFlags::HAS_REX) != 0 && index >= 4 {
				index += 4;
			}
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, index + Register::AL as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Gv_Ev_REX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Gv_Ev_REX {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Gv_Ev_REX::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			if (decoder.state.flags & StateFlags::W) != 0 {
				super::instruction_internal::internal_set_op1_register_u32(
					instruction,
					decoder.state.rm + decoder.state.extra_base_register_base + Register::RAX as u32,
				);
			} else {
				super::instruction_internal::internal_set_op1_register_u32(
					instruction,
					decoder.state.rm + decoder.state.extra_base_register_base + Register::EAX as u32,
				);
			}
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Ev_Gv_REX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_Ev_Gv_REX {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_Ev_Gv_REX::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		debug_assert_ne!(3, decoder.state.mod_);
		super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
		decoder.read_op_mem(instruction);
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::RAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.reg + decoder.state.extra_register_base + Register::EAX as u32,
			);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_GvM_VX_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_GvM_VX_Ib {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_GvM_VX_Ib::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		let gpr;
		if (decoder.state.flags & StateFlags::W) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + gpr as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_Wbinvd {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
}

impl OpCodeHandler_Wbinvd {
	pub(crate) fn new() -> Self {
		Self { decode: OpCodeHandler_Wbinvd::decode, has_modrm: false }
	}

	pub(crate) fn decode(_self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		debug_assert_eq!(EncodingKind::Legacy, decoder.state.encoding());
		if (decoder.options & DecoderOptions::NO_WBNOINVD) != 0 || decoder.state.mandatory_prefix != MandatoryPrefixByte::PF3 as u32 {
			super::instruction_internal::internal_set_code(instruction, Code::Wbinvd);
		} else {
			decoder.clear_mandatory_prefix_f3(instruction);
			super::instruction_internal::internal_set_code(instruction, Code::Wbnoinvd);
		}
	}
}
