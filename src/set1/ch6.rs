/*  Break repeating-key XOR
It is officially on, now.

This challenge isn't conceptually hard, but it involves actual error-prone coding. The other challenges in this set are
there to bring you up to speed. This one is there to qualify you. If you can do this one, you're probably just fine up
to Set 6.

There's a file here. It's been base64'd after being encrypted with repeating-key XOR.

Decrypt it.

Here's how:

    1. Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
    2. Write a function to compute the edit distance/Hamming distance between two strings.
    The Hamming distance is just the number of differing bits. The distance between:

    `this is a test`

    and

    `wokka wokka!!!`

    is **37**. *Make sure your code agrees before you proceed.*

    3. For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the
       edit distance between them. Normalize this result by dividing by KEYSIZE.
    4. The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with
       the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
    5. Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
    6. Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second
       byte of every block, and so on.
    7. Solve each block as if it was single-character XOR. You already have code to do this.
    8. For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key
       byte for that block. Put them together and you have the key.

This code is going to turn out to be surprisingly useful later on. Breaking repeating-key XOR ("Vigenere")
statistically is obviously an academic exercise, a "Crypto 101" thing. But more people "know how" to break it than can
actually break it, and a similar technique breaks something much more important.

No, that's not a mistake.
We get more tech support questions for this challenge than any of the other ones. We promise, there aren't any blatant
errors in this text. In particular: the "wokka wokka!!!" edit distance really is 37.
*/

#![allow(dead_code)]

use std::fs;
use std::io;

use base64;

use crate::set1::ch3;
use crate::set1::ch5;
#[allow(unused_imports)]
use crate::set1::constants;


const MAX_KEY_SIZE: u8 = 40;

pub fn read_file_base64(filename: String) -> io::Result<Vec<u8>> {
    let mut file = fs::read(filename)?;
    // strip out CR+LF newlines
    file.retain(|&x| x != 13 && x != 10);
    Ok(base64::decode(file).expect("Invalid base64"))
}

// get_block returns a specific block `number` from `ciphertext` based on the provided `key_size`
pub fn get_block(ciphertext: &Vec<u8>, key_size: u8, number: u16) -> Vec<u8> {
    return ciphertext[usize::from(key_size as u16 * number)..usize::from(key_size as u16 *(number+1))].to_vec();
}

// https://www.hackertouch.com/matrix-transposition-in-rust.html
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> 
where
    T: Clone,
    T: Copy,
{
    let mut t = vec![Vec::with_capacity(v.len()); v[0].len()];
    for r in v {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

#[test]
fn test_transpose() {
    assert_eq!(
        transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}

pub fn hamming_distance(a: &Vec<u8>, b: &Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    //println!("{:?}, {:?}", a, b);
    if a.len() != b.len() {
        panic!("Lengths are not equal");
    }
    for (pos, elem) in a.iter().enumerate() {
        sum += (elem ^ b[pos]).count_ones();
    }
    sum
}

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance(&"this is a test".as_bytes().to_vec(), &"wokka wokka!!!".as_bytes().to_vec()), 37);
}

#[test]
fn test_read_file_base64() {
    let data = read_file_base64(String::from("./src/set1/testdata/6.txt")).unwrap();
    assert_eq!(data.len(), 2876);
}

pub fn find_key_size(ciphertext: &Vec<u8>) -> u8 {
    let mut lowest_distance = f64::MAX;
    let mut key_size: u8 = 0;

    for size in 2..=MAX_KEY_SIZE {
        let (block0, block1, block2, block3) = (
            get_block(&ciphertext, size, 0),
            get_block(&ciphertext, size, 1),
            get_block(&ciphertext, size, 2),
            get_block(&ciphertext, size, 3),
        );

        let sum = hamming_distance(&block0, &block1) + hamming_distance(&block1, &block2)
            + hamming_distance(&block2, &block3) + hamming_distance(&block0, &block2)
            + hamming_distance(&block0, &block3) + hamming_distance(&block1, &block3);

        let normalized_distance: f64 = f64::from((sum as f64) / size as f64);

        if normalized_distance < lowest_distance {
            lowest_distance = normalized_distance;
            key_size = size;
        }
    }

    key_size
}

#[test]
fn test_find_key_size() {
    let data = read_file_base64(String::from("./src/set1/testdata/6.txt")).unwrap();
    assert_eq!(find_key_size(&data), 29);
}

#[test]
fn test_chunks() {
    let data = read_file_base64(String::from("./src/set1/testdata/6.txt")).unwrap();
    let chunks = data.chunks(32usize);
    assert_eq!(chunks.len(), 90);
}

fn break_repeating_key_xor(cipher: Vec<u8>) -> (Vec<u8>, String) {
    let mut key = Vec::new();
    let key_size = find_key_size(&cipher);
    
    let chunks: Vec<Vec<u8>> = transpose(cipher.chunks(key_size as usize).map(|x| x.to_vec()).collect());

    for chunk in chunks.iter() {
        key.push(ch3::find_xor_byte(chunk.to_vec()));
    }

    let plaintext = ch5::repeating_key_xor(&cipher, &key);
    (key, String::from_utf8_lossy(&plaintext).to_string())
}

#[test]
fn test_break_repeating_key_xor() {
    let data = read_file_base64(String::from("./src/set1/testdata/6.txt")).unwrap();

    let expected_key = vec![84, 101, 114, 109, 105, 110, 97, 116, 111, 114, 32, 88, 58, 32, 66, 114, 105, 110, 103, 32, 116, 104, 101, 32, 110, 111, 105, 115, 101];

    let (key, plaintext) = break_repeating_key_xor(data);
    assert_eq!(key, expected_key);
    assert_eq!(plaintext, constants::PLAY_THAT_FUNKY_MUSIC.to_string());
}