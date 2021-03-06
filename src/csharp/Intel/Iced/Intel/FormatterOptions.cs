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

#if !NO_GAS || !NO_INTEL || !NO_MASM || !NO_NASM
namespace Iced.Intel {
	/// <summary>
	/// Formatter options
	/// </summary>
	public sealed class FormatterOptions {
		/// <summary>
		/// Prefixes are upper cased
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>REP stosd</c>
		/// <br/>
		/// <see langword="false"/>: <c>rep stosd</c>
		/// </summary>
		public bool UppercasePrefixes { get; set; }

		/// <summary>
		/// Mnemonics are upper cased
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>MOV rcx,rax</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov rcx,rax</c>
		/// </summary>
		public bool UppercaseMnemonics { get; set; }

		/// <summary>
		/// Registers are upper cased
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov RCX,[RAX+RDX*8]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov rcx,[rax+rdx*8]</c>
		/// </summary>
		public bool UppercaseRegisters { get; set; }

		/// <summary>
		/// Keywords are upper cased (eg. <c>BYTE PTR</c>, <c>SHORT</c>)
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov BYTE PTR [rcx],12h</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov byte ptr [rcx],12h</c>
		/// </summary>
		public bool UppercaseKeywords { get; set; }

		/// <summary>
		/// Upper case decorators, eg. <c>{z}</c>, <c>{sae}</c>, <c>{rd-sae}</c> (but not op mask registers: <c>{k1}</c>)
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>vunpcklps xmm2{k5}{Z},xmm6,dword bcst [rax+4]</c>
		/// <br/>
		/// <see langword="false"/>: <c>vunpcklps xmm2{k5}{z},xmm6,dword bcst [rax+4]</c>
		/// </summary>
		public bool UppercaseDecorators { get; set; }

		/// <summary>
		/// Everything is upper cased, except numbers and their prefixes/suffixes
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>MOV EAX,GS:[RCX*4+0ffh]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,gs:[rcx*4+0ffh]</c>
		/// </summary>
		public bool UppercaseAll { get; set; }

		/// <summary>
		/// Character index (0-based) where the first operand is formatted. Can be set to 0 to format it immediately after the mnemonic.
		/// At least one space or tab is always added between the mnemonic and the first operand.
		/// <br/>
		/// Default: <c>0</c>
		/// <br/>
		/// <c>0</c>: <c>mov•rcx,rbp</c>
		/// <br/>
		/// <c>8</c>: <c>mov•••••rcx,rbp</c>
		/// <br/>
		/// </summary>
		public int FirstOperandCharIndex { get; set; }

		/// <summary>
		/// Size of a tab character or &lt;= 0 to use spaces
		/// <br/>
		/// Default: <c>0</c>
		/// </summary>
		public int TabSize { get; set; }

		/// <summary>
		/// Add a space after the operand separator
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov rax, rcx</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov rax,rcx</c>
		/// </summary>
		public bool SpaceAfterOperandSeparator { get; set; }

		/// <summary>
		/// Add a space between the memory expression and the brackets
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[ rcx+rdx ]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[rcx+rdx]</c>
		/// </summary>
		public bool SpaceAfterMemoryBracket { get; set; }

		/// <summary>
		/// Add spaces between memory operand <c>+</c> and <c>-</c> operators
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[rcx + rdx*8 - 80h]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[rcx+rdx*8-80h]</c>
		/// </summary>
		public bool SpaceBetweenMemoryAddOperators { get; set; }

		/// <summary>
		/// Add spaces between memory operand <c>*</c> operator
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[rcx+rdx * 8-80h]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[rcx+rdx*8-80h]</c>
		/// </summary>
		public bool SpaceBetweenMemoryMulOperators { get; set; }

		/// <summary>
		/// Show memory operand scale value before the index register
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[8*rdx]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[rdx*8]</c>
		/// </summary>
		public bool ScaleBeforeIndex { get; set; }

		/// <summary>
		/// Always show the scale value even if it's <c>*1</c>
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[rbx+rcx*1]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[rbx+rcx]</c>
		/// </summary>
		public bool AlwaysShowScale { get; set; }

