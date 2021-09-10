/* Single-byte XOR cipher

The hex encoded string:

1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
Achievement Unlocked

You now have our permission to make "ETAOIN SHRDLU" jokes on Twitter.
*/

use hex;

pub fn xor_bytes(bytes: &Vec<u8>, key: u8) -> Vec<u8> {
    bytes.iter().map(|x| x^key).collect()
}

pub fn find_xor_byte(cipher: Vec<u8>) -> u8 {
    let mut top_score = 0;
    let mut top_byte = 0x00u8;

    for key in 0x00u8 ..= 0xFFu8 {
        let score = get_score(xor_bytes(&cipher, key));
        if score > top_score {
            top_score = score;
            top_byte = key;
        }
    }
    top_byte
}

pub fn find_xor_byte_from_string(cipher: String) -> u8 {
    let bytes = hex::decode(cipher).unwrap();
    return find_xor_byte(bytes);
}

pub fn get_score(bytes: Vec<u8>) -> u32 {
    let mut score = 0;
    for c in bytes.iter() {
        if c >= &0x41u8 && c <= &0x7au8 || *c == ' ' as u8 {
            score += 1;
        }
    }
    score
}

#[test]
fn test_find_xor_byte_from_string() {
    let cipher = String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let bytes = hex::decode(&cipher).unwrap();

    let key = find_xor_byte_from_string(cipher);
    assert_eq!(key, 0x58);

    let result = String::from_utf8(xor_bytes(&bytes, key)).expect("Invalid bytes");
    assert_eq!(result, "Cooking MC's like a pound of bacon");
}

#[test]
fn test_xor_bytes() {
    let result = xor_bytes(&vec![0x03, 0x03, 0x03], 0x03);
    assert_eq!(result, vec![0x00, 0x00, 0x00])
}

#[test]
fn test_get_score() {
    assert_eq!(get_score(String::from("foo").into_bytes()), 3);
    assert_eq!(get_score(String::from("abc").into_bytes()), 3);
    assert_eq!(get_score(String::from("f o o!").into_bytes()), 5);
    assert_eq!(get_score(vec![0xffu8, 0x01u8, 0x00u8]), 0);
}