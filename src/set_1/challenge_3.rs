use crate::tools::{get_chars_as_bytes, xor, BASIC_ASCII_CHARS};

pub fn resolve() -> Option<String> {
    let input = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
        .unwrap();
    let chars = get_chars_as_bytes(BASIC_ASCII_CHARS);

    let result = xor::single_byte_decode(&input, chars);

    result.message
}
