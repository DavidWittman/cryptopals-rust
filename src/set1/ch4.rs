/* Detect single-character XOR

One of the 60-character strings in this file has been encrypted by single-character XOR.

Find it.

(Your code from #3 should help.)

*/

use std::fs::File;
use std::io::{self, BufRead};

use hex;

use crate::set1::ch3;

pub fn find_single_xor_in_file(filename: String) -> Result<(String, u8), io::Error> {
    let file = File::open(filename)?;
    let mut top_score = 0;
    let mut line: String;
    let mut the_line = String::from("");
    let mut the_key = 0x00u8;

    for wrapped_line in io::BufReader::new(file).lines() {
        line = wrapped_line.unwrap();
        let key = ch3::find_xor_byte_from_string(line.clone());
        let bytes = hex::decode(&line).unwrap();
        let decoded = ch3::xor_bytes(&bytes, key);
        let score = ch3::get_score(decoded);
        if score > top_score {
            top_score = score;
            the_line = line;
            the_key = key;
        }
    }
    return Ok((String::from(the_line), the_key));
}

#[test]
pub fn test_find_single_xor_in_file() {
    let (line, key) = find_single_xor_in_file(String::from("./src/set1/testdata/4.txt")).unwrap();
    assert_eq!(line, "7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f");
    assert_eq!(key, 0x35u8);

    let result = String::from_utf8(ch3::xor_bytes(&hex::decode(line).unwrap(), key)).unwrap();
    assert_eq!(result, "Now that the party is jumping\n");
}