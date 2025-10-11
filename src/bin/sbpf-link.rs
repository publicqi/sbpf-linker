use std::path::{Path, PathBuf};
use std::fs;
use clap::Parser;
use sbpf_linker::link_program;

/// Links an object file by reading it from the given path and processing its bytecode
fn link_object_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, String> {
    // Read the object file into a byte array
    let bytes = fs::read(path.as_ref())
        .map_err(|e| format!("Failed to read object file: {}", e))?;
    
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Link the object file
    println!("Linking: {}", args.input.display());
    let linked_bytecode = link_object_file(&args.input)
        .map_err(|e| format!("Failed to link object file: {}", e))?;

    // Determine output path in same directory with .so extension
    let parent = args.input.parent().unwrap_or_else(|| Path::new("."));
    let stem = args.input.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output = parent.join(format!("{}.so", stem));

    // Write the output
    println!("Writing output to: {}", output.display());
    std::fs::write(&output, &linked_bytecode)?;

    println!("Successfully linked {} bytes", linked_bytecode.len());
    Ok(())
}

