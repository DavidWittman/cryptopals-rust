use base64;
use hex;

pub fn hex_to_base64(s: String) -> String {
    let bytes = hex::decode(s).unwrap();
    base64::encode(bytes)
}

#[test]
fn test_hex_to_b64() {
    let result = hex_to_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}