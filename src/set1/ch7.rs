/* AES in ECB mode

The Base64-encoded content in [this file](./testdata/7.txt) has been encrypted via AES-128 in ECB mode under the key

    "YELLOW SUBMARINE".

(case-sensitive, without the quotes; exactly 16 characters; I like "YELLOW SUBMARINE" because it's exactly 16 bytes
long, and now you do too).

Decrypt it. You know the key, after all.

Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.
Do this with code.

You can obviously decrypt this using the OpenSSL command-line tool, but we're having you get ECB working in code for a
reason. You'll need it a lot later on, and not just for attacking ECB.
*/

use openssl::symm::{decrypt, Cipher};

#[allow(unused_imports)]
use crate::set1::ch6;
#[allow(unused_imports)]
use crate::set1::constants;

pub fn decrypt_aes128_ecb(cipher: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    return decrypt(Cipher::aes_128_ecb(), &key, None, &cipher).unwrap();
}

#[test]
fn test_decrypt_aes128_ecb() {
    let key = String::from("YELLOW SUBMARINE").into_bytes();
    let data = ch6::read_file_base64(String::from("./src/set1/testdata/7.txt")).unwrap();

    let plaintext = decrypt_aes128_ecb(data, key);
    assert_eq!(String::from_utf8_lossy(&plaintext), constants::PLAY_THAT_FUNKY_MUSIC.to_string());
}