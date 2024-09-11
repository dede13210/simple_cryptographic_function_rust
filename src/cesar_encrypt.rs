use rand::Rng;

// Give a letter and a key, return the encrypted letter
pub fn cesar_encrypt_letter(letter: char, key: u8) -> char {
    let mut encrypted_letter = letter as u8;
    encrypted_letter += key;
    if encrypted_letter > b'z' {
        encrypted_letter -= 26;
    }
    encrypted_letter as char
}

// Give a string and a key, return the encrypted string
pub fn cesar_encrypt_string(input: &str, key: u8) -> String {
    let mut encrypted_string = String::with_capacity(input.len());
    for letter in input.chars() {
        // If the letter is a space, we keep it as is
        if letter == ' ' {
            encrypted_string.push(' ');
        } else {
            encrypted_string.push(cesar_encrypt_letter(letter, key));
        }
    }
    encrypted_string
}

// Give a letter and a key, return the decrypted letter
pub fn cesar_decrypt_letter(letter: char, key: u8) -> char {
    let mut decrypted_letter = letter as u8;
    decrypted_letter -= key;
    if decrypted_letter < b'a' {
        decrypted_letter += 26;
    }
    decrypted_letter as char
}

// Give a string and a key, return the decrypted string
pub fn cesar_decrypt_string(input: &str, key: u8) -> String {
    let mut decrypted_string = String::with_capacity(input.len());
    for letter in input.chars() {
        // If the letter is a space, we keep it as is
        if letter == ' ' {
            decrypted_string.push(' ');
        } else {
            decrypted_string.push(cesar_decrypt_letter(letter, key));
        }
    }
    decrypted_string
}

// make scenario
pub fn scenario_cesar() {
    let message: &str = "ceciestlemessageclairadechiffrer";
    // generate random integer between 1 and 25
    let k = rand::thread_rng().gen_range(1..=25);
    println!("Voici la clé: {}", k);
    println!("Voici le message de bob: {}", message);
    let secret_message = cesar_encrypt_string(message, k);
    println!("Eve voit ce message: {}", secret_message);
    let decrypted_message = cesar_decrypt_string(&secret_message, k);
    println!("Alice peut le déchiffrer: {}", decrypted_message);
}