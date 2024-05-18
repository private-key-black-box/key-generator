use std::fmt;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, Message};
use cocoon::{Error as CocoonError, MiniCocoon};
use rand::{distributions::Alphanumeric, Rng};
use thiserror::Error;

// Wrapper for cocoon::Error implementing std::error::Error
#[derive(Debug)]
pub struct WrappedCocoonError(CocoonError);

impl fmt::Display for WrappedCocoonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::error::Error for WrappedCocoonError {}

impl From<CocoonError> for WrappedCocoonError {
    fn from(error: CocoonError) -> Self {
        WrappedCocoonError(error)
    }
}

#[derive(Error, Debug)]
pub enum HelpersError {
    #[error("Failed to encrypt the string")]
    EncryptError(#[from] WrappedCocoonError),
}

pub fn encrypt_string(input: &str) -> Result<Vec<u8>, HelpersError> {
    let passphrase = generate_secure_passphrase(32);
    println!("Passphrase: {}", passphrase);
    let mut data = input.to_owned().into_bytes();
    let mut cocoon = MiniCocoon::from_key(passphrase.as_bytes(), &[0; 32]);

    let detached_prefix = cocoon
        .encrypt(&mut data)
        .map_err(WrappedCocoonError::from)?;

    // cocoon.decrypt(&mut data, &detached_prefix).map_err(WrappedCocoonError::from)?;
    Ok(detached_prefix.to_vec())
}

fn generate_secure_passphrase(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn generate_private_key() {
    // Create a new Secp256k1 context
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);


    // Print the private key and public key as a hex string
    println!("Private Key: {:?}", secret_key);
    println!("Public Key: {:?}", public_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secure_passphrase() {
        let length = 32;
        let secure_string = generate_secure_passphrase(length);
        assert_eq!(secure_string.len(), length);
    }
}
