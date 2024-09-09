mod cesar_encrypt;

use std::string;
use cesar_encrypt::cesar_encrypt_string;

fn main() {
    let secret_message:string::String = cesar_encrypt_string("hello", 3);
    println!("{}", secret_message);
}
