use sbpf_assembler::ast::AST;
use sbpf_assembler::astnode::{ASTNode, ROData};
use sbpf_assembler::instruction::Instruction;
use sbpf_assembler::lexer::{ImmediateValue, Token};
use sbpf_assembler::opcode::Opcode;
use sbpf_assembler::parser::ParseResult;

use object::RelocationTarget::Symbol;
use object::{File, Object as _, ObjectSection as _, ObjectSymbol as _};

use std::collections::HashMap;

use crate::SbpfLinkerError;

/// Parses the given bytecode and constructs an Abstract Syntax Tree (AST) representation
/// of the program. The function processes the `.rodata` and `.text` sections of the bytecode,
/// extracting relevant symbols, relocations, and instructions.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes representing the bytecode to be parsed.
///
/// # Returns
///
/// Returns a `Result` containing the `ParseResult` on success, or an `SbpfLinkerError` on failure.
///
/// # Errors
///
/// This function may return the following errors:
/// * `SbpfLinkerError::BuildProgramError` - If there are errors while building the program.
/// * Errors from the `object` crate when parsing the bytecode or accessing sections and symbols.
///
/// # Details
///
/// 1. **`.rodata` Section**:
///    - Extracts symbols from the `.rodata` section and constructs `ROData` nodes for the AST.
///    - Builds a `rodata_table` mapping symbol addresses to their names.
///    - Updates the AST with the size of the `.rodata` section.
///
/// 2. **`.text` Section**:
///    - Parses the `.text` section to extract instructions and builds `Instruction` nodes for the AST.
///    - Handles relocations for symbols in the `.rodata` section:
///        - Resolves the relocation target symbol.
///        - Replaces the immediate value in the instruction with the corresponding `.rodata` label.
///
/// 3. **Program Construction**:
///    - Constructs the final program from the AST and returns the result.
///
/// # Example
///
/// ```rust
/// use sbpf_linker::byteparser::parse_bytecode;
/// 
/// let bytecode: &[u8] = &[0, 1, 2, 3, 4, 5]; // Example bytecode
/// match parse_bytecode(bytecode) {
///     Ok(parse_result) => {
///         // Use the parse result
///     }
///     Err(err) => {
///         eprintln!("Error parsing bytecode: {:?}", err);
///     }
/// }
/// ```
pub fn parse_bytecode(bytes: &[u8]) -> Result<ParseResult, SbpfLinkerError> {
    let mut ast = AST::new();

    let obj = File::parse(bytes)?;
    let mut rodata_table = HashMap::new();
    if let Some(ro_section) = obj.section_by_name(".rodata") {
        // only handle symbols in the .rodata section for now
        let mut rodata_offset = 0;
        for symbol in obj.symbols() {
            if symbol.section_index() == Some(ro_section.index())
                && symbol.size() > 0
            {
                let mut bytes = Vec::new();
                for i in 0..symbol.size() {
                    bytes.push(ImmediateValue::Int(i64::from(
                        ro_section.data().unwrap()
                            [(symbol.address() + i) as usize],
                    )));
                }
                ast.rodata_nodes.push(ASTNode::ROData {
                    rodata: ROData {
                        name: symbol.name().unwrap().to_owned(),
                        args: vec![
                            Token::Directive(String::from("byte"), 0..1), //
                            Token::VectorLiteral(bytes.clone(), 0..1),
                        ],
                        span: 0..1,
                    },
                    offset: rodata_offset,
                });
                rodata_table.insert(
                    symbol.address(),
                    symbol.name().unwrap().to_owned(),
                );
                rodata_offset += symbol.size();
            }
        }
        ast.set_rodata_size(rodata_offset);
    }

    for section in obj.sections() {
        if section.name() == Ok(".text") {
            // parse text section and build instruction nodes
            // lddw takes 16 bytes, other instructions take 8 bytes
            let mut offset = 0;
            while offset < section.data().unwrap().len() {
                let node_len =
                    match Opcode::from_u8(section.data().unwrap()[offset]) {
                        Some(Opcode::Lddw) => 16,
                        _ => 8,
                    };
                let node = &section.data().unwrap()[offset..offset + node_len];
                ast.nodes.push(ASTNode::Instruction {
                    instruction: Instruction::from_bytes(node).unwrap(),
                    offset: offset as u64,
                });
                offset += node_len;
            }

            if let Some(ro_section) = obj.section_by_name(".rodata") {
                // handle relocations
                for rel in section.relocations() {
                    // only handle relocations for symbols in the .rodata section for now
                    let symbol = match rel.1.target() {
                        Symbol(sym) => Some(obj.symbol_by_index(sym).unwrap()),
                        _ => None,
                    };
                    println!("Symbol: {symbol:?}");

                    if symbol.unwrap().section_index()
                        == Some(ro_section.index())
                    {
                        println!("Relocation found");
                        // addend is not explicit in the relocation entry, but implicitly encoded
                        // as the immediate value of the instruction
                        let addend = match ast
                            .get_instruction_at_offset(rel.0)
                            .unwrap()
                            .operands
                            .last()
                            .unwrap()
                            .clone()
                        {
                            Token::ImmediateValue(
                                ImmediateValue::Int(val),
                                _,
                            ) => val,
                            _ => 0,
                        };

                        // Replace the immediate value with the rodata label
                        let ro_label = &rodata_table[&(addend as u64)];
                        let ro_label_name = ro_label.clone();
                        let node: &mut Instruction =
                            ast.get_instruction_at_offset(rel.0).unwrap();
                        let last_idx = node.operands.len() - 1;
                        node.operands[last_idx] =
                            Token::Identifier(ro_label_name, 0..1);
                    }
                }
            }
            ast.set_text_size(section.size());
        }
    }

    ast.build_program()
        .map_err(|errors| SbpfLinkerError::BuildProgramError { errors })
}
