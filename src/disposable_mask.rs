use rand::Rng;

// Generate a random binary string of the given length
fn random_binary_string(length: usize) -> Vec<u16> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<u16>()).collect()
}

// XOR two binary strings
fn xor_string(string1: Vec<u16>, string2: Vec<u16>) -> Vec<u16> {
    assert_eq!(string1.len(), string2.len(), "Strings must have equal length");

    string1
        .iter()
        .zip(string2.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn display_utf16_vec_in_bits(vec: Vec<u16>) {
    for value in vec {
        println!("{:016b}", value);
    }
}

// make scenario
pub fn scenario_disposable_mask() {
    let message: &str = "ceciestlemessageclairadechiffrer";
    println!("Voici le message de Alice: {}", message);
    
    // Encode the message in UTF-16 to determine the correct length
    let encoded_message: Vec<u16> = message.encode_utf16().collect();
    
    // Generate a key with the same length as the UTF-16 encoded message
    let key = random_binary_string(encoded_message.len());
    println!("Voici la clé en bits");
    display_utf16_vec_in_bits(key.clone());

    // XOR the encoded message with the key
    let secret_message = xor_string(encoded_message.clone(), key.clone());
    println!("Eve voit ce message en bits:");
    display_utf16_vec_in_bits(secret_message.clone());

    // Decrypt the message using the same key
    let decrypted_message = xor_string(secret_message, key);
    let decoded_message: String = String::from_utf16(&decrypted_message).expect("Invalid UTF-16 sequence");
    println!("Bob peut le déchiffrer: {}", decoded_message);
}


