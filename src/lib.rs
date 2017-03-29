fn hex_digit(c: u8) -> Option<u8> {
    match c as char {
        '0'...'9' => Some(c - 48),
        'a'...'f' => Some(c - 87),
        'A'...'F' => Some(c - 55),
        _ => None,
    }
}

fn to_hex(s: &[u8]) -> Option<u8> {
    if s.len() != 2 {
        panic!("Not supported");
    }

    let first = hex_digit(s[0]).unwrap();
    let second = hex_digit(s[1]).unwrap();
    Some(first * 16 + second)
}

pub fn to_binary(s: &str) -> Vec<u8> {
    // always assume bytes here
    s.as_bytes().chunks(2).map(|s| to_hex(s).unwrap()).collect::<Vec<u8>>()
}