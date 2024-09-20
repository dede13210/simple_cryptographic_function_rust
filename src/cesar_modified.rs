use rand::Rng;

// Generate random key of 32 characters alphabet
fn random_key() -> String {
    let mut rng = rand::thread_rng();
    let key: String = (0..32).map(|_| rng.gen_range(b'a'..=b'z') as char).collect();
    key
}

// Give a letter and a key, return the encrypted letter
fn cesar_encrypt_letter(letter: char, key_char: char) -> char {
    let key = key_char as u8 - b'a';
    let mut encrypted_letter = letter as u8 + key;
    if encrypted_letter > b'z' {
        encrypted_letter -= 26;
    }
    encrypted_letter as char
}

// Give a string and a key, return the encrypted string
pub fn cesar_encrypt_string(input: &str, key: &str) -> String {
    if input.len() != key.len() {
        panic!("The key must have the same length as the input string");
    }
    let mut encrypted_string = String::with_capacity(input.len());
    let key_chars: Vec<char> = key.chars().collect();
    for (i, letter) in input.chars().enumerate() {
        encrypted_string.push(cesar_encrypt_letter(letter, key_chars[i % key.len()]));
    }
    encrypted_string
}

// Give a letter and a key, return the decrypted letter
fn cesar_decrypt_letter(letter: char, key_char: char) -> char {
    let key = key_char as u8 - b'a';
    let mut decrypted_letter = letter as u8 - key;
    if decrypted_letter < b'a' {
        decrypted_letter += 26;
    }
    decrypted_letter as char
}

// Give a lette and a int, return the translated letter
fn translated_letter_with_int(letter: char, key: u8) -> char {
    let mut translated = letter as u8 + key;
    if translated > b'z' {
        translated -= 26;
    }
    translated as char
}

// Give a string and a key, return the decrypted string
pub fn cesar_decrypt_string(input: &str, key: &str) -> String {
    let mut decrypted_string = String::with_capacity(input.len());
    let key_chars: Vec<char> = key.chars().collect();
    for (i, letter) in input.chars().enumerate() {
        decrypted_string.push(cesar_decrypt_letter(letter, key_chars[i % key.len()]));
    }
    decrypted_string
}

// make scenario
pub fn scenario1_cesar_modified() {
    let message: &str = "ceciestlemessageclairadechiffrer";
    let key = random_key();
    println!("Voici la clé: {}", key);
    println!("Voici le message de Alice: {}", message);
    let secret_message = cesar_encrypt_string(message, &key);
    println!("Eve voit ce message: {}", secret_message);
    let decrypted_message = cesar_decrypt_string(&secret_message, &key);
    println!("Bob peut le déchiffrer: {}", decrypted_message);
}

pub fn scenario2_cesar_modified() {
    let message: &str = "ceciestlemessageclairadechiffrer";
    let key = random_key();
    println!("Voici la clé: {}", key);
    println!("Voici le message de Alice: {}", message);
    let secret_message = cesar_encrypt_string(message, &key);
    println!("Eve voit ce message: {}", secret_message);
    let mut modified_message: Vec<char> = secret_message.chars().collect();
    modified_message[2] = translated_letter_with_int(modified_message[2], 9 );
    modified_message[3] = translated_letter_with_int(modified_message[3], 18);
    let modified_message: String = modified_message.into_iter().collect();
    println!("Eve modifie le message: {}", modified_message);
    let decrypted_message = cesar_decrypt_string(&modified_message, &key);
    println!("Bob le déchiffre: {}", decrypted_message);
}