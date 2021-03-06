/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Parsing tables extracted from files generated by `Bison` for `C` language.
//! This file was generated by dedicated tool, do not modify manually.

use dmntk_common::Result;

/// Types of tokens returned by lexer.
#[derive(Debug, Clone)]
pub enum TokenType {
  YyEmpty = -2,
  YyEof = 0,
  YyError = 256,
  YyUndef = 257,
  StartExpression = 258,
  StartBoxedExpression = 259,
  StartContext = 260,
  StartTextualExpression = 261,
  StartTextualExpressions = 262,
  StartUnaryTests = 263,
  At = 264,
  Not = 265,
  Colon = 266,
  Comma = 267,
  Every = 268,
  For = 269,
  LeftBrace = 270,
  Null = 271,
  RightArrow = 272,
  Of = 273,
  List = 274,
  Range = 275,
  Context = 276,
  Then = 277,
  Function = 278,
  External = 279,
  If = 280,
  RightBrace = 281,
  RightBracket = 282,
  RightParen = 283,
  Return = 284,
  Ellipsis = 285,
  Some = 286,
  Numeric = 287,
  String = 288,
  Boolean = 289,
  Satisfies = 290,
  Else = 291,
  Or = 292,
  And = 293,
  Eq = 294,
  Nq = 295,
  Lt = 296,
  Le = 297,
  Gt = 298,
  Ge = 299,
  Between = 300,
  BetweenAnd = 301,
  In = 302,
  Minus = 303,
  Plus = 304,
  Mul = 305,
  Div = 306,
  Exp = 307,
  Instance = 309,
  Name = 310,
  NameDateTime = 311,
  BuiltInTypeName = 312,
  LeftParen = 313,
  LeftBracket = 314,
  Dot = 315,
}

/// Kinds of symbols recognized by parser.
#[allow(clippy::enum_variant_names)]
pub enum SymbolKind {
  YyEmpty = -2,
  YyEof = 0,
  YyError = 1,
  YyUndef = 2,
  YyAccept = 61,
}

///
pub const YY_PACT_N_INF: i16 = -209;

///
pub const YY_TABLE_N_INF: i16 = -106;

///
pub const YY_FINAL: usize = 48;

///
pub const YY_LAST: i16 = 885;

///
pub const YY_N_TOKENS: usize = 61;

/// `YY_TRANSLATE[TOKEN-NUM]` - symbol number corresponding to TOKEN-NUM as returned by lexer.
pub const YY_TRANSLATE: [i8; 316] = [
  0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
  2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 3, 4,
  5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
  45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
];

/// `YY_PACT[STATE-NUM]` - index in YY_TABLE of the portion describing STATE-NUM.
pub const YY_PACT: [i16; 282] = [
  174, 308, -5, 20, 308, 308, -209, 42, 15, -209, -209, -209, -209, -8, 308, 61, -209, -209, -209, -209, 61, 61, 61, 61, 308, -7, 1, 345, 382, 744, -209, -209,
  -209, -209, 61, -209, -209, -209, -209, -209, 419, -209, -209, 744, 63, 99, -209, 186, -209, -209, -209, -209, 18, -209, 595, 30, 31, -209, -209, -209, -209,
  -209, -209, -209, -209, 107, 45, -209, -11, 646, 67, 78, 61, 479, 80, -209, 308, 308, 308, 308, 308, 308, 308, 308, -209, 456, 308, 308, 308, 308, 308, 83,
  234, 308, 57, -209, 6, 308, 56, 308, 571, -209, -209, 84, 114, 72, 106, 116, 87, -209, -209, -209, -209, 34, 127, 17, 308, 93, -209, 104, -209, 234, 94,
  -209, -209, 308, -209, -209, -209, 767, 789, 811, 811, 811, 811, 811, 811, 308, 345, 825, 71, 71, 92, 92, 107, -209, -209, -3, 513, -209, -209, 55, -209,
  621, -209, -209, -209, -209, -209, 308, 308, 308, -209, -209, 308, -209, -209, 19, -209, -209, 308, -209, 134, 271, -209, 696, -209, 308, -209, 8, 479, 720,
  547, -209, 86, 308, 308, -209, -209, 98, -209, -209, -209, 126, -209, 744, -209, 109, 744, -209, 111, 34, 744, -209, 308, 744, -209, 90, 308, 744, -209, 308,
  308, 118, 121, 129, 130, -209, -209, -209, 744, 513, 157, 55, -209, 308, 308, -209, 86, 744, 119, -209, -209, 744, 825, 145, -209, -209, 120, 140, -209,
  -209, 744, 671, -209, -209, -209, -209, 86, 86, 175, -209, 12, -209, 170, 86, 308, 90, 146, 147, -209, 120, -209, -209, -209, 13, 744, -209, -209, -209, 86,
  12, 86, -209, -209, -209, -209, -209, -209, 86, 13, -209,
];

