use crate::tools::xor;

pub fn resolve() -> String {
    let input = "1c0111001f010100061a024b53535009181c";
    let key = "686974207468652062756c6c277320657965";

    let input_as_bytes = hex::decode(input).unwrap();
    let key_as_bytes = hex::decode(key).unwrap();

    hex::encode(xor::encrypt(&input_as_bytes, &key_as_bytes))
}
