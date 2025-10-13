use clap::Parser;
use sbpf_linker::{SbpfLinkerError, link_program};
use std::fs;
use std::path::{Path, PathBuf};

/// Links an object file by reading it from the given path and processing its bytecode
fn link_object_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, SbpfLinkerError> {
    // Read the object file into a byte array
    let bytes = fs::read(path.as_ref())?;

    // Call link_program on the bytes
    link_program(&bytes)
}

#[derive(Debug, Parser)]
#[command(
    name = "sbpf-link",
    version,
    about = "Simple SBPF linker that processes object files directly"
)]
struct Args {
    /// Input object file to link
    #[clap(value_name = "INPUT")]
    input: PathBuf,
}

fn main() -> Result<(), SbpfLinkerError> {
    let args = Args::parse();

    // Link the object file
    println!("Linking: {}", args.input.display());
    let linked_bytecode = link_object_file(&args.input)?;

    // Determine output path in same directory with .so extension
    let parent = args.input.parent().unwrap_or_else(|| Path::new("."));
    let stem = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output = parent.join(format!("{}.so", stem));

    // Write the output
    println!("Writing output to: {}", output.display());
    std::fs::write(&output, &linked_bytecode)?;

    println!("Successfully linked {} bytes", linked_bytecode.len());
    Ok(())
}
