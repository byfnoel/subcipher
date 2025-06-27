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
                write!(
                    f,
                    "Invalid text: only alphabetic characters (A-Z, a-z) are allowed"
                )
            }
        }
    }
}

impl std::error::Error for SubCipherError {}

/// Encrypts a string using  Caesar cipher encryption algorithm scheme.  
///
/// # Arguments and Returns
///
/// encrypt(input: string to encrypt, key: # of positions to shift each letter) -> encrypted string or error
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

    let mut result = String::new();

    for c in input.chars() {
        if c >= 'A' && c <= 'Z' {
            // Handle uppercase letters
            let shifted = (((c as u8 - b'A') + key) % 26) + b'A';
            result.push(shifted as char);
        } else if c >= 'a' && c <= 'z' {
            // Handle lowercase letters
            let shifted = (((c as u8 - b'a') + key) % 26) + b'a';
            result.push(shifted as char);
        } else {
            // Return error for invalid characters
            return Err(SubCipherError::InvalidCharacter);
        }
    }

    Ok(result)
}

/// Decrypts a string that was encrypted using the Caesar cipher.
///
/// # Arguments and Returns
///
/// decrypt(input: string to decrypt, key: # of positions previously used as shift) -> decrypted string or error  
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

    let mut result = String::new();

    for c in input.chars() {
        if c >= 'A' && c <= 'Z' {
            // Handle uppercase letters
            let shifted = (((c as u8 - b'A') + 26 - (key % 26)) % 26) + b'A';
            result.push(shifted as char);
        } else if c >= 'a' && c <= 'z' {
            // Handle lowercase letters
            let shifted = (((c as u8 - b'a') + 26 - (key % 26)) % 26) + b'a';
            result.push(shifted as char);
        } else {
            // Return error for invalid characters
            return Err(SubCipherError::InvalidCharacter);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("HELLO", 5).unwrap(), "MJQQT");
        assert_eq!(encrypt("ZEBRA", 5).unwrap(), "EJGWF");
        assert_eq!(encrypt("Hello", 5).unwrap(), "Mjqqt");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("MJQQT", 5).unwrap(), "HELLO");
        assert_eq!(decrypt("EJGWF", 5).unwrap(), "ZEBRA");
        assert_eq!(decrypt("Mjqqt", 5).unwrap(), "Hello");
    }

    #[test]
    fn test_invalid_key() {
        assert!(encrypt("HELLO", 26).is_err());
        assert!(decrypt("HELLO", 26).is_err());
    }

    #[test]
    fn test_invalid_character() {
        assert!(encrypt("Hello World!", 5).is_err());
        assert!(decrypt("Hello World!", 5).is_err());
        assert!(encrypt("HELLO123", 5).is_err());
        assert!(decrypt("HELLO123", 5).is_err());
        assert!(encrypt("HELLO@#$", 5).is_err());
        assert!(decrypt("HELLO@#$", 5).is_err());
    }

    #[test]
    fn test_roundtrip() {
        let original = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let key = 13;
        let encrypted = encrypt(original, key).unwrap();
        let decrypted = decrypt(&encrypted, key).unwrap();
        assert_eq!(decrypted, original);
    }

    #[test]
    fn test_mixed_case() {
        let original = "HelloWorld";
        let key = 3;
        let encrypted = encrypt(original, key).unwrap();
        let decrypted = decrypt(&encrypted, key).unwrap();
        assert_eq!(decrypted, original);
        assert_eq!(encrypted, "KhoorZruog");
    }
}