/// `YY_DEF_ACT[STATE-NUM]` - default reduction number in state STATE-NUM.
/// Performed when YY_TABLE does not specify something else to do.
/// Zero means the default is an error.
pub const YY_DEF_ACT: [u8; 282] = [
  0, 0, 0, 0, 0, 0, 7, 0, 0, 19, 14, 80, 73, 0, 0, 0, 17, 74, 75, 76, 0, 0, 0, 0, 0, 47, 0, 0, 0, 2, 9, 10, 46, 61, 0, 45, 72, 13, 11, 12, 0, 3, 4, 0, 10, 10,
  6, 0, 1, 77, 135, 128, 0, 138, 0, 105, 0, 69, 71, 70, 135, 57, 58, 59, 60, 38, 0, 78, 47, 0, 0, 72, 90, 0, 0, 89, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 62, 0, 0, 0, 51, 55, 8, 53, 0, 134, 0, 0, 127, 0, 82, 88, 87, 81, 0, 0, 0, 0, 0, 64, 0, 41, 0, 0, 48, 63, 0, 92, 91, 65, 23, 24, 25, 26, 27,
  28, 29, 30, 0, 0, 32, 34, 33, 35, 36, 37, 39, 94, 47, 0, 44, 95, 0, 96, 0, 42, 68, 66, 67, 49, 0, 0, 0, 135, 136, 0, 128, 129, 0, 85, 83, 0, 140, 148, 0,
  141, 0, 104, 0, 79, 41, 0, 0, 0, 31, 0, 0, 0, 102, 101, 0, 99, 97, 43, 0, 54, 20, 133, 0, 15, 126, 0, 0, 84, 146, 0, 150, 139, 0, 0, 18, 93, 0, 0, 0, 0, 0,
  0, 106, 107, 40, 98, 0, 0, 0, 52, 0, 0, 86, 0, 149, 0, 143, 142, 16, 22, 0, 108, 110, 0, 121, 103, 100, 137, 131, 130, 147, 144, 56, 0, 0, 0, 112, 0, 120, 0,
  0, 0, 0, 0, 0, 116, 0, 118, 115, 113, 0, 132, 145, 109, 111, 0, 0, 0, 124, 123, 122, 117, 119, 114, 0, 0, 125,
];

/// `YY_P_GOTO[NTERM-NUM]`
pub const YY_P_GOTO: [i16; 70] = [
  -209, -209, -209, -1, 189, 7, -209, -209, -209, -209, -209, 91, -209, -154, -209, -209, -209, -209, -209, 9, -209, -209, -6, -209, 190, -209, -209, 25, -4,
  -209, -209, -209, 14, 77, -209, 16, -19, -209, -15, -115, -208, -209, -209, -209, -209, -53, -209, -57, -209, -209, -68, -209, 50, -209, -209, -209, -209,
  -59, -209, -209, -209, -209, -209, -209, -209, -41, -209, -9, -209, -209,
];

/// `YY_DEF_GOTO[NTERM-NUM]`
pub const YY_DEF_GOTO: [i16; 70] = [
  0, 7, 47, 100, 30, 31, 51, 60, 50, 137, 184, 46, 101, 102, 183, 32, 33, 34, 95, 56, 57, 35, 36, 121, 37, 52, 112, 113, 169, 114, 38, 75, 127, 149, 150, 151,
  191, 152, 188, 59, 219, 248, 249, 272, 251, 252, 270, 263, 254, 255, 275, 279, 106, 107, 108, 200, 244, 103, 104, 105, 197, 39, 115, 173, 207, 232, 257, 174,
  228, 206,
];

