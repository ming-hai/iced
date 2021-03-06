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

use super::super::*;
use super::handlers::*;
use super::*;

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VectorLength_VEX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) handlers: [&'static OpCodeHandler; 4],
}

impl OpCodeHandler_VectorLength_VEX {
	#[allow(trivial_casts)]
	pub(crate) fn new(has_modrm: bool, handler128: *const OpCodeHandler, handler256: *const OpCodeHandler) -> Self {
		const_assert_eq!(0, VectorLength::L128 as u32);
		const_assert_eq!(1, VectorLength::L256 as u32);
		const_assert_eq!(2, VectorLength::L512 as u32);
		const_assert_eq!(3, VectorLength::Unknown as u32);
		assert!(!is_null_instance_handler(handler128));
		assert!(!is_null_instance_handler(handler256));
		let handlers = unsafe {
			[
				&*handler128,
				&*handler256,
				&*(&INVALID_HANDLER as *const _ as *const OpCodeHandler),
				&*(&INVALID_HANDLER as *const _ as *const OpCodeHandler),
			]
		};
		assert_eq!(has_modrm, handlers[0].has_modrm);
		assert_eq!(has_modrm, handlers[1].has_modrm);
		Self { decode: OpCodeHandler_VectorLength_VEX::decode, has_modrm, handlers }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		let handler = unsafe { *this.handlers.get_unchecked(decoder.state.vector_length as usize) };
		(handler.decode)(handler, decoder, instruction);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX2 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) handler_mem: &'static OpCodeHandler,
}

