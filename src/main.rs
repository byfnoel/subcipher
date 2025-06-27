use colored::Colorize;
use std::io::{Write, stdin, stdout};
use subcipher::{decrypt, encrypt};

fn main() {
    let entry_message = "Welcome to subcipher, the caesar cipher encryption tool";
    let width = entry_message.chars().count();
    let horizontal = "─".repeat(width + 2);
    println!(
        "{}{}{}",
        "┌".cyan().bold(),
        horizontal.cyan(),
        "┐".cyan().bold()
    );
    println!(
        "{} {} {}",
        "│".cyan(),
        entry_message.yellow().bold(),
        "│".cyan()
    );
    println!(
        "{}{}{}",
        "└".cyan().bold(),
        horizontal.cyan(),
        "┘".cyan().bold()
    );

    loop {
        println!(
            "{}",
            " Let me help you ENCRYPT or DECRYPT your message".green()
        );
        println!("Press 1 to ENCRYPT\tor Press 2 to DECRYPT\n");

        let choice = get_user_choice();

        match choice {
            1 => handle_encryption(),
            2 => handle_decryption(),
            _ => {
                println!("INVALID INPUT");
                continue;
            }
        }

        if !continue_prompt() {
            println!("Thank you for using the subcipher encryption tool!");
            break;
        }
    }
}

fn get_user_choice() -> u8 {
    print!("Enter your SELECTION: ");
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse().unwrap_or(0),
        Err(_) => {
            println!("We cannot read your input");
            0
        }
    }
}

fn handle_encryption() {
    println!("Enter your message for ENCRYPTION: ");
    let input = get_user_input();

    println!("Enter your encryption key (0-25): ");
    let key = get_key();

    match encrypt(&input, key) {
        Ok(encrypted) => println!("ENCRYPTED Message: {}", encrypted),
        Err(e) => println!("Error: {}", e),
    }
}

fn handle_decryption() {
    println!("Enter your message for DECRYPTION: ");
    let input = get_user_input();

    println!("Enter your decryption key (0-25): ");
    let key = get_key();

    match decrypt(&input, key) {
        Ok(decrypted) => println!("DECRYPTED Message: {}", decrypted),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn get_key() -> u8 {
    let mut input = String::new();
    stdout().flush().expect("Failed to flush stdout");

    match stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<u8>() {
            Ok(key) if key <= 25 => key,
            Ok(_) => {
                println!("Key must be between 0 and 25. Using default key of 5.");
                5
            }
            Err(_) => {
                println!("Invalid key. Using default key of 5.");
                5
            }
        },
        Err(_) => {
            println!("Error reading key. Using default key of 5.");
            5
        }
    }
}

fn continue_prompt() -> bool {
    print!("Still want to continue? [C for CONTINUE and E to END the program]: ");
    stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            let choice = input.trim().chars().next().unwrap_or('E');
            choice == 'C' || choice == 'c'
        }
        Err(_) => {
            println!("Error reading input. EXITING THE PROGRAM NOW!");
            false
        }
    }
}
