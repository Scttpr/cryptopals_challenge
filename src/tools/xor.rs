use crate::tools::get_score;

pub fn encrypt(bytes: &[u8], key: &Vec<u8>) -> Vec<u8> {
    bytes
        .iter()
        .enumerate()
        .map(|(index, byte)| byte ^ key[index % key.len()])
        .collect()
}

pub struct SingleByteDecodeResult {
    pub score: isize,
    pub key: Option<Vec<u8>>,
    pub message: Option<String>,
}

impl SingleByteDecodeResult {
    const fn init_empty() -> Self {
        Self {
            score: 0,
            key: None,
            message: None,
        }
    }
}

pub fn single_byte_decode(input: &[u8], chars: Vec<Vec<u8>>) -> SingleByteDecodeResult {
    chars
        .iter()
        .fold(SingleByteDecodeResult::init_empty(), |result, char| {
            let xored_input = encrypt(input, char);
            let message = String::from_utf8_lossy(&xored_input).to_string();
            let score = get_score(&message);

            if score > result.score {
                return SingleByteDecodeResult {
                    score,
                    key: Some(char.clone()),
                    message: Some(message),
                };
            }

            result
        })
}
