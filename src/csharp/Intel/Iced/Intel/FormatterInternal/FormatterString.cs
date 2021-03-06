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
using System.Diagnostics;

namespace Iced.Intel.FormatterInternal {
	readonly struct FormatterString {
		readonly string lower;
		readonly string upper;

		public bool IsDefault => lower is null;
		public int Length => lower.Length;

		public FormatterString(string lower) {
			Debug.Assert(lower.ToLowerInvariant() == lower);
			this.lower = lower;
			upper = string.Intern(lower.ToUpperInvariant());
		}

		public string Get(bool upper) {
			Debug.Assert(!IsDefault);
			return upper ? this.upper : lower;
		}
	}
}
#endif
