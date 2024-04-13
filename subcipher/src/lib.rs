use std::io::{self, stdin, stdout, Write};

fn main(){
    let mut user_input = String::new();
    let mut key: i64 = 5;
    let mut encrypt_or_decrypt: i64 = 0;
    let mut choice = 'E';
    let mut num = 0;

    loop {
        println!("Let me help you ENCRYPT or DECRYPT your messages");
        println!("Press 1 to ENCRYPT\tand 2 to DECRYPT\n");
        print!("Enter your SELECTION: ");
        stdout().flush().unwrap();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                encrypt_or_decrypt = user_input.trim().parse().unwrap_or(0);
                if encrypt_or_decrypt == 1 || encrypt_or_decrypt == 2 {
                   break;
                }
            }
            Err(_) => {
                println!("INVALID INPUT");
                continue;
            }
        }
        user_input.clear();
    }

    match encrypt_or_decrypt{
        1 => {
            println!("Enter your message for ENCRYPTION: ");
            getline(&mut user_input);
            for c in user_input.chars() {
                if c < 'A' || c > 'Z' {
                    continue;
                }
                if c == 'Z' {
                    user_input.replace_range(
                        user_input.find(c).unwrap()..user_input.find(c).unwrap() + 1, "A");
                } else {
                    user_input.replace_range(
                        user_input.find(c).unwrap()..user_input.find(c).unwrap() + 1, 
                        &((((c as u8 - 65) + key as u8) % 26) + 65) as char);
                }
            }
            println!("ENCRYPTED Message: {}", user_input);
        }

        2 => {
            println!("Enter your message for DECRYPTION: ");
            getline(&mut user_input);
            for c in user_input.chars() {
                if c < 'A' || c > 'Z' {
                    continue;
                }
                if c - key as char < 'A' {
                    num = (c as u8 - 65 - key as u8).abs() as i32;
                    user_input.replace_range(
                        user_input.find(c).unwrap()..user_input.find(c).unwrap() + 1,
                        &(90 - num + 1) as char);
                } else {
                    user_input.replace_range(
                        user_input.find(c).unwrap()..user_input.find(c).unwrap() + 1, 
                        &((((c as u8 - 65) - key as u8) % 26) + 65) as char);
                }
            }
            println!("DECRYPTED Message: {}", user_input);
        }
        _ => {
            println!("INVALID INPUT");
        }
    }

    loop {
        print!("Still want to continue [C for CONTINUE and E to END the program]: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                choice = user_input.trim().chars().next().unwrap_or('E');
                if choice == 'E' {
                    break;
                }
            }
            Err(_) => {
                println!("INVALID INPUT");
                continue;
            }

        user_input.clear();
        }
        println!("Thank You for playing along");
    }
}

fn getline(s: &mut String) {
    s.clear();
    stdin().read_line(s).expect("Failed to read line");
    s.pop();
}
