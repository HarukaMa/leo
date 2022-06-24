// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

#[derive(Clone, Default)]
pub struct OutputOptions {
    /// Whether spans are enabled in the output ASTs.
    pub spans_enabled: bool,
    /// If enabled, write the AST after parsing.
    pub ast_initial: bool,
    /// If enabled, write the input AST after parsing.
    pub input_ast_initial: bool,
    /// If enabled, write the flattened AST after parsing.
    pub flattened_ast: bool,
    /// If enabled, write the AST after it has been converted to SSA form.
    pub ssa_ast: bool,
    /// If enabled, write the AST after dead code has been eliminated.
    pub dead_code_eliminated_ast: bool,
}
