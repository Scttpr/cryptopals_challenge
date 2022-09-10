pub mod binary;
pub mod display;
pub mod xor;

pub const BASIC_ASCII_CHARS: &str = " !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

const fn get_letter_value(char: char) -> isize {
    match char {
        'e' => 13,
        't' => 9,
        'a' | 'o' => 8,
        'i' | 'n' => 7,
        'h' | 'r' | 's' => 6,
        'd' | 'l' => 4,
        'c' | 'u' => 3,
        'b' | 'f' | 'g' | 'm' | 'p' | 'w' | 'y' => 2,
        'j' | 'k' | 'v' => 1,
        'q' | 'x' | 'z' | ' ' => 0,
        _ => -5,
    }
}

pub fn get_chars_as_bytes(input: &str) -> Vec<Vec<u8>> {
    input
        .chars()
        .map(|char| {
            let mut buffer = [0; 1];

            char.encode_utf8(&mut buffer);

            Vec::from(buffer)
        })
        .collect()
}

pub fn get_score(input: &str) -> isize {
    let chars: Vec<char> = input.to_lowercase().chars().collect();

    chars
        .iter()
        .fold(0, |acc, char| acc + get_letter_value(*char))
}

pub fn hex_to_base64(input: &str) -> String {
    let as_bytes = hex::decode(input).unwrap();
    base64::encode(as_bytes)
}
