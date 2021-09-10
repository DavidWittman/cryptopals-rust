/*

Fixed XOR

Write a function that takes two equal-length buffers and produces their XOR combination.

If your function works properly, then when you feed it the string:

1c0111001f010100061a024b53535009181c

... after hex decoding, and when XOR'd against:

686974207468652062756c6c277320657965

... should produce:

746865206b696420646f6e277420706c6179

*/

use hex;

pub fn xor_strings(a: String, b: String) -> String {
    if a.len() != b.len() {
        panic!("string lengths do not match")
    }
    let bytes_a = hex::decode(a).unwrap();
    let mut bytes_b = hex::decode(b).unwrap();
    for (pos, b) in bytes_a.iter().enumerate() {
        bytes_b[pos] ^= b;
    }
    hex::encode(bytes_b)
}

#[test]
fn test_xor_strings() {
    let result = xor_strings(
        String::from("1c0111001f010100061a024b53535009181c"),
        String::from("686974207468652062756c6c277320657965")
    );
    assert_eq!(result, "746865206b696420646f6e277420706c6179");
}