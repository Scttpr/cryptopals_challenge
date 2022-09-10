use crate::tools::xor;
use crate::tools::{binary, get_chars_as_bytes, BASIC_ASCII_CHARS};

pub fn resolve() -> Vec<String> {
    let input = get_input();
    let key_sizes: Vec<usize> = (2..41).collect();

    let best_key_sizes = get_best_key_sizes(&input, &key_sizes);

    best_key_sizes
        .iter()
        .map(|key_size| {
            let slices = get_n_first_slices(None, &input, *key_size);
            let transposed_slices = transpose_slices(&slices);

            let key: Vec<u8> = transposed_slices.iter().fold(vec![], |mut key, slice| {
                let result = xor::single_byte_decode(slice, get_chars_as_bytes(BASIC_ASCII_CHARS));

                if let Some(key_slice) = result.key {
                    key.push(key_slice[0]);
                }

                key
            });

            String::from_utf8_lossy(&key).to_string()
        })
        .collect()
}

fn get_input() -> Vec<u8> {
    let raw = std::fs::read_to_string("6.txt").expect("Unable to read file");
    base64::decode(&raw.replace('\n', "")).unwrap()
}

fn get_best_key_sizes(bytes: &[u8], key_sizes: &[usize]) -> Vec<usize> {
    key_sizes
        .iter()
        .fold(
            (vec![], usize::MAX),
            |(mut best_key_sizes, lowest_hamming_distance), key_size| {
                let slices = get_n_first_slices(Some(5), bytes, *key_size);
                let average_hamming_distance = get_average_hamming_distance(&slices) / key_size;

                match average_hamming_distance {
                    v if v < lowest_hamming_distance => (vec![*key_size], average_hamming_distance),
                    v if v == lowest_hamming_distance => {
                        best_key_sizes.push(*key_size);
                        (best_key_sizes, lowest_hamming_distance)
                    }
                    _ => (best_key_sizes, lowest_hamming_distance),
                }
            },
        )
        .0
}

fn get_n_first_slices(n: Option<usize>, bytes: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    let slices = get_key_size_slices(bytes, key_size);

    match n {
        Some(v) => Vec::from(&slices[0..v]),
        None => slices,
    }
}

fn get_key_size_slices(bytes: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    bytes
        .chunks(key_size)
        .map(std::convert::Into::into)
        .collect()
}

fn get_average_hamming_distance(slices: &[Vec<u8>]) -> usize {
    let sum = slices.iter().enumerate().fold(0, |sum, (index, slice)| {
        if index == 0 {
            return sum;
        }

        sum + get_hamming_distance(&slices[index - 1], slice)
    });

    sum / (slices.len() - 1)
}

fn get_hamming_distance(a: &[u8], b: &Vec<u8>) -> usize {
    assert_eq!(a.len(), b.len(), "Strings must have equal length");

    let a_as_bynary = binary::from_bytes(a);
    let b_as_bynary = binary::from_bytes(b);

    a_as_bynary
        .iter()
        .enumerate()
        .fold(0, |total, (index, bit)| {
            if index >= b_as_bynary.len() || *bit == b_as_bynary[index] {
                return total;
            }

            total + 1
        })
}

fn transpose_slices(slices: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let blocks_number = slices[0].len();

    (0..blocks_number)
        .map(|index| {
            slices.iter().fold(vec![], |mut acc, slice| {
                if index >= slice.len() {
                    return acc;
                }

                acc.push(slice[index]);

                acc
            })
        })
        .collect()
}
