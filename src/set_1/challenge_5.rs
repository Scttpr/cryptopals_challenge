use crate::tools::xor;

pub fn resolve() -> String {
    let input = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"
        .to_vec();
    let key = b"ICE".to_vec();

    hex::encode(xor::encrypt(&input, &key))
}
