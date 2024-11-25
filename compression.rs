use std::fs::File;
use std::io::{self, Read, Write};

fn compress(data: &str) -> String {
    let mut compressed = String::new();
    let mut chars = data.chars().peekable();

    while let Some(current_char) = chars.next() {
        let mut count = 1;

        while let Some(&next_char) = chars.peek() {
            if next_char == current_char {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }

        compressed.push(current_char);
        compressed.push_str(&count.to_string());
    }

    compressed
}

fn decompress(data: &str) -> String {
    let mut decompressed = String::new();
    let mut chars = data.chars();

    while let Some(char) = chars.next() {
        if let Some(count_char) = chars.next() {
            let count = count_char.to_digit(10).unwrap_or(1);
            for _ in 0..count {
                decompressed.push(char);
            }
        }
    }

    decompressed
}

fn main() -> io::Result<()> {
    // Read the input file
    let mut file = File::open("input.txt")?;
    let mut input_data = String::new();
    file.read_to_string(&mut input_data)?;

    // Compress the data
    let compressed_data = compress(&input_data);
    println!("Compressed Data: {}", compressed_data);

    // Write the compressed data to a file
    let mut compressed_file = File::create("compressed.txt")?;
    compressed_file.write_all(compressed_data.as_bytes())?;

    // Read the compressed file
    let mut compressed_data_read = String::new();
    let mut compressed_file_read = File::open("compressed.txt")?;
    compressed_file_read.read_to_string(&mut compressed_data_read)?;

    // Decompress the data
    let decompressed_data = decompress(&compressed_data_read);
    println!("Decompressed Data: {}", decompressed_data);

    Ok(())
}