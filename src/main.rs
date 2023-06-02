use std::io;

fn main() {
    vigenere();
}

fn vigenere() -> String {
    let mut result = String::new();
    let alphabet_chars: [char; 26] = (b'a'..=b'z')
        .map(|x| x as char)
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    let mut tableau: [[char; 26]; 26] = [[' '; 26]; 26];
    for i in 0..26 {
        let shifted_alphabet: [char; 26] = alphabet_chars[i..26]
            .iter()
            .chain(alphabet_chars[0..i].iter())
            .copied()
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        tableau[i] = shifted_alphabet;
    }

    println!("{:?}", tableau.iter().find(|x| x[0] == 'h').unwrap());

    println!("Choose what operation you want to perform:");
    println!("1. Encrypt text with a key");
    println!("2. Decrypt text with a key");

    let mut operationInput = String::new();
    io::stdin()
        .read_line(&mut operationInput)
        .expect("Failed to read line");

    let operationType: i8 = match operationInput.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut input_key = String::new();
    let mut input_phrase = String::new();

    println!("Enter the key:");
    io::stdin()
        .read_line(&mut input_key)
        .expect("Failed to read the key");
    println!("Enter the phrase:");
    io::stdin()
        .read_line(&mut input_phrase)
        .expect("Failed to read the phrase");

    let key = input_key.trim();
    let phrase = input_phrase.trim();

    let key_chars = key.chars().collect::<Vec<char>>();
    let phrase_chars = phrase.chars().collect::<Vec<char>>();
    let key_queue = key_chars.iter().cycle();
    for ch in 0..phrase_chars.len() {}

    return result;
}
