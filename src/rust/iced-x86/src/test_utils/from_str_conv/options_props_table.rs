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

#![allow(unused_results)]

use super::super::super::formatter::tests::enums::OptionsProps;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

lazy_static! {
	pub(super) static ref TO_OPTIONS_PROPS_HASH: HashMap<&'static str, OptionsProps> = {
		// GENERATOR-BEGIN: OptionsPropsHash
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut h = HashMap::with_capacity(49);
		h.insert("AddLeadingZeroToHexNumbers", OptionsProps::AddLeadingZeroToHexNumbers);
		h.insert("AlwaysShowScale", OptionsProps::AlwaysShowScale);
		h.insert("AlwaysShowSegmentRegister", OptionsProps::AlwaysShowSegmentRegister);
		h.insert("BinaryDigitGroupSize", OptionsProps::BinaryDigitGroupSize);
		h.insert("BinaryPrefix", OptionsProps::BinaryPrefix);
		h.insert("BinarySuffix", OptionsProps::BinarySuffix);
		h.insert("BranchLeadingZeroes", OptionsProps::BranchLeadingZeroes);
		h.insert("DecimalDigitGroupSize", OptionsProps::DecimalDigitGroupSize);
		h.insert("DecimalPrefix", OptionsProps::DecimalPrefix);
		h.insert("DecimalSuffix", OptionsProps::DecimalSuffix);
		h.insert("DigitSeparator", OptionsProps::DigitSeparator);
		h.insert("DisplacementLeadingZeroes", OptionsProps::DisplacementLeadingZeroes);
		h.insert("FirstOperandCharIndex", OptionsProps::FirstOperandCharIndex);
		h.insert("GasNakedRegisters", OptionsProps::GasNakedRegisters);
		h.insert("GasShowMnemonicSizeSuffix", OptionsProps::GasShowMnemonicSizeSuffix);
		h.insert("GasSpaceAfterMemoryOperandComma", OptionsProps::GasSpaceAfterMemoryOperandComma);
		h.insert("HexDigitGroupSize", OptionsProps::HexDigitGroupSize);
		h.insert("HexPrefix", OptionsProps::HexPrefix);
		h.insert("HexSuffix", OptionsProps::HexSuffix);
		h.insert("IP", OptionsProps::IP);
		h.insert("LeadingZeroes", OptionsProps::LeadingZeroes);
		h.insert("MasmAddDsPrefix32", OptionsProps::MasmAddDsPrefix32);
		h.insert("MemorySizeOptions", OptionsProps::MemorySizeOptions);
		h.insert("NasmShowSignExtendedImmediateSize", OptionsProps::NasmShowSignExtendedImmediateSize);
		h.insert("NumberBase", OptionsProps::NumberBase);
		h.insert("OctalDigitGroupSize", OptionsProps::OctalDigitGroupSize);
		h.insert("OctalPrefix", OptionsProps::OctalPrefix);
		h.insert("OctalSuffix", OptionsProps::OctalSuffix);
		h.insert("PreferST0", OptionsProps::PreferST0);
		h.insert("RipRelativeAddresses", OptionsProps::RipRelativeAddresses);
		h.insert("ScaleBeforeIndex", OptionsProps::ScaleBeforeIndex);
		h.insert("ShowBranchSize", OptionsProps::ShowBranchSize);
		h.insert("ShowZeroDisplacements", OptionsProps::ShowZeroDisplacements);
		h.insert("SignedImmediateOperands", OptionsProps::SignedImmediateOperands);
		h.insert("SignedMemoryDisplacements", OptionsProps::SignedMemoryDisplacements);
		h.insert("SmallHexNumbersInDecimal", OptionsProps::SmallHexNumbersInDecimal);
		h.insert("SpaceAfterMemoryBracket", OptionsProps::SpaceAfterMemoryBracket);
		h.insert("SpaceAfterOperandSeparator", OptionsProps::SpaceAfterOperandSeparator);
		h.insert("SpaceBetweenMemoryAddOperators", OptionsProps::SpaceBetweenMemoryAddOperators);
		h.insert("SpaceBetweenMemoryMulOperators", OptionsProps::SpaceBetweenMemoryMulOperators);
		h.insert("TabSize", OptionsProps::TabSize);
		h.insert("UppercaseAll", OptionsProps::UppercaseAll);
		h.insert("UppercaseDecorators", OptionsProps::UppercaseDecorators);
		h.insert("UppercaseHex", OptionsProps::UppercaseHex);
		h.insert("UppercaseKeywords", OptionsProps::UppercaseKeywords);
		h.insert("UppercaseMnemonics", OptionsProps::UppercaseMnemonics);
		h.insert("UppercasePrefixes", OptionsProps::UppercasePrefixes);
		h.insert("UppercaseRegisters", OptionsProps::UppercaseRegisters);
		h.insert("UsePseudoOps", OptionsProps::UsePseudoOps);
		// GENERATOR-END: OptionsPropsHash
		h
	};
}
