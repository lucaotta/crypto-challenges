extern crate crypto;

fn main() {
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let compare = "746865206b696420646f6e277420706c6179";

    assert_eq!(crypto::to_binary(input1)
                   .into_iter()
                   .zip(crypto::to_binary(input2))
                   .map(|t| t.0 ^ t.1)
                   .collect::<Vec<u8>>(),
               crypto::to_binary(compare));
}