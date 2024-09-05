
use std::fs::File;
use std::io::{self, Read, Write};
use flate2::write::GzEncoder;
use flate2::Compression;

// Function to compress a file
pub fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    // Open the input file for reading
    let mut input_file = File::open(input_path)?;

    // Create the output file for writing the compressed data
    let output_file = File::create(output_path)?;

    // Create a GzEncoder with the specified compression level
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // Buffer to read data from the input file
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    // Write the compressed data to the output file
    encoder.write_all(&buffer)?;

    // Finish writing and flush the encoder
    encoder.finish()?;

    Ok(())
}