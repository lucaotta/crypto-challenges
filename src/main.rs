extern crate base64;
extern crate crypto;

fn main() {
    let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let compare = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    // println!("{:?}", to_binary(s));
    // println!("{:?}", base64::encode(to_binary(s).as_slice()));
    assert_eq!(base64::encode(crypto::to_binary(s).as_slice()), compare);
}
