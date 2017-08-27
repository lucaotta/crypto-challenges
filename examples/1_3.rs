extern crate crypto;

use crypto::{to_binary, chi_squared, best_key};

// http://cryptopals.com/sets/1/challenges/3
fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    use std::collections::HashMap;
    let mut chi_distances = HashMap::new();
    for c in "abcdefghijklmnopqrstuywxz".chars() {
        let decoded_text =
            String::from_utf8(to_binary(input).into_iter().map(|v| v ^ c as u8).collect()).unwrap();
        chi_distances.insert(c, chi_squared(&decoded_text));
    }
    let key = best_key(&chi_distances);

    // finally print the decoded string
    println!("best key {}, decoded text {}",
             key,
             String::from_utf8(to_binary(input).into_iter().map(|v| v ^ key as u8).collect())
                 .unwrap());
}