		/// <summary>
		/// Always show the effective segment register. If the option is <see langword="false"/>, only show the segment register if
		/// there's a segment override prefix.
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,ds:[ecx]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[ecx]</c>
		/// </summary>
		public bool AlwaysShowSegmentRegister { get; set; }

		/// <summary>
		/// Show zero displacements
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[rcx*2+0]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[rcx*2]</c>
		/// </summary>
		public bool ShowZeroDisplacements { get; set; }

		/// <summary>
		/// Hex number prefix or <see langword="null"/>/empty string, eg. "0x"
		/// <br/>
		/// Default: <see langword="null"/> (masm/nasm/intel), <c>"0x"</c> (gas)
		/// </summary>
		public string? HexPrefix { get; set; }

		/// <summary>
		/// Hex number suffix or <see langword="null"/>/empty string, eg. "h"
		/// <br/>
		/// Default: <c>"h"</c> (masm/nasm/intel), <see langword="null"/> (gas)
		/// </summary>
		public string? HexSuffix { get; set; }

		/// <summary>
		/// Size of a digit group, see also <see cref="DigitSeparator"/>
		/// <br/>
		/// Default: <c>4</c>
		/// <br/>
		/// <c>0</c>: <c>0x12345678</c>
		/// <br/>
		/// <c>4</c>: <c>0x1234_5678</c>
		/// </summary>
		public int HexDigitGroupSize { get; set; } = 4;

		/// <summary>
		/// Decimal number prefix or <see langword="null"/>/empty string
		/// <br/>
		/// Default: <see langword="null"/>
		/// </summary>
		public string? DecimalPrefix { get; set; }

		/// <summary>
		/// Decimal number suffix or <see langword="null"/>/empty string
		/// <br/>
		/// Default: <see langword="null"/>
		/// </summary>
		public string? DecimalSuffix { get; set; }

		/// <summary>
		/// Size of a digit group, see also <see cref="DigitSeparator"/>
		/// <br/>
		/// Default: <c>3</c>
		/// <br/>
		/// <c>0</c>: <c>12345678</c>
		/// <br/>
		/// <c>3</c>: <c>12_345_678</c>
		/// </summary>
		public int DecimalDigitGroupSize { get; set; } = 3;

		/// <summary>
		/// Octal number prefix or <see langword="null"/>/empty string
		/// <br/>
		/// Default: <see langword="null"/> (masm/nasm/intel), <c>"0"</c> (gas)
		/// </summary>
		public string? OctalPrefix { get; set; }

		/// <summary>
		/// Octal number suffix or <see langword="null"/>/empty string
		/// <br/>
		/// Default: <c>"o"</c> (masm/nasm/intel), <see langword="null"/> (gas)
		/// </summary>
		public string? OctalSuffix { get; set; }

		/// <summary>
		/// Size of a digit group, see also <see cref="DigitSeparator"/>
		/// <br/>
		/// Default: <c>4</c>
		/// <br/>
		/// <c>0</c>: <c>12345670</c>
		/// <br/>
		/// <c>4</c>: <c>1234_5670</c>
		/// </summary>
		public int OctalDigitGroupSize { get; set; } = 4;

		/// <summary>
		/// Binary number prefix or <see langword="null"/>/empty string
		/// <br/>
		/// Default: <see langword="null"/> (masm/nasm/intel), <c>"0b"</c> (gas)
		/// </summary>
		public string? BinaryPrefix { get; set; }

		/// <summary>
		/// Binary number suffix or <see langword="null"/>/empty string
		/// <br/>
		/// Default: <c>"b"</c> (masm/nasm/intel), <see langword="null"/> (gas)
		/// </summary>
		public string? BinarySuffix { get; set; }

		/// <summary>
		/// Size of a digit group, see also <see cref="DigitSeparator"/>
		/// <br/>
		/// Default: <c>4</c>
		/// <br/>
		/// <c>0</c>: <c>11010111</c>
		/// <br/>
		/// <c>4</c>: <c>1101_0111</c>
		/// </summary>
		public int BinaryDigitGroupSize { get; set; } = 4;

		/// <summary>
		/// Digit separator or <see langword="null"/>/empty string. See also eg. <see cref="HexDigitGroupSize"/>.
		/// <br/>
		/// Default: <see langword="null"/>
		/// <br/>
		/// <c>""</c>: <c>0x12345678</c>
		/// <br/>
		/// <c>"_"</c>: <c>0x1234_5678</c>
		/// </summary>
		public string? DigitSeparator { get; set; }

