/* Implement repeating-key XOR

Here is the opening stanza of an important work of the English language:

> Burning 'em, if you ain't quick and nimble
> I go crazy when I hear a cymbal

Encrypt it, under the key "ICE", using repeating-key XOR.

In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.

It should come out to:

```
0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
```

Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt your mail. Encrypt your password file. Your .sig file. Get a feel for it. I promise, we aren't wasting your time with this.
*/

pub fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(plaintext.len());

    for (pos, elem) in plaintext.iter().enumerate() {
        result.push(elem ^ key[pos % key.len()]);
    }
    result
}

pub fn repeating_key_xor_strings(plaintext: String, key: String) -> Vec<u8> {
    let plaintext = plaintext.as_bytes();
    let key = key.as_bytes();
    return repeating_key_xor(plaintext, key);
}

#[test]
fn test_repeating_key_xor() {
    use hex;

    let key = String::from("ICE");
    let plaintext = String::from("Burning 'em, if you ain't quick and nimble\n\
        I go crazy when I hear a cymbal");
    let expected = hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f2043\
        0a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();

    assert_eq!(repeating_key_xor_strings(plaintext, key), expected);
}