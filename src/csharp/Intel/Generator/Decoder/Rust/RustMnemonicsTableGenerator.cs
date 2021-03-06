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

using System;
using System.IO;
using Generator.Constants;
using Generator.Enums;
using Generator.IO;

namespace Generator.Decoder.Rust {
	[Generator(TargetLanguage.Rust, GeneratorNames.Code_Mnemonic)]
	sealed class RustMnemonicsTableGenerator {
		readonly IdentifierConverter idConverter;
		readonly GeneratorOptions generatorOptions;

		public RustMnemonicsTableGenerator(GeneratorOptions generatorOptions) {
			idConverter = RustIdentifierConverter.Create();
			this.generatorOptions = generatorOptions;
		}

		public void Generate() {
			var data = MnemonicsTable.Table;
			var mnemonicName = MnemonicEnum.Instance.Name(idConverter);
			using (var writer = new FileWriter(TargetLanguage.Rust, FileUtils.OpenWrite(Path.Combine(generatorOptions.RustDir, "mnemonics.rs")))) {
				writer.WriteFileHeader();

				writer.WriteLine($"use super::iced_constants::{IcedConstantsType.Instance.Name(idConverter)};");
				writer.WriteLine($"use super::{MnemonicEnum.Instance.Name(idConverter)};");
				writer.WriteLine();
				writer.WriteLine(RustConstants.AttributeNoRustFmt);
				writer.WriteLine($"pub(crate) static TO_MNEMONIC: [{mnemonicName}; {IcedConstantsType.Instance.Name(idConverter)}::{IcedConstantsType.Instance[IcedConstants.NumberOfCodeValuesName].Name(idConverter)}] = [");
				using (writer.Indent()) {
					foreach (var d in data) {
						if (d.mnemonicEnum.Value > ushort.MaxValue)
							throw new InvalidOperationException();
						writer.WriteLine($"{mnemonicName}::{d.mnemonicEnum.Name(idConverter)},// {d.codeEnum.Name(idConverter)}");
					}
				}
				writer.WriteLine("];");
			}
		}
	}
}