		/// <summary>
		/// Add leading zeroes to hexadecimal/octal/binary numbers.
		/// This option has no effect on branch targets and displacements, use <see cref="BranchLeadingZeroes"/>
		/// and <see cref="DisplacementLeadingZeroes"/>.
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>0x0000000A</c>/<c>0000000Ah</c>
		/// <br/>
		/// <see langword="false"/>: <c>0xA</c>/<c>0Ah</c>
		/// </summary>
		public bool LeadingZeroes { get; set; }

		/// <summary>
		/// Use upper case hex digits
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>0xFF</c>
		/// <br/>
		/// <see langword="false"/>: <c>0xff</c>
		/// </summary>
		public bool UppercaseHex { get; set; } = true;

		/// <summary>
		/// Small hex numbers (-9 .. 9) are shown in decimal
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>9</c>
		/// <br/>
		/// <see langword="false"/>: <c>0x9</c>
		/// </summary>
		public bool SmallHexNumbersInDecimal { get; set; } = true;

		/// <summary>
		/// Add a leading zero to hex numbers if there's no prefix and the number starts with hex digits <c>A-F</c>
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>0FFh</c>
		/// <br/>
		/// <see langword="false"/>: <c>FFh</c>
		/// </summary>
		public bool AddLeadingZeroToHexNumbers { get; set; } = true;

		/// <summary>
		/// Number base
		/// <br/>
		/// Default: <see cref="NumberBase.Hexadecimal"/>
		/// </summary>
		public NumberBase NumberBase {
			get => numberBase;
			set {
				if ((uint)value > (uint)NumberBase.Binary)
					ThrowHelper.ThrowArgumentOutOfRangeException_value();
				numberBase = value;
			}
		}
		NumberBase numberBase = NumberBase.Hexadecimal;

		/// <summary>
		/// Add leading zeroes to branch offsets. Used by <c>CALL NEAR</c>, <c>CALL FAR</c>, <c>JMP NEAR</c>, <c>JMP FAR</c>, <c>Jcc</c>, <c>LOOP</c>, <c>LOOPcc</c>, <c>XBEGIN</c>
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>je 00000123h</c>
		/// <br/>
		/// <see langword="false"/>: <c>je 123h</c>
		/// </summary>
		public bool BranchLeadingZeroes { get; set; } = true;

		/// <summary>
		/// Show immediate operands as signed numbers
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,-1</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,FFFFFFFF</c>
		/// </summary>
		public bool SignedImmediateOperands { get; set; }

		/// <summary>
		/// Displacements are signed numbers
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov al,[eax-2000h]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov al,[eax+0FFFFE000h]</c>
		/// </summary>
		public bool SignedMemoryDisplacements { get; set; } = true;

		/// <summary>
		/// Add leading zeroes to displacements
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov al,[eax+00000012h]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov al,[eax+12h]</c>
		/// </summary>
		public bool DisplacementLeadingZeroes { get; set; }

		/// <summary>
		/// Add leading zeroes to displacements
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov al,[eax+00000012h]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov al,[eax+12h]</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool SignExtendMemoryDisplacements {
			get => DisplacementLeadingZeroes;
			set => DisplacementLeadingZeroes = value;
		}

		/// <summary>
		/// Options that control if the memory size (eg. <c>DWORD PTR</c>) is shown or not.
		/// This is ignored by the gas (AT&amp;T) formatter.
		/// <br/>
		/// Default: <see cref="Intel.MemorySizeOptions.Default"/>
		/// </summary>
		public MemorySizeOptions MemorySizeOptions {
			get => memorySizeOptions;
			set {
				if ((uint)value > (uint)MemorySizeOptions.Never)
					ThrowHelper.ThrowArgumentOutOfRangeException_value();
				memorySizeOptions = value;
			}
		}
		MemorySizeOptions memorySizeOptions = MemorySizeOptions.Default;

		/// <summary>
		/// Show <c>RIP+displ</c> or the virtual address
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[rip+12345678h]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[1029384756AFBECDh]</c>
		/// </summary>
		public bool RipRelativeAddresses { get; set; }

