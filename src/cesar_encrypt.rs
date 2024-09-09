// Give a letter and a key, return the encrypted letter
pub fn cesar_encrypt_letter(letter: char, key: u8) -> char {
    let mut encrypted_letter = letter as u8;
    encrypted_letter += key;
    if encrypted_letter > b'z' {
        encrypted_letter -= 26;
    }
    encrypted_letter as char
}

pub fn cesar_encrypt_string(input: &str, key: u8) -> String {
    input.chars().map(|c| cesar_encrypt_letter(c, key)).collect()
}