/// `YY_TABLE[YY_PACT[STATE-NUM]]` - what to do in state STATE-NUM.
/// If positive, shift that token.
/// If negative, reduce the rule whose number is the opposite.
/// If `YY_TABLE_N_INF`, syntax error.
pub const YY_TABLE: [i16; 886] = [
  29, 119, 176, 43, 43, 193, 194, 176, 185, 58, 11, 44, 45, 54, 58, 58, 58, 58, 13, -105, 245, 71, 71, 65, 261, 273, 69, 73, 58, 61, 62, 63, 64, 155, 156, 11,
  70, 74, -105, 73, 258, 259, 48, 96, 109, 171, 167, 265, 49, 122, 53, 110, 110, 66, 40, 262, 274, 66, 235, 67, 168, 118, 276, -5, 278, 157, 58, 189, 117, 218,
  8, 280, 172, 111, 111, 129, 130, 131, 132, 133, 134, 135, 136, 190, 139, 140, 141, 142, 143, 144, 117, 148, 153, 17, 18, 19, 43, 124, 65, -50, 120, 145, 230,
  196, 45, 213, 214, 215, -71, 216, 128, 97, 154, 218, 159, 175, 55, 26, 231, 161, 148, 88, 89, 90, 180, 91, 162, 163, 165, 92, 93, 94, 71, 218, 218, 164, 181,
  182, 170, 177, 218, 55, 166, 217, 90, 203, 91, 70, 55, 179, 92, 93, 94, 222, 224, 218, 225, 218, 226, 236, 195, 91, 237, 198, 218, 92, 93, 94, 185, 202, 238,
  239, 205, 247, 172, 250, 209, 1, 2, 3, 4, 5, 6, 253, 220, 221, 260, 264, 158, 268, 269, 41, 201, 42, 210, 8, 98, 227, 178, 9, 10, 11, 12, 229, 241, 223, 240,
  233, 271, 13, 234, 14, 281, 15, 277, 199, 267, 16, 17, 18, 19, 246, 0, 0, 242, 243, 0, 20, 21, 22, 23, 0, 0, 0, 99, 0, 0, 0, 0, 0, 0, 25, 26, 8, 27, 28, 0,
  9, 10, 11, 12, 0, 0, 0, 0, 266, 0, 13, 0, 14, 0, 15, 146, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 147, 26,
  0, 27, 28, 13, 204, 14, 0, 15, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 25, 26, 0, 27, 28, 13, 0, 14, 0,
  15, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 25, 26, 0, 27, 28, 13, 0, 14, 0, 15, 0, 0, 0, 16, 17, 18,
  19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 68, 26, 0, 27, 28, 13, 0, 14, 0, 72, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20,
  21, 22, 23, 0, 8, 0, 24, 0, 9, 10, 11, 12, 0, 68, 26, 0, 27, 28, 13, 0, 14, 0, 72, 0, 0, 0, 16, 17, 18, 19, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 8, 0, 24, 0,
  9, 10, 11, 12, 0, 25, 26, 0, 27, 28, 13, 0, 14, 0, 15, 0, 0, 0, 16, 17, 18, 19, 125, 0, 0, 0, 0, 0, 20, 21, 22, 23, 0, 0, 0, 24, 0, 126, 0, 0, 0, 0, 25, 26,
  0, 138, 28, 76, 77, 78, 79, 80, 81, 82, 83, 84, 186, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 0, 187, 0, 0, 0, 0, 0, 0, 0, 0, 76, 77, 78, 79, 80,
  81, 82, 83, 84, 212, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 0, 123, 0, 0, 0, 0, 0, 0, 0, 160, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87,
  88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79, 80, 81, 82, 83, 84, 116, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79, 80, 81,
  82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 192, 91, 0, 0, 0, 92, 93, 94, 0, 0, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 123, 91, 0, 0, 0,
  92, 93, 94, 0, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 256, 0, 0, 92, 93, 94, 0, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86,
  87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 208, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79,
  80, 81, 82, 83, 84, 211, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 76, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0,
  92, 93, 94, 77, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 78, 79, 80, 81, 82, 83, 84, 0, 85, 86, 87, 88, 89, 90, 0,
  91, 0, 0, 0, 92, 93, 94, -106, -106, -106, -106, -106, -106, 84, 0, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0, 0, 92, 93, 94, 85, 86, 87, 88, 89, 90, 0, 91, 0, 0,
  0, 92, 93, 94,
];

