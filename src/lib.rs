pub mod byteparser;
use std::io;

use bpf_linker::LinkerError;
use byteparser::parse_bytecode;

use sbpf_assembler::{CompileError, Program};

#[derive(thiserror::Error, Debug)]
pub enum SbpfLinkerError {
    #[error("Error opening object file. Error detail: ({0}).")]
    ObjectFileOpenError(#[from] object::Error),
    #[error("Error reading object file. Error detail: ({0}).")]
    ObjectFileReadError(#[from] io::Error),
    #[error("Linker Error. Error detail: ({0}).")]
    LinkerError(#[from] LinkerError),
    #[error("LLVM issued diagnostic with error severity.")]
    LlvmDiagnosticError,
    #[error("Build Program Error. Error details: {errors:?}.")]
    BuildProgramError { errors: Vec<CompileError> },
}

/// Links an SBPF program from the given source bytecode.
///
/// # Arguments
///
/// * `source` - A byte slice containing the source bytecode to be linked.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - A vector of bytes representing the linked program's bytecode.
/// * `Err(SbpfLinkerError)` - An error that occurred during the linking process.
///
/// # Errors
///
/// This function can return the following errors:
///
/// * `SbpfLinkerError::ObjectFileOpenError` - If opening the object file fails.
/// * `SbpfLinkerError::ObjectFileReadError` - If reading the object file fails.
/// * `SbpfLinkerError::LinkerError` - If an error occurs during the linking process.
/// * `SbpfLinkerError::LlvmDiagnosticError` - If LLVM issues a diagnostic with error severity.
/// * `SbpfLinkerError::BuildProgramError` - If building the program fails.
pub fn link_program(source: &[u8]) -> Result<Vec<u8>, SbpfLinkerError> {
    let parse_result = parse_bytecode(source)?;
    let program = Program::from_parse_result(parse_result);
    let bytecode = program.emit_bytecode();

    Ok(bytecode)
}
