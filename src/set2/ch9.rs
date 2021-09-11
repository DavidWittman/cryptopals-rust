/* Implement PKCS#7 padding

A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into ciphertext. But we almost never
 want to transform a single block; we encrypt irregularly-sized messages.

One way we account for irregularly-sized messages is by padding, creating a plaintext that is an even multiple of the
blocksize. The most popular padding scheme is called PKCS#7.

So: pad any block to a specific block length, by appending the number of bytes of padding to the end of the block. For
instance,

    "YELLOW SUBMARINE"

... padded to 20 bytes would be:

    "YELLOW SUBMARINE\x04\x04\x04\x04"

*/

fn pkcs7_pad(bytes: Vec<u8>, block_size: usize) -> Vec<u8> {
    let pad_size = block_size - (bytes.len() % block_size);
    [bytes, std::iter::repeat(pad_size as u8).take(pad_size).collect()].concat()
}

#[test]
fn test_pkcs7_pad() {
    assert_eq!(
        pkcs7_pad("YELLOW SUBMARINE".as_bytes().to_vec(), 20), 
        "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes().to_vec()
    );

    assert_eq!(
        pkcs7_pad("LOLWUT".as_bytes().to_vec(), 16),
        "LOLWUT\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A".as_bytes().to_vec()
    );
}