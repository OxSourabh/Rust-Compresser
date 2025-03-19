use std::fs::File;
use std::io::{self, Read, Write};

/// Compresses a string using Run-Length Encoding (RLE).
/// For example: "aaabbbcc" becomes "a3b3c2".
fn compress(input: &str) -> String {
    let mut compressed_result = String::new();
    let mut characters = input.chars().peekable(); // Use peekable to look ahead without consuming

    while let Some(current_character) = characters.next() {
        let mut repeat_count = 1;

        // Count how many times the current character repeats consecutively
        while matches!(characters.peek(), Some(&next_char) if next_char == current_character) {
            repeat_count += 1;
            characters.next(); // Move to the next character
        }

        // Append the character and its count to the result
        compressed_result.push(current_character);
        compressed_result.push_str(&repeat_count.to_string());
    }

    compressed_result
}

/// Decompresses a string encoded with Run-Length Encoding (RLE).
/// For example: "a3b3c2" becomes "aaabbbcc".
fn decompress(encoded_data: &str) -> Result<String, &'static str> {
    let mut decompressed_result = String::new();
    let mut characters = encoded_data.chars();

    while let Some(character) = characters.next() {
        let mut count_digits = String::new();

        // Collect all consecutive digits to form the count
        while let Some(next_char) = characters.peek() {
            if next_char.is_ascii_digit() {
                count_digits.push(characters.next().unwrap()); // Add the digit to the count
            } else {
                break; // Stop collecting digits when a non-digit is encountered
            }
        }

        // Parse the collected digits into a number
        let repeat_count = count_digits.parse::<usize>().map_err(|_| "Invalid count")?;

        // Repeat the character `repeat_count` times and add it to the result
        decompressed_result.extend(std::iter::repeat(character).take(repeat_count));
    }

    Ok(decompressed_result)
}

fn main() -> io::Result<()> {
    println!("=== Compression and Decompression Program ===");

    // Step 1: Read the input file
    let mut input_file_content = String::new();
    match File::open("input.txt") {
        Ok(mut file) => file.read_to_string(&mut input_file_content)?,
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, "Input file not found")),
    };

    println!("Original Data: {}", input_file_content);

    // Step 2: Compress the data
    let compressed_data = compress(&input_file_content);
    println!("Compressed Data: {}", compressed_data);

    // Step 3: Save the compressed data to a file
    let mut compressed_file = File::create("compressed.txt")?;
    compressed_file.write_all(compressed_data.as_bytes())?;
    println!("Compressed data saved to 'compressed.txt'.");

    // Step 4: Read the compressed file
    let mut compressed_file_content = String::new();
    File::open("compressed.txt")?.read_to_string(&mut compressed_file_content)?;

    // Step 5: Decompress the data
    match decompress(&compressed_file_content) {
        Ok(decompressed_data) => {
            println!("Decompressed Data: {}", decompressed_data);
        }
        Err(error_message) => {
            eprintln!("Error during decompression: {}", error_message);
        }
    }

    println!("=== Program Finished ===");
    Ok(())
}