/// ???
pub const YY_CHECK: [i16; 886] = [
  1, 60, 117, 4, 5, 159, 160, 122, 11, 15, 15, 4, 5, 14, 20, 21, 22, 23, 23, 30, 228, 27, 28, 24, 12, 12, 27, 28, 34, 20, 21, 22, 23, 27, 28, 15, 27, 28, 30,
  40, 248, 249, 0, 34, 26, 28, 12, 255, 33, 60, 58, 33, 33, 60, 59, 43, 43, 60, 212, 58, 26, 30, 270, 0, 272, 59, 72, 12, 60, 184, 9, 279, 55, 55, 55, 76, 77,
  78, 79, 80, 81, 82, 83, 28, 85, 86, 87, 88, 89, 90, 60, 92, 93, 32, 33, 34, 97, 30, 99, 0, 55, 18, 12, 162, 97, 19, 20, 21, 30, 23, 30, 12, 55, 228, 58, 116,
  55, 56, 28, 35, 121, 50, 51, 52, 125, 54, 12, 55, 12, 58, 59, 60, 138, 248, 249, 29, 137, 138, 11, 35, 255, 55, 55, 57, 52, 11, 54, 138, 55, 55, 58, 59, 60,
  55, 28, 270, 47, 272, 47, 41, 161, 54, 41, 164, 279, 58, 59, 60, 11, 170, 41, 41, 173, 28, 55, 55, 177, 3, 4, 5, 6, 7, 8, 43, 185, 186, 11, 17, 97, 43, 43,
  2, 167, 3, 180, 9, 10, 201, 121, 13, 14, 15, 16, 204, 223, 189, 221, 208, 261, 23, 211, 25, 280, 27, 271, 165, 257, 31, 32, 33, 34, 230, -1, -1, 225, 226,
  -1, 41, 42, 43, 44, -1, -1, -1, 48, -1, -1, -1, -1, -1, -1, 55, 56, 9, 58, 59, -1, 13, 14, 15, 16, -1, -1, -1, -1, 256, -1, 23, -1, 25, -1, 27, 28, -1, -1,
  31, 32, 33, 34, -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, 24, 25, -1, 27, -1, -1, -1, 31, 32,
  33, 34, -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34,
  -1, -1, -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, -1, -1,
  -1, -1, -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1,
  -1, -1, 41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, -1, -1, -1, -1, -1, -1,
  41, 42, 43, 44, -1, 9, -1, 48, -1, 13, 14, 15, 16, -1, 55, 56, -1, 58, 59, 23, -1, 25, -1, 27, -1, -1, -1, 31, 32, 33, 34, 12, -1, -1, -1, -1, -1, 41, 42,
  43, 44, -1, -1, -1, 48, -1, 27, -1, -1, -1, -1, 55, 56, -1, 58, 59, 37, 38, 39, 40, 41, 42, 43, 44, 45, 12, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58,
  59, 60, -1, 28, -1, -1, -1, -1, -1, -1, -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, 12, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, -1, 28, -1,
  -1, -1, -1, -1, -1, -1, 12, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40, 41, 42, 43, 44,
  45, 22, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, 27, 54, -1, -1, -1, 58, 59,
  60, -1, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, 28, 54, -1, -1, -1, 58, 59, 60, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47,
  48, 49, 50, 51, 52, -1, 54, 30, -1, -1, 58, 59, 60, -1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 36,
  37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
  52, -1, 54, -1, -1, -1, 58, 59, 60, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 38, 39, 40, 41, 42, 43,
  44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60,
  39, 40, 41, 42, 43, 44, 45, -1, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60, 47, 48, 49, 50, 51, 52, -1, 54, -1, -1, -1, 58, 59, 60,
];

/// `YY_R1[YYN]` - symbol number of symbol that rule YYN derives.
pub const YY_R1: [u8; 151] = [
  0, 61, 62, 62, 62, 62, 62, 63, 62, 64, 64, 65, 65, 65, 67, 66, 66, 68, 66, 69, 66, 70, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
  71, 66, 66, 66, 66, 66, 66, 66, 66, 66, 72, 72, 73, 73, 73, 74, 74, 75, 76, 76, 76, 76, 76, 77, 78, 78, 78, 79, 79, 79, 80, 81, 81, 82, 82, 83, 83, 83, 83,
  84, 83, 86, 85, 87, 87, 88, 89, 89, 90, 90, 91, 92, 92, 93, 93, 94, 94, 94, 95, 96, 97, 97, 98, 99, 99, 100, 100, 101, 101, 102, 101, 103, 101, 101, 104,
  101, 105, 107, 106, 108, 108, 109, 110, 109, 111, 112, 111, 113, 113, 115, 116, 114, 117, 117, 118, 118, 120, 121, 119, 123, 122, 124, 125, 124, 126, 127,
  126, 129, 128, 128, 130, 130,
];

