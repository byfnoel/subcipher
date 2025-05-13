use subcipher::{decrypt, encrypt};

fn main() {
    let original = "I HOPE ALL IS WELL FOR YOU AND YOUR FAMILY";
    let key = 13;

    match encrypt(original, key) {
        Ok(encrypted) => {
            println!("The encrypoted text is: {}", encrypted);

            match decrypt(&encrypted, key) {
                Ok(decrypted) => println!("The decrypted text is: {}", decrypted),
                Err(e) => println!("Decryption error: {}", e),
            }
        }
        Err(e) => println!("Encryption error: {}", e),
    }
}
