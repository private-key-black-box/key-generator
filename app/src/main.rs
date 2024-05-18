use zeroize::Zeroize;

use helpers::{generate_private_key, encrypt_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "my private key";
    let mut secure_string = encrypt_string(input)?;
    println!("Encrypted string: {:?}", secure_string);

    generate_private_key();
    // Zero the secure string memory
    secure_string.zeroize();
    println!("Memory zeroed for the original secure string.");

    Ok(())
}
