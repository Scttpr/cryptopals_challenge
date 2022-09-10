pub fn from_bytes(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .flat_map(|byte| {
            format!("{byte:08b}")
                .split("")
                .filter_map(|bit| bit.parse().ok())
                .collect::<Vec<u8>>()
        })
        .collect()
}