impl OpCodeHandler_VEX2 {
	pub(crate) fn new(handler_mem: *const OpCodeHandler) -> Self {
		assert!(!is_null_instance_handler(handler_mem));
		Self { decode: OpCodeHandler_VEX2::decode, has_modrm: true, handler_mem: unsafe { &*handler_mem } }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		if decoder.state.mod_ == 3 || decoder.is64_mode {
			decoder.vex2(instruction);
		} else {
			let handler = this.handler_mem;
			(handler.decode)(handler, decoder, instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX3 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) handler_mem: &'static OpCodeHandler,
}

impl OpCodeHandler_VEX3 {
	pub(crate) fn new(handler_mem: *const OpCodeHandler) -> Self {
		assert!(!is_null_instance_handler(handler_mem));
		Self { decode: OpCodeHandler_VEX3::decode, has_modrm: true, handler_mem: unsafe { &*handler_mem } }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		if decoder.state.mod_ == 3 || decoder.is64_mode {
			decoder.vex3(instruction);
		} else {
			let handler = this.handler_mem;
			(handler.decode)(handler, decoder, instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_XOP {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) handler_reg0: &'static OpCodeHandler,
}

impl OpCodeHandler_XOP {
	pub(crate) fn new(handler_reg0: *const OpCodeHandler) -> Self {
		assert!(!is_null_instance_handler(handler_reg0));
		Self { decode: OpCodeHandler_XOP::decode, has_modrm: true, handler_reg0: unsafe { &*handler_reg0 } }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		if (decoder.state.modrm & 0x1F) < 8 {
			let handler = this.handler_reg0;
			(handler.decode)(handler, decoder, instruction);
		} else {
			decoder.xop(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Simple {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_VEX_Simple {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Simple::decode, has_modrm: false, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHEv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VHEv {
	pub(crate) fn new(base_reg: Register, code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHEv::decode, has_modrm: true, base_reg, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w1);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w0);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
		} else {
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHEvIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VHEvIb {
	pub(crate) fn new(base_reg: Register, code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHEvIb::decode, has_modrm: true, base_reg, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w1);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w0);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
		} else {
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VW {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg1: Register,
	pub(crate) base_reg2: Register,
}

impl OpCodeHandler_VEX_VW {
	pub(crate) fn new(base_reg1: Register, base_reg2: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VW::decode, has_modrm: true, base_reg1, base_reg2, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg1 as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg2 as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VX_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_VX_Ev {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VX_Ev::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
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
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Ev_VX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_Ev_VX {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Ev_VX::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
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
pub(crate) struct OpCodeHandler_VEX_WV {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg1: Register,
	pub(crate) base_reg2: Register,
}

impl OpCodeHandler_VEX_WV {
	pub(crate) fn new(reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_WV::decode, has_modrm: true, base_reg1: reg, base_reg2: reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg2 as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg1 as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VM {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VM {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VM::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
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
pub(crate) struct OpCodeHandler_VEX_MV {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_MV {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_MV::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
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
pub(crate) struct OpCodeHandler_VEX_M {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_VEX_M {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_M::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
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
pub(crate) struct OpCodeHandler_VEX_RdRq {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_RdRq {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_RdRq::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
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
		if decoder.state.mod_ != 3 {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_rDI_VX_RX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_rDI_VX_RX {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_rDI_VX_RX::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
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
pub(crate) struct OpCodeHandler_VEX_VWIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
	pub(crate) base_reg1: Register,
	pub(crate) base_reg2: Register,
}

impl OpCodeHandler_VEX_VWIb {
	pub(crate) fn new(base_reg1: Register, base_reg2: Register, code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VWIb::decode, has_modrm: true, base_reg1, base_reg2, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w1);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_w0);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg1 as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg2 as u32,
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
pub(crate) struct OpCodeHandler_VEX_WVIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg1: Register,
	pub(crate) base_reg2: Register,
}

impl OpCodeHandler_VEX_WVIb {
	pub(crate) fn new(base_reg1: Register, base_reg2: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_WVIb::decode, has_modrm: true, base_reg1, base_reg2, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg1 as u32,
			);
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg2 as u32,
		);
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Ed_V_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_Ed_V_Ib {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Ed_V_Ib::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
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
pub(crate) struct OpCodeHandler_VEX_VHW {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_r: u32,
	pub(crate) code_m: u32,
	pub(crate) base_reg1: Register,
	pub(crate) base_reg2: Register,
	pub(crate) base_reg3: Register,
}

impl OpCodeHandler_VEX_VHW {
	pub(crate) fn new(base_reg1: Register, base_reg2: Register, base_reg3: Register, code_r: u32, code_m: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHW::decode, has_modrm: true, base_reg1, base_reg2, base_reg3, code_r, code_m }
	}

	pub(crate) fn new1(base_reg1: Register, base_reg2: Register, base_reg3: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHW::decode, has_modrm: true, base_reg1, base_reg2, base_reg3, code_r: code, code_m: code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg1 as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg2 as u32);
		if decoder.state.mod_ == 3 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_r);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg3 as u32,
			);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code_m);
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VWH {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VWH {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VWH::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
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
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_WHV {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_r: u32,
	pub(crate) code_m: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_WHV {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_WHV::decode, has_modrm: true, base_reg, code_r: code, code_m: code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		debug_assert_eq!(3, decoder.state.mod_);
		super::instruction_internal::internal_set_code_u32(instruction, this.code_r);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHM {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VHM {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHM::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_MHV {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_MHV {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_MHV::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHWIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg1: Register,
	pub(crate) base_reg2: Register,
	pub(crate) base_reg3: Register,
}

impl OpCodeHandler_VEX_VHWIb {
	pub(crate) fn new(base_reg1: Register, base_reg2: Register, base_reg3: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHWIb::decode, has_modrm: true, base_reg1, base_reg2, base_reg3, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg1 as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg2 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg3 as u32,
			);
		} else {
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_HRIb {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_HRIb {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_HRIb::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
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
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHWIs4 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VHWIs4 {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHWIs4::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op3_register_u32(instruction, ((decoder.read_u8() as u32) >> 4) + this.base_reg as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHIs4W {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VHIs4W {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHIs4W::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op3_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register_u32(instruction, ((decoder.read_u8() as u32) >> 4) + this.base_reg as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHWIs5 {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VHWIs5 {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHWIs5::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		let ib = decoder.read_u8() as u32;
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op3_register_u32(instruction, (ib >> 4) + this.base_reg as u32);
		debug_assert_eq!(OpKind::Immediate8, instruction.op4_kind()); // It's hard coded
		super::instruction_internal::internal_set_immediate8(instruction, ib & 3);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VHIs5W {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_VHIs5W {
	pub(crate) fn new(base_reg: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VHIs5W::decode, has_modrm: true, base_reg, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.base_reg as u32,
		);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + this.base_reg as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op3_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.base_reg as u32,
			);
		} else {
			super::instruction_internal::internal_set_op3_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		let ib = decoder.read_u8() as u32;
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register_u32(instruction, (ib >> 4) + this.base_reg as u32);
		debug_assert_eq!(OpKind::Immediate8, instruction.op4_kind()); // It's hard coded
		super::instruction_internal::internal_set_immediate8(instruction, ib & 3);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VK_HK_RK {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_VEX_VK_HK_RK {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VK_HK_RK::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if decoder.invalid_check_mask != 0 && (decoder.state.vvvv > 7 || decoder.state.extra_register_base != 0) {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::K0 as u32);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, (decoder.state.vvvv & 7) + Register::K0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.rm + Register::K0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VK_RK {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_VEX_VK_RK {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VK_RK::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if ((decoder.state.vvvv | decoder.state.extra_register_base) & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::K0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::K0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VK_RK_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_VEX_VK_RK_Ib {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VK_RK_Ib::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if ((decoder.state.vvvv | decoder.state.extra_register_base) & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::K0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::K0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate8);
		super::instruction_internal::internal_set_immediate8(instruction, decoder.read_u8() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VK_WK {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_VEX_VK_WK {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VK_WK::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if ((decoder.state.vvvv | decoder.state.extra_register_base) & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::K0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::K0 as u32);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_M_VK {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
}

impl OpCodeHandler_VEX_M_VK {
	pub(crate) fn new(code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_M_VK::decode, has_modrm: true, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if ((decoder.state.vvvv | decoder.state.extra_register_base) & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.reg + Register::K0 as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VK_R {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) gpr: Register,
}

impl OpCodeHandler_VEX_VK_R {
	pub(crate) fn new(code: u32, gpr: Register) -> Self {
		Self { decode: OpCodeHandler_VEX_VK_R::decode, has_modrm: true, code, gpr }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if ((decoder.state.vvvv | decoder.state.extra_register_base) & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + Register::K0 as u32);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + this.gpr as u32,
			);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_G_VK {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) gpr: Register,
}

impl OpCodeHandler_VEX_G_VK {
	pub(crate) fn new(code: u32, gpr: Register) -> Self {
		Self { decode: OpCodeHandler_VEX_G_VK::decode, has_modrm: true, code, gpr }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(
			instruction,
			decoder.state.reg + decoder.state.extra_register_base + this.gpr as u32,
		);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + Register::K0 as u32);
		} else {
			decoder.set_invalid_instruction();
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Gv_W {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code_w0: u32,
	pub(crate) code_w1: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_Gv_W {
	pub(crate) fn new(base_reg: Register, code_w0: u32, code_w1: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Gv_W::decode, has_modrm: true, base_reg, code_w0, code_w1 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
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
pub(crate) struct OpCodeHandler_VEX_Gv_RX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_Gv_RX {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Gv_RX::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + decoder.state.extra_register_base + gpr);
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
pub(crate) struct OpCodeHandler_VEX_Gv_GPR_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_Gv_GPR_Ib {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Gv_GPR_Ib::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + decoder.state.extra_register_base + gpr);
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
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_VX_VSIB_HX {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code: u32,
	pub(crate) base_reg1: Register,
	pub(crate) vsib_index: Register,
	pub(crate) base_reg3: Register,
}

impl OpCodeHandler_VEX_VX_VSIB_HX {
	pub(crate) fn new(base_reg1: Register, vsib_index: Register, base_reg3: Register, code: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_VX_VSIB_HX::decode, has_modrm: true, base_reg1, vsib_index, base_reg3, code }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		super::instruction_internal::internal_set_code_u32(instruction, this.code);
		let reg_num = decoder.state.reg + decoder.state.extra_register_base;
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, reg_num + this.base_reg1 as u32);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.vvvv + this.base_reg3 as u32);
		if decoder.state.mod_ == 3 {
			decoder.set_invalid_instruction();
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem_vsib(instruction, this.vsib_index, TupleType::None);
			if decoder.invalid_check_mask != 0 {
				let index_num = (instruction.memory_index() as u32).wrapping_sub(Register::XMM0 as u32) % IcedConstants::VMM_COUNT;
				if reg_num == index_num || decoder.state.vvvv == index_num || reg_num == decoder.state.vvvv {
					decoder.set_invalid_instruction();
				}
			}
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Gv_Gv_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_Gv_Gv_Ev {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Gv_Gv_Ev::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + decoder.state.extra_register_base + gpr);
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.vvvv + gpr);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
		} else {
			super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Gv_Ev_Gv {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_Gv_Ev_Gv {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Gv_Ev_Gv::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + decoder.state.extra_register_base + gpr);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op2_register_u32(instruction, decoder.state.vvvv + gpr);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Hv_Ev {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_Hv_Ev {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Hv_Ev::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.vvvv + gpr);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_Hv_Ed_Id {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_Hv_Ed_Id {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Hv_Ed_Id::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.vvvv + Register::RAX as u32);
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.vvvv + Register::EAX as u32);
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(
				instruction,
				decoder.state.rm + decoder.state.extra_base_register_base + Register::EAX as u32,
			);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate32);
		instruction.set_immediate32(decoder.read_u32() as u32);
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct OpCodeHandler_VEX_GvM_VX_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
	pub(crate) base_reg: Register,
}

impl OpCodeHandler_VEX_GvM_VX_Ib {
	pub(crate) fn new(base_reg: Register, code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_GvM_VX_Ib::decode, has_modrm: true, base_reg, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
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
pub(crate) struct OpCodeHandler_VEX_Gv_Ev_Ib {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_Gv_Ev_Ib {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Gv_Ev_Ib::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + decoder.state.extra_register_base + gpr);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
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
pub(crate) struct OpCodeHandler_VEX_Gv_Ev_Id {
	pub(crate) decode: OpCodeHandlerDecodeFn,
	pub(crate) has_modrm: bool,
	pub(crate) code32: u32,
	pub(crate) code64: u32,
}

impl OpCodeHandler_VEX_Gv_Ev_Id {
	pub(crate) fn new(code32: u32, code64: u32) -> Self {
		Self { decode: OpCodeHandler_VEX_Gv_Ev_Id::decode, has_modrm: true, code32, code64 }
	}

	pub(crate) fn decode(self_ptr: *const OpCodeHandler, decoder: &mut Decoder, instruction: &mut Instruction) {
		let this = unsafe { &*(self_ptr as *const Self) };
		debug_assert!(decoder.state.encoding() == EncodingKind::VEX || decoder.state.encoding() == EncodingKind::XOP);
		if (decoder.state.vvvv & decoder.invalid_check_mask) != 0 {
			decoder.set_invalid_instruction();
		}
		let gpr;
		if (decoder.state.flags & decoder.is64_mode_and_w) != 0 {
			super::instruction_internal::internal_set_code_u32(instruction, this.code64);
			gpr = Register::RAX as u32;
		} else {
			super::instruction_internal::internal_set_code_u32(instruction, this.code32);
			gpr = Register::EAX as u32;
		}
		const_assert_eq!(0, OpKind::Register as u32);
		//super::instruction_internal::internal_set_op0_kind(instruction, OpKind::Register);
		super::instruction_internal::internal_set_op0_register_u32(instruction, decoder.state.reg + decoder.state.extra_register_base + gpr);
		if decoder.state.mod_ == 3 {
			const_assert_eq!(0, OpKind::Register as u32);
			//super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Register);
			super::instruction_internal::internal_set_op1_register_u32(instruction, decoder.state.rm + decoder.state.extra_base_register_base + gpr);
		} else {
			super::instruction_internal::internal_set_op1_kind(instruction, OpKind::Memory);
			decoder.read_op_mem(instruction);
		}
		super::instruction_internal::internal_set_op2_kind(instruction, OpKind::Immediate32);
		instruction.set_immediate32(decoder.read_u32() as u32);
	}
}