/// `YY_R2[YYN]` - number of symbols on the right hand side of rule YYN.
pub const YY_R2: [i8; 151] = [
  0, 2, 2, 2, 2, 2, 2, 0, 3, 1, 1, 1, 1, 1, 0, 5, 6, 0, 5, 0, 5, 0, 6, 3, 3, 3, 3, 3, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 2, 0, 5, 3, 3, 4, 3, 1, 1, 1, 3, 3, 1, 1,
  4, 1, 3, 1, 4, 2, 2, 2, 2, 1, 2, 3, 3, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 0, 4, 0, 3, 1, 2, 3, 1, 3, 1, 1, 2, 1, 2, 1, 3, 1, 1, 1, 2, 3, 1, 3, 2, 1, 3,
  3, 1, 1, 1, 0, 5, 0, 5, 3, 0, 6, 2, 0, 4, 1, 3, 1, 0, 3, 1, 0, 4, 3, 1, 0, 0, 5, 1, 3, 3, 1, 0, 0, 5, 0, 5, 1, 0, 3, 1, 0, 4, 0, 4, 1, 2, 1,
];

///Trait for reduce action definitions.
pub trait ReduceActions {
  fn action_addition(&mut self) -> Result<()>;
  fn action_between(&mut self) -> Result<()>;
  fn action_between_begin(&mut self) -> Result<()>;
  fn action_built_in_type_name(&mut self) -> Result<()>;
  fn action_comparison_eq(&mut self) -> Result<()>;
  fn action_comparison_ge(&mut self) -> Result<()>;
  fn action_comparison_gt(&mut self) -> Result<()>;
  fn action_comparison_in(&mut self) -> Result<()>;
  fn action_comparison_le(&mut self) -> Result<()>;
  fn action_comparison_lt(&mut self) -> Result<()>;
  fn action_comparison_nq(&mut self) -> Result<()>;
  fn action_comparison_unary_ge(&mut self) -> Result<()>;
  fn action_comparison_unary_gt(&mut self) -> Result<()>;
  fn action_comparison_unary_le(&mut self) -> Result<()>;
  fn action_comparison_unary_lt(&mut self) -> Result<()>;
  fn action_conjunction(&mut self) -> Result<()>;
  fn action_context_begin(&mut self) -> Result<()>;
  fn action_context_end(&mut self) -> Result<()>;
  fn action_context_entry(&mut self) -> Result<()>;
  fn action_context_entry_tail(&mut self) -> Result<()>;
  fn action_context_type_entry(&mut self) -> Result<()>;
  fn action_context_type_entry_tail(&mut self) -> Result<()>;
  fn action_disjunction(&mut self) -> Result<()>;
  fn action_division(&mut self) -> Result<()>;
  fn action_empty_context(&mut self) -> Result<()>;
  fn action_every(&mut self) -> Result<()>;
  fn action_every_begin(&mut self) -> Result<()>;
  fn action_exponentiation(&mut self) -> Result<()>;
  fn action_expression_list_tail(&mut self) -> Result<()>;
  fn action_filter(&mut self) -> Result<()>;
  fn action_for(&mut self) -> Result<()>;
  fn action_for_begin(&mut self) -> Result<()>;
  fn action_formal_parameter_with_type(&mut self) -> Result<()>;
  fn action_formal_parameter_without_type(&mut self) -> Result<()>;
  fn action_formal_parameters_begin(&mut self) -> Result<()>;
  fn action_formal_parameters_empty(&mut self) -> Result<()>;
  fn action_formal_parameters_first(&mut self) -> Result<()>;
  fn action_formal_parameters_tail(&mut self) -> Result<()>;
  fn action_function_body(&mut self) -> Result<()>;
  fn action_function_body_external(&mut self) -> Result<()>;
  fn action_function_definition(&mut self) -> Result<()>;
  fn action_function_invocation(&mut self) -> Result<()>;
  fn action_function_invocation_no_parameters(&mut self) -> Result<()>;
  fn action_function_type(&mut self) -> Result<()>;
  fn action_function_type_parameters_empty(&mut self) -> Result<()>;
  fn action_function_type_parameters_tail(&mut self) -> Result<()>;
  fn action_if(&mut self) -> Result<()>;
  fn action_instance_of(&mut self) -> Result<()>;
  fn action_interval(&mut self) -> Result<()>;
  fn action_interval_end(&mut self) -> Result<()>;
  fn action_interval_start(&mut self) -> Result<()>;
  fn action_iteration_context_value_range(&mut self) -> Result<()>;
  fn action_iteration_context_value_single(&mut self) -> Result<()>;
  fn action_iteration_context_variable_name(&mut self) -> Result<()>;
  fn action_iteration_context_variable_name_begin(&mut self) -> Result<()>;
  fn action_iteration_contexts_tail(&mut self) -> Result<()>;
  fn action_key_name(&mut self) -> Result<()>;
  fn action_key_string(&mut self) -> Result<()>;
  fn action_list(&mut self) -> Result<()>;
  fn action_list_empty(&mut self) -> Result<()>;
  fn action_list_tail(&mut self) -> Result<()>;
  fn action_list_type(&mut self) -> Result<()>;
  fn action_literal_at(&mut self) -> Result<()>;
  fn action_literal_boolean(&mut self) -> Result<()>;
  fn action_literal_date_time(&mut self) -> Result<()>;
  fn action_literal_null(&mut self) -> Result<()>;
  fn action_literal_numeric(&mut self) -> Result<()>;
  fn action_literal_string(&mut self) -> Result<()>;
  fn action_multiplication(&mut self) -> Result<()>;
  fn action_name(&mut self) -> Result<()>;
  fn action_named_parameter(&mut self) -> Result<()>;
  fn action_named_parameters_tail(&mut self) -> Result<()>;
  fn action_negation(&mut self) -> Result<()>;
  fn action_path(&mut self) -> Result<()>;
  fn action_path_names(&mut self) -> Result<()>;
  fn action_positional_parameters_tail(&mut self) -> Result<()>;
  fn action_qualified_name(&mut self) -> Result<()>;
  fn action_qualified_name_tail(&mut self) -> Result<()>;
  fn action_quantified_expression(&mut self) -> Result<()>;
  fn action_quantified_expression_variable_name(&mut self) -> Result<()>;
  fn action_quantified_expression_variable_name_begin(&mut self) -> Result<()>;
  fn action_quantified_expressions_tail(&mut self) -> Result<()>;
  fn action_range_type(&mut self) -> Result<()>;
  fn action_some(&mut self) -> Result<()>;
  fn action_some_begin(&mut self) -> Result<()>;
  fn action_subtraction(&mut self) -> Result<()>;
  fn action_type_name(&mut self) -> Result<()>;
  fn action_unary_tests_begin(&mut self) -> Result<()>;
  fn action_unary_tests_irrelevant(&mut self) -> Result<()>;
  fn action_unary_tests_negated(&mut self) -> Result<()>;
}

