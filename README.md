# Rust-Compresser
A simple text compression tool implemented in Rust that uses run-length encoding (RLE) to compress and decompress text files.
The program uses run-length encoding, which replaces sequences of repeated characters with the character followed by its count. For example:

- "AABBBCCCC" becomes "A2B3C4"
- "WWWWWWWWWWW" becomes "W11"

## Installation

Make sure you have Rust installed on your system. You can install Rust from [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/OxSourabh/Rust-Compresser/
cd [Rust-Compresser]

# Build the project
cargo build --release
```

## Usage

1. Place your input text file named `input.txt` in the project directory
2. Run the program:
```bash
cargo run
```

The program will:
1. Read the contents of `input.txt`
2. Compress the data using RLE
3. Save the compressed data to `compressed.txt`
4. Read and decompress the data
5. Display both compressed and decompressed output

## Functions

### `compress(data: &str) -> String`
Compresses the input string using run-length encoding.
- Input: Reference to a string slice containing the text to compress
- Output: Compressed string where each character is followed by its count

### `decompress(data: &str) -> String`
Decompresses an RLE-encoded string back to its original form.
- Input: Reference to a string slice containing the compressed text
- Output: Decompressed string with repeated characters expanded



## Contributing

Feel free to submit issues and pull requests for:
- Multi-digit run length support
- Better compression for varied text
- Additional file format support
- Performance improvements

## License

This project is open source and available under the MIT License.