		/// <summary>
		/// Show <c>NEAR</c>, <c>SHORT</c>, etc if it's a branch instruction
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>je short 1234h</c>
		/// <br/>
		/// <see langword="false"/>: <c>je 1234h</c>
		/// </summary>
		public bool ShowBranchSize { get; set; } = true;

		/// <summary>
		/// Use pseudo instructions
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>vcmpnltsd xmm2,xmm6,xmm3</c>
		/// <br/>
		/// <see langword="false"/>: <c>vcmpsd xmm2,xmm6,xmm3,5</c>
		/// </summary>
		public bool UsePseudoOps { get; set; } = true;

		/// <summary>
		/// Show the original value after the symbol name
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,[myfield (12345678)]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[myfield]</c>
		/// </summary>
		public bool ShowSymbolAddress { get; set; }

		/// <summary>
		/// (gas only): If <see langword="true"/>, the formatter doesn't add <c>%</c> to registers
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,ecx</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov %eax,%ecx</c>
		/// </summary>
		public bool GasNakedRegisters { get; set; }

		/// <summary>
		/// (gas only): Shows the mnemonic size suffix even when not needed
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>movl %eax,%ecx</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov %eax,%ecx</c>
		/// </summary>
		public bool GasShowMnemonicSizeSuffix { get; set; }

		/// <summary>
		/// (gas only): Add a space after the comma if it's a memory operand
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>(%eax, %ecx, 2)</c>
		/// <br/>
		/// <see langword="false"/>: <c>(%eax,%ecx,2)</c>
		/// </summary>
		public bool GasSpaceAfterMemoryOperandComma { get; set; }

		/// <summary>
		/// (masm only): Add a <c>DS</c> segment override even if it's not present. Used if it's 16/32-bit code and mem op is a displ
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov eax,ds:[12345678]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,[12345678]</c>
		/// </summary>
		public bool MasmAddDsPrefix32 { get; set; } = true;

		/// <summary>
		/// (masm only): Show symbols in brackets
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>[ecx+symbol]</c> / <c>[symbol]</c>
		/// <br/>
		/// <see langword="false"/>: <c>symbol[ecx]</c> / <c>symbol</c>
		/// </summary>
		public bool MasmSymbolDisplInBrackets { get; set; } = true;

		/// <summary>
		/// (masm only): Show displacements in brackets
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>[ecx+1234h]</c>
		/// <br/>
		/// <see langword="false"/>: <c>1234h[ecx]</c>
		/// </summary>
		public bool MasmDisplInBrackets { get; set; } = true;

		/// <summary>
		/// (nasm only): Shows <c>BYTE</c>, <c>WORD</c>, <c>DWORD</c> or <c>QWORD</c> if it's a sign extended immediate operand value
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>or rcx,byte -1</c>
		/// <br/>
		/// <see langword="false"/>: <c>or rcx,-1</c>
		/// </summary>
		public bool NasmShowSignExtendedImmediateSize { get; set; }

		/// <summary>
		/// Use <c>st(0)</c> instead of <c>st</c> if <c>st</c> can be used. Ignored by the nasm formatter.
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>fadd st(0),st(3)</c>
		/// <br/>
		/// <see langword="false"/>: <c>fadd st,st(3)</c>
		/// </summary>
		public bool PreferST0 { get; set; }

		/// <summary>
		/// Prefixes are upper cased
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>REP stosd</c>
		/// <br/>
		/// <see langword="false"/>: <c>rep stosd</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool UpperCasePrefixes {
			get => UppercasePrefixes;
			set => UppercasePrefixes = value;
		}

		/// <summary>
		/// Mnemonics are upper cased
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>MOV rcx,rax</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov rcx,rax</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool UpperCaseMnemonics {
			get => UppercaseMnemonics;
			set => UppercaseMnemonics = value;
		}

		/// <summary>
		/// Registers are upper cased
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov RCX,[RAX+RDX*8]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov rcx,[rax+rdx*8]</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool UpperCaseRegisters {
			get => UppercaseRegisters;
			set => UppercaseRegisters = value;
		}

