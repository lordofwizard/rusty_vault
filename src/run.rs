/*
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::vec::Vec;
use crate::pass_entry::*;
fn test() -> io::Result<()> {
    let authenticated = authenticate_user();
    
    if !authenticated {
        println!("Authentication failed. Exiting.");
        return Ok(());
    }

    let mut password_entries: Vec<PasswordEntry> = load_passwords()?;

    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                let username = prompt("Username: ");
                let password = prompt("Password: ");
                let website = prompt("Website: ");

                let entry = PasswordEntry::new(&username, &password, &website);
                password_entries.push(entry);

                println!("Entry added successfully.");
            }
            "2" => {
                list_passwords(&password_entries)?;
            }
            "3" => {
                save_passwords(&password_entries)?;
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }

    Ok(())
}

fn authenticate_user() -> bool {
    println!("Please enter your credentials to authenticate:");
    let expected_credentials = load_first_line_from_file("passwords.txt");

    if let Some(credentials) = expected_credentials {
        let entered_username = prompt("Username: ");
        let entered_password = prompt("Password: ");
        return credentials == format!("{}:{}", entered_username, entered_password);
    }

    false
}

fn load_first_line_from_file(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).ok()?;
    
    Some(first_line.trim().to_string())
}

fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn load_passwords() -> io::Result<Vec<PasswordEntry>> {
    let path = Path::new("passwords.txt");
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            // If the file does not exist, return an empty Vec.
            return Ok(Vec::new());
        }
    };

    let reader = BufReader::new(file);
    let mut password_entries: Vec<PasswordEntry> = Vec::new();
    let mut current_entry = PasswordEntry {
        username: String::new(),
        password: String::new(),
        website: String::new(),
    };

    let mut skip_first_line = true;
    
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        
        if skip_first_line {
            skip_first_line = false;
            continue;
        }

        if line.is_empty() {
            // Empty line indicates the end of an entry.
            if !current_entry.username.is_empty() {
                password_entries.push(current_entry);
                current_entry = PasswordEntry::new("", "", "");
            }
        } else {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                match parts[0] {
                    "Username" => current_entry.username = parts[1].to_string(),
                    "Password" => current_entry.password = parts[1].to_string(),
                    "Website" => current_entry.website = parts[1].to_string(),
                    _ => (),
                }
            }
        }
    }

    if !current_entry.username.is_empty() {
        password_entries.push(current_entry);
    }

    Ok(password_entries)
}

fn save_passwords(entries: &Vec<PasswordEntry>) -> io::Result<()> {
    let path = Path::new("passwords.txt");
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(path)?;

    for entry in entries {
        file.write_all(entry.to_string().as_bytes())?;
        file.write_all(b"\n")?; // Separate entries with newlines
    }
    Ok(())
}

fn list_passwords(entries: &Vec<PasswordEntry>) -> io::Result<()> {
    println!("Password Entries:");
    
    for entry in entries {
        println!("{}", entry.to_string());
    }

    Ok(())
}*/