/// Calls requested reduce action.
pub fn reduce(reduce_actions: &mut impl ReduceActions, rule_number: i16) -> Result<()> {
  match rule_number {
    7 => reduce_actions.action_unary_tests_begin(),                           // $@1: %empty
    14 => reduce_actions.action_for_begin(),                                  // $@2: %empty
    15 => reduce_actions.action_for(),                                        // textual_expression: FOR $@2 iteration_contexts RETURN expression
    16 => reduce_actions.action_if(),                                         // textual_expression: IF expression THEN expression ELSE expression
    17 => reduce_actions.action_some_begin(),                                 // $@3: %empty
    18 => reduce_actions.action_some(),                                       // textual_expression: SOME $@3 quantified_expressions SATISFIES expression
    19 => reduce_actions.action_every_begin(),                                // $@4: %empty
    20 => reduce_actions.action_every(),                                      // textual_expression: EVERY $@4 quantified_expressions SATISFIES expression
    21 => reduce_actions.action_between_begin(),                              // $@5: %empty
    22 => reduce_actions.action_between(),                                    // textual_expression: expression BETWEEN $@5 expression BETWEEN_AND expression
    23 => reduce_actions.action_disjunction(),                                // textual_expression: expression OR expression
    24 => reduce_actions.action_conjunction(),                                // textual_expression: expression AND expression
    25 => reduce_actions.action_comparison_eq(),                              // textual_expression: expression EQ expression
    26 => reduce_actions.action_comparison_nq(),                              // textual_expression: expression NQ expression
    27 => reduce_actions.action_comparison_lt(),                              // textual_expression: expression LT expression
    28 => reduce_actions.action_comparison_le(),                              // textual_expression: expression LE expression
    29 => reduce_actions.action_comparison_gt(),                              // textual_expression: expression GT expression
    30 => reduce_actions.action_comparison_ge(),                              // textual_expression: expression GE expression
    31 => reduce_actions.action_comparison_in(),                              // textual_expression: expression IN LEFT_PAREN comparison_in
    32 => reduce_actions.action_comparison_in(),                              // textual_expression: expression IN expression
    33 => reduce_actions.action_addition(),                                   // textual_expression: expression PLUS expression
    34 => reduce_actions.action_subtraction(),                                // textual_expression: expression MINUS expression
    35 => reduce_actions.action_multiplication(),                             // textual_expression: expression MUL expression
    36 => reduce_actions.action_division(),                                   // textual_expression: expression DIV expression
    37 => reduce_actions.action_exponentiation(),                             // textual_expression: expression EXP expression
    38 => reduce_actions.action_negation(),                                   // textual_expression: MINUS expression
    39 => reduce_actions.action_type_name(),                                  // $@6: %empty
    40 => reduce_actions.action_instance_of(),                                // textual_expression: expression INSTANCE OF $@6 type
    41 => reduce_actions.action_path_names(),                                 // textual_expression: NAME DOT NAME
    42 => reduce_actions.action_path(),                                       // textual_expression: expression DOT NAME
    43 => reduce_actions.action_filter(),                                     // textual_expression: expression LEFT_BRACKET expression RIGHT_BRACKET
    47 => reduce_actions.action_name(),                                       // textual_expression: NAME
    49 => reduce_actions.action_expression_list_tail(),                       // textual_expressions: textual_expression COMMA textual_expressions
    50 => reduce_actions.action_expression_list_tail(),                       // textual_expressions: textual_expression
    51 => reduce_actions.action_unary_tests_irrelevant(),                     // unary_tests: MINUS
    52 => reduce_actions.action_unary_tests_negated(),                        // unary_tests: NOT LEFT_PAREN positive_unary_tests RIGHT_PAREN
    54 => reduce_actions.action_expression_list_tail(),                       // positive_unary_tests: expression COMMA positive_unary_tests
    55 => reduce_actions.action_expression_list_tail(),                       // positive_unary_tests: expression
    56 => reduce_actions.action_expression_list_tail(),                       // comparison_in: expression COMMA positive_unary_tests RIGHT_PAREN
    57 => reduce_actions.action_comparison_unary_lt(),                        // simple_positive_unary_test: LT endpoint
    58 => reduce_actions.action_comparison_unary_le(),                        // simple_positive_unary_test: LE endpoint
    59 => reduce_actions.action_comparison_unary_gt(),                        // simple_positive_unary_test: GT endpoint
    60 => reduce_actions.action_comparison_unary_ge(),                        // simple_positive_unary_test: GE endpoint
    62 => reduce_actions.action_interval(),                                   // interval: interval_start interval_end
    63 => reduce_actions.action_interval_start(),                             // interval_start: LEFT_PAREN endpoint ELLIPSIS
    64 => reduce_actions.action_interval_start(),                             // interval_start: RIGHT_BRACKET endpoint ELLIPSIS
    65 => reduce_actions.action_interval_start(),                             // interval_start: LEFT_BRACKET endpoint ELLIPSIS
    66 => reduce_actions.action_interval_end(),                               // interval_end: endpoint RIGHT_PAREN
    67 => reduce_actions.action_interval_end(),                               // interval_end: endpoint LEFT_BRACKET
    68 => reduce_actions.action_interval_end(),                               // interval_end: endpoint RIGHT_BRACKET
    73 => reduce_actions.action_literal_null(),                               // literal: NULL
    74 => reduce_actions.action_literal_numeric(),                            // simple_literal: NUMERIC
    75 => reduce_actions.action_literal_string(),                             // simple_literal: STRING
    76 => reduce_actions.action_literal_boolean(),                            // simple_literal: BOOLEAN
    77 => reduce_actions.action_literal_at(),                                 // simple_literal: AT STRING
    78 => reduce_actions.action_literal_date_time(),                          // $@7: %empty
    80 => reduce_actions.action_context_begin(),                              // $@8: %empty
    81 => reduce_actions.action_context_end(),                                // context: LEFT_BRACE $@8 context_entries
    82 => reduce_actions.action_empty_context(),                              // context_entries: RIGHT_BRACE
    83 => reduce_actions.action_context_entry_tail(),                         // context_entries: context_entry context_entry_tail
    84 => reduce_actions.action_context_entry(),                              // context_entry: key COLON expression
    86 => reduce_actions.action_context_entry_tail(),                         // context_entry_tail: COMMA context_entry context_entry_tail
    87 => reduce_actions.action_key_name(),                                   // key: NAME
    88 => reduce_actions.action_key_string(),                                 // key: STRING
    89 => reduce_actions.action_list(),                                       // list: LEFT_BRACKET list_items
    90 => reduce_actions.action_list_empty(),                                 // list_items: RIGHT_BRACKET
    91 => reduce_actions.action_list_tail(),                                  // list_items: expression list_tail
    93 => reduce_actions.action_list_tail(),                                  // list_tail: COMMA expression list_tail
    94 => reduce_actions.action_function_invocation_no_parameters(),          // parameters: RIGHT_PAREN
    95 => reduce_actions.action_function_invocation(),                        // parameters: named_parameters
    96 => reduce_actions.action_function_invocation(),                        // parameters: positional_parameters
    97 => reduce_actions.action_named_parameters_tail(),                      // named_parameters: named_parameter named_parameters_tail
    98 => reduce_actions.action_named_parameter(),                            // named_parameter: NAME COLON expression
    100 => reduce_actions.action_named_parameters_tail(),                     // named_parameters_tail: COMMA named_parameter named_parameters_tail
    101 => reduce_actions.action_positional_parameters_tail(),                // positional_parameters: expression positional_parameters_tail
    103 => reduce_actions.action_positional_parameters_tail(),                // positional_parameters_tail: COMMA expression positional_parameters_tail
    104 => reduce_actions.action_qualified_name_tail(),                       // qualified_name: NAME DOT qualified_name
    105 => reduce_actions.action_qualified_name(),                            // qualified_name: NAME
    106 => reduce_actions.action_built_in_type_name(),                        // type: BUILT_IN_TYPE_NAME
    108 => reduce_actions.action_type_name(),                                 // $@9: %empty
    109 => reduce_actions.action_list_type(),                                 // type: LIST LT $@9 type GT
    110 => reduce_actions.action_type_name(),                                 // $@10: %empty
    111 => reduce_actions.action_range_type(),                                // type: RANGE LT $@10 type GT
    113 => reduce_actions.action_type_name(),                                 // $@11: %empty
    114 => reduce_actions.action_function_type(),                             // type: FUNCTION LT function_type_parameters RIGHT_ARROW $@11 type
    115 => reduce_actions.action_context_type_entry_tail(),                   // context_type_entries: context_type_entry context_type_entry_tail
    116 => reduce_actions.action_type_name(),                                 // $@12: %empty
    117 => reduce_actions.action_context_type_entry(),                        // context_type_entry: NAME COLON $@12 type
    119 => reduce_actions.action_context_type_entry_tail(),                   // context_type_entry_tail: COMMA context_type_entry context_type_entry_tail
    120 => reduce_actions.action_function_type_parameters_empty(),            // function_type_parameters: GT
    121 => reduce_actions.action_type_name(),                                 // $@13: %empty
    122 => reduce_actions.action_function_type_parameters_tail(),             // function_type_parameters: $@13 type function_type_parameters_tail
    124 => reduce_actions.action_type_name(),                                 // $@14: %empty
    125 => reduce_actions.action_function_type_parameters_tail(),             // function_type_parameters_tail: COMMA $@14 type function_type_parameters_tail
    126 => reduce_actions.action_iteration_contexts_tail(),                   // iteration_contexts: iteration_context COMMA iteration_contexts
    127 => reduce_actions.action_iteration_contexts_tail(),                   // iteration_contexts: iteration_context
    128 => reduce_actions.action_iteration_context_variable_name_begin(),     // $@15: %empty
    129 => reduce_actions.action_iteration_context_variable_name(),           // $@16: %empty
    131 => reduce_actions.action_iteration_context_value_single(),            // iteration_context_value: expression
    132 => reduce_actions.action_iteration_context_value_range(),             // iteration_context_value: expression ELLIPSIS expression
    133 => reduce_actions.action_quantified_expressions_tail(),               // quantified_expressions: quantified_expression COMMA quantified_expressions
    134 => reduce_actions.action_quantified_expressions_tail(),               // quantified_expressions: quantified_expression
    135 => reduce_actions.action_quantified_expression_variable_name_begin(), // $@17: %empty
    136 => reduce_actions.action_quantified_expression_variable_name(),       // $@18: %empty
    137 => reduce_actions.action_quantified_expression(),                     // quantified_expression: $@17 NAME $@18 IN expression
    138 => reduce_actions.action_formal_parameters_begin(),                   // $@19: %empty
    139 => reduce_actions.action_function_definition(),                       // function_definition: FUNCTION LEFT_PAREN $@19 formal_parameters external
    140 => reduce_actions.action_formal_parameters_empty(),                   // formal_parameters: RIGHT_PAREN
    141 => reduce_actions.action_formal_parameters_first(),                   // $@20: %empty
    144 => reduce_actions.action_formal_parameters_tail(),                    // $@21: %empty
    146 => reduce_actions.action_type_name(),                                 // $@22: %empty
    147 => reduce_actions.action_formal_parameter_with_type(),                // formal_parameter: NAME COLON $@22 type
    148 => reduce_actions.action_formal_parameter_without_type(),             // formal_parameter: NAME
    149 => reduce_actions.action_function_body_external(),                    // external: EXTERNAL expression
    150 => reduce_actions.action_function_body(),                             // external: expression
    _ => Ok(()),
  }
}