		/// <summary>
		/// Keywords are upper cased (eg. <c>BYTE PTR</c>, <c>SHORT</c>)
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>mov BYTE PTR [rcx],12h</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov byte ptr [rcx],12h</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool UpperCaseKeywords {
			get => UppercaseKeywords;
			set => UppercaseKeywords = value;
		}

		/// <summary>
		/// Upper case decorators, eg. <c>{z}</c>, <c>{sae}</c>, <c>{rd-sae}</c> (but not op mask registers: <c>{k1}</c>)
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>vunpcklps xmm2{k5}{Z},xmm6,dword bcst [rax+4]</c>
		/// <br/>
		/// <see langword="false"/>: <c>vunpcklps xmm2{k5}{z},xmm6,dword bcst [rax+4]</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool UpperCaseDecorators {
			get => UppercaseDecorators;
			set => UppercaseDecorators = value;
		}

		/// <summary>
		/// Everything is upper cased, except numbers and their prefixes/suffixes
		/// <br/>
		/// Default: <see langword="false"/>
		/// <br/>
		/// <see langword="true"/>: <c>MOV EAX,GS:[RCX*4+0ffh]</c>
		/// <br/>
		/// <see langword="false"/>: <c>mov eax,gs:[rcx*4+0ffh]</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool UpperCaseAll {
			get => UppercaseAll;
			set => UppercaseAll = value;
		}

		/// <summary>
		/// Use upper case hex digits
		/// <br/>
		/// Default: <see langword="true"/>
		/// <br/>
		/// <see langword="true"/>: <c>0xFF</c>
		/// <br/>
		/// <see langword="false"/>: <c>0xff</c>
		/// </summary>
		[System.Obsolete("Use " + nameof(DisplacementLeadingZeroes) + " instead of this property", true)]
		[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
		public bool UpperCaseHex {
			get => UppercaseHex;
			set => UppercaseHex= value;
		}

#if !NO_GAS
		/// <summary>
		/// Creates GNU assembler (AT&amp;T) formatter options
		/// </summary>
		/// <returns></returns>
		public static FormatterOptions CreateGas() =>
			new FormatterOptions {
				HexPrefix = "0x",
				OctalPrefix = "0",
				BinaryPrefix = "0b",
			};
#endif

#if !NO_INTEL
		/// <summary>
		/// Creates Intel (XED) formatter options
		/// </summary>
		/// <returns></returns>
		public static FormatterOptions CreateIntel() =>
			new FormatterOptions {
				HexSuffix = "h",
				OctalSuffix = "o",
				BinarySuffix = "b",
			};
#endif

#if !NO_MASM
		/// <summary>
		/// Creates masm formatter options
		/// </summary>
		/// <returns></returns>
		public static FormatterOptions CreateMasm() =>
			new FormatterOptions {
				HexSuffix = "h",
				OctalSuffix = "o",
				BinarySuffix = "b",
			};
#endif

#if !NO_NASM
		/// <summary>
		/// Creates nasm formatter options
		/// </summary>
		/// <returns></returns>
		public static FormatterOptions CreateNasm() =>
			new FormatterOptions {
				HexSuffix = "h",
				OctalSuffix = "o",
				BinarySuffix = "b",
			};
#endif
	}

	// GENERATOR-BEGIN: NumberBase
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	/// <summary>Number base</summary>
	public enum NumberBase {
		/// <summary>Hex numbers (base 16)</summary>
		Hexadecimal,
		/// <summary>Decimal numbers (base 10)</summary>
		Decimal,
		/// <summary>Octal numbers (base 8)</summary>
		Octal,
		/// <summary>Binary numbers (base 2)</summary>
		Binary,
	}
	// GENERATOR-END: NumberBase

	// GENERATOR-BEGIN: MemorySizeOptions
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	/// <summary>Memory size options used by the formatters</summary>
	public enum MemorySizeOptions {
		/// <summary>Show memory size if the assembler requires it, else don&apos;t show anything</summary>
		Default,
		/// <summary>Always show the memory size, even if the assembler doesn&apos;t need it</summary>
		Always,
		/// <summary>Show memory size if a human can&apos;t figure out the size of the operand</summary>
		Minimum,
		/// <summary>Never show memory size</summary>
		Never,
	}
	// GENERATOR-END: MemorySizeOptions
}
#endif
