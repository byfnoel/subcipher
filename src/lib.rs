//! SubCipher - A Caesar cipher implementation in Rust
//!
//! This library provides functions for encrypting and decrypting text
//! using the Caesar cipher algorithm.

use std::fmt;

/// Errors that can occur during encryption or decryption
#[derive(Debug)]
pub enum SubCipherError {
    InvalidKey,
    InvalidCharacter,
}

impl fmt::Display for SubCipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubCipherError::InvalidKey => write!(f, "Invalid key: must be between 0 and 25"),
            SubCipherError::InvalidCharacter => {
                write!(f, "Invalid text: all characters must be between A-Z")
            }
        }
    }
}

impl std::error::Error for SubCipherError {}

/// Encrypts a string using the Caesar cipher.  
///
/// # Arguments
///
/// * `input` - The string to encrypt
/// * `key` - The number of positions to shift each letter (0-25)
///
/// # Returns
///
/// The encrypted string, or an error if the key is invalid.
///
/// # Examples
///
/// ```
/// use subcipher::encrypt;
///   
/// let encrypted = encrypt("HELLO", 5).unwrap();
/// assert_eq!(encrypted, "MJQQT");
/// ```
pub fn encrypt(input: &str, key: u8) -> Result<String, SubCipherError> {
    if key > 25 {
        return Err(SubCipherError::InvalidKey);
    }

    Ok(input
        .chars()
        .map(|c| {
            if c >= 'A' && c <= 'Z' {
                let shifted = (((c as u8 - b'A') + key) % 26) + b'A';
                shifted as char
            } else {
                c
            }
        })
        .collect())
}

/// Decrypts a string that was encrypted using the Caesar cipher.
///
/// # Arguments
///  
/// * `input` - The string to decrypt
/// * `key` - The number of positions that were used to shift each letter (0-25)
///
/// # Returns
///
/// The decrypted string, or an error if the key is invalid.
///
/// # Examples
///
/// ```
/// use subcipher::decrypt;
///
/// let decrypted = decrypt("MJQQT", 5).unwrap();
/// assert_eq!(decrypted, "HELLO");  
/// ```
pub fn decrypt(input: &str, key: u8) -> Result<String, SubCipherError> {
    if key > 25 {
        return Err(SubCipherError::InvalidKey);
    }

    Ok(input
        .chars()
        .map(|c| {
            if c >= 'A' && c <= 'Z' {
                let shifted = (((c as u8 - b'A') + 26 - (key % 26)) % 26) + b'A';
                shifted as char
            } else {
                c
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("HELLO", 5).unwrap(), "MJQQT");
        assert_eq!(encrypt("ZEBRA", 5).unwrap(), "EJGWF");
        assert_eq!(encrypt("Hello World!", 5).unwrap(), "Hello World!");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("MJQQT", 5).unwrap(), "HELLO");
        assert_eq!(decrypt("EJGWF", 5).unwrap(), "ZEBRA");
        assert_eq!(decrypt("Hello World!", 5).unwrap(), "Hello World!");
    }

    #[test]
    fn test_invalid_key() {
        assert!(encrypt("HELLO", 26).is_err());
        assert!(decrypt("HELLO", 26).is_err());
    }

    #[test]
    fn test_roundtrip() {
        let original = "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";
        let key = 13;
        let encrypted = encrypt(original, key).unwrap();
        let decrypted = decrypt(&encrypted, key).unwrap();
        assert_eq!(decrypted, original);
    }
}
