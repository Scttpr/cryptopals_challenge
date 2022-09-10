use crate::tools::xor::SingleByteDecodeResult;
use crate::tools::{get_chars_as_bytes, xor, BASIC_ASCII_CHARS};

pub fn resolve() -> Option<String> {
    let raw_input: String =
        std::fs::read_to_string("../../data/set_1__challenge_4.txt").expect("Unable to read file");

    let input: Vec<&str> = raw_input.split('\n').collect();
    let chars = get_chars_as_bytes(BASIC_ASCII_CHARS);

    let mut output: Vec<SingleByteDecodeResult> = input
        .iter()
        .map(|line| {
            let bytes = hex::decode(line).unwrap();
            xor::single_byte_decode(&bytes, chars.clone())
        })
        .collect();

    output.sort_by(|a, b| b.score.cmp(&a.score));

    output[0].message.clone()
}
