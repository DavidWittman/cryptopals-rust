/* Detect AES in ECB mode

In [this file](./testdata/8.txt) are a bunch of hex-encoded ciphertexts.

One of them has been encrypted with ECB.

Detect it.

Remember that the problem with ECB is that it is stateless and deterministic;
the same 16 byte plaintext block will always produce the same 16 byte ciphertext.
*/

use std::fs::File;
use std::io::{self, BufRead};

use hex;

fn find_ecb_in_file(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    let mut top_score = 0;
    let mut top_line = String::from("");

    for wrapped_line in io::BufReader::new(file).lines() {
        let line = wrapped_line?;
        let cipher = hex::decode(&line).unwrap();
        let score = ecb_score(&cipher, 16usize);
        if score > top_score {
            top_score = score;
            top_line = line;
        }
    }    

    Ok(top_line)
}

#[test]
fn test_find_ecb_in_file() {
    let result = find_ecb_in_file("./src/set1/testdata/8.txt").unwrap();
    assert_eq!(result, String::from("d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9\
d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93ea\
b8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba1\
86a"));
}

pub fn ecb_score(cipher: &Vec<u8>, block_size: usize) -> u32 {
    if (cipher.len() % block_size) != 0 {
        panic!("ecb_score: cipher is not a multiple of block_size");
    }

    let mut seen = Vec::new();
    let mut score = 0;

    for chunk in cipher.chunks(block_size) {
        if seen.contains(&chunk) {
            score += 1;
        } else {
            seen.push(chunk);
        }
    }

    score
}

#[test]
fn test_ecb_score() {
    let test = "YELLOW SUBMARINEYELLOW SUBMARINEYELLOW SUBMARINEYELLOW SUBMARINE".as_bytes().to_vec();
    assert_eq!(ecb_score(&test, 16usize), 3);
}

#[test]
#[should_panic(expected = "cipher is not a multiple of block_size")]
fn test_ecb_score_panic() {
    ecb_score(&vec![0u8], 16);
}