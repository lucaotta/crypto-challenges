use std::collections::HashMap;

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

fn english_probabilties() -> HashMap<char, f32> {
    let mut prob = HashMap::new();

    prob.insert('a', 0.08167);
    prob.insert('b', 0.01492);
    prob.insert('c', 0.02782);
    prob.insert('d', 0.04253);
    prob.insert('e', 0.12702);
    prob.insert('f', 0.02228);
    prob.insert('g', 0.02015);
    prob.insert('h', 0.06094);
    prob.insert('i', 0.06966);
    prob.insert('j', 0.00153);
    prob.insert('k', 0.00772);
    prob.insert('l', 0.04025);
    prob.insert('m', 0.02406);
    prob.insert('n', 0.06749);
    prob.insert('o', 0.07507);
    prob.insert('p', 0.01929);
    prob.insert('q', 0.00095);
    prob.insert('r', 0.05987);
    prob.insert('s', 0.06327);
    prob.insert('t', 0.09056);
    prob.insert('u', 0.02758);
    prob.insert('v', 0.00978);
    prob.insert('w', 0.02360);
    prob.insert('x', 0.00150);
    prob.insert('y', 0.01974);
    prob.insert('z', 0.00074);
    prob
}

pub fn chi_squared(s: &str) -> f32 {
    let mut letter_count = HashMap::new();
    // filter away non-letters
    for c in s.to_lowercase().chars().filter(|ch| ch.is_alphabetic()) {
        let count = letter_count.entry(c).or_insert(0);
        *count += 1;
    }

    let mut chi = 0f32;
    let english = english_probabilties();
    for c in "abcdefghijklmnopqrstuywxz".chars() {
        let expected_count = english[&c] * s.len() as f32;
        chi += (*letter_count.get(&c).unwrap_or(&0) as f32 - expected_count).powi(2) /
               expected_count;
    }
    return chi;
}

pub fn best_key(chi_distances: &HashMap<char, f32>) -> char {
    let mut min = std::f32::MAX;
    let mut key = 'a';
    for (c, value) in chi_distances {
        if *value < min {
            min = *value;
            key = *c;
        }
    }
    key
}

pub fn to_binary(s: &str) -> Vec<u8> {
    // always assume bytes here
    s.as_bytes().chunks(2).map(|s| to_hex(s).unwrap()).collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_best_key() {
        let mut chi = HashMap::new();
        chi.insert('a', 12.4);
        chi.insert('b', 1.3);
        chi.insert('c', 31.4);
        assert_eq!(best_key(&chi), 'b');
    }

    #[test]
    fn test_chi_squared() {
        assert_eq!(chi_squared("poiuasdf asfkjjqwou"), 215.37578);
        assert_eq!(chi_squared("This is English text"), 46.999886);
    }
}