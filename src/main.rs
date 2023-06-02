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
    for i in 0..tableau.len() {
        println!("{:?}", tableau[i]);
    }
    let mut operation_type: i8;

    loop {
        println!("Choose what operation you want to perform:");
        println!("1. Encrypt text with a key");
        println!("2. Decrypt text with a key");
        let mut operation_input: String = String::new();

        io::stdin()
            .read_line(&mut operation_input)
            .expect("Failed to read line");

        operation_type = match operation_input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        println!("Chosen option: {}", operation_type);

        if operation_type < 1 || operation_type > 2 {
            println!("Wrong input: please enter either 1 or 2 and press return");
        } else {
            break;
        }
    }

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

    if operation_type == 1 {
        result = perform_encrypt(&alphabet_chars, &tableau, &key_chars, &phrase_chars);
    } else if operation_type == 2 {
        result = perform_decrypt(&alphabet_chars, &tableau, &key_chars, &phrase_chars);
    }
    println!(
        "Convertion of phrase {} with key {}: {}",
        input_phrase, input_key, result
    );
    return result;
}

fn perform_encrypt(
    alphabet_chars: &[char; 26],
    tableau: &[[char; 26]; 26],
    key_chars: &Vec<char>,
    phrase_chars: &Vec<char>,
) -> String {
    let mut result = String::new();
    let mut cycl = key_chars.iter().cycle();

    for i in 0..phrase_chars.len() {
        if !alphabet_chars.contains(&phrase_chars[i]) {
            result.push(phrase_chars[i]);
            continue;
        }
        let current_key_char = cycl.next().unwrap();
        let current_phrase_char = &phrase_chars[i];
        let current_key_char_index_option =
            alphabet_chars.iter().position(|x| x == current_key_char);

        let current_phrase_char_index_option =
            alphabet_chars.iter().position(|x| x == current_phrase_char);
        if current_key_char_index_option.is_none() || current_phrase_char_index_option.is_none() {
            continue;
        }
        // println!("key char: {}, phrase char: {}, key alpha index: {}, phase alpha index: {}, tablue row: {:?}", current_key_char, current_phrase_char,current_key_char_index_option.unwrap(), current_phrase_char_index_option.unwrap(), tableau[current_phrase_char_index_option.unwrap()]);
        let curr_position_char = tableau[current_phrase_char_index_option.unwrap()]
            [current_key_char_index_option.unwrap()];
        result.push(curr_position_char);
    }

    return result;
}

fn perform_decrypt(
    alphabet_chars: &[char; 26],
    tableau: &[[char; 26]; 26],
    key_chars: &Vec<char>,
    phrase_chars: &Vec<char>,
) -> String {
    let mut result = String::new();
    let mut cycl = key_chars.iter().cycle();

    for i in 0..phrase_chars.len() {
        if !alphabet_chars.contains(&phrase_chars[i]) {
            result.push(phrase_chars[i]);
            continue;
        }
        let current_key_char = cycl.next().unwrap();
        let current_phrase_char = &phrase_chars[i];
        let current_key_char_index_option =
            alphabet_chars.iter().position(|x| x == current_key_char);

        let current_phrase_char_index_option =
            alphabet_chars.iter().position(|x| x == current_phrase_char);
        if current_key_char_index_option.is_none() || current_phrase_char_index_option.is_none() {
            continue;
        }
        println!("key char: {}, phrase char: {}, key alpha index: {}, phase alpha index: {}, tablue row: {:?}", current_key_char, current_phrase_char,current_key_char_index_option.unwrap(), current_phrase_char_index_option.unwrap(), tableau[current_phrase_char_index_option.unwrap()]);
        let curr_position_char = tableau[current_phrase_char_index_option.unwrap()]
            [alphabet_chars.len() - current_key_char_index_option.unwrap()];
        result.push(curr_position_char);
    }

    return result;
}
