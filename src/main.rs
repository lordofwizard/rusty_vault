use sled::{Db, Tree};
use crypto::aes::cbc_decrypt;
use crypto::aes::KeySize::KeySize256;
use crypto::blockmodes::NoPadding;
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::symmetriccipher::{Decryptor, Encryptor};
use std::env;
use std::fs;
use std::io::{self, Write};
use rpassword::read_password_from_tty;

const CONFIG_DIR: &str = ".config/rusty_vault";
const MASTER_PASSWORD_FILE: &str = "pass.txt";
const ENCRYPTED_DB_FILE: &str = "database.sled";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("stfu");
        return;
    }

    let config_dir = match dirs::home_dir() {
        Some(mut path) => {
            path.push(CONFIG_DIR);
            path
        }
        None => {
            eprintln!("Failed to determine home directory.");
            return;
        }
    };

    let _ = fs::create_dir_all(&config_dir);

    let master_password = read_master_password(&config_dir);

    if master_password.is_none() {
        println!("Master password is not set.");
        return;
    }

    let master_password = master_password.unwrap();

    let db = Db::open(config_dir.join(ENCRYPTED_DB_FILE)).expect("Failed to open sled DB");
    let tree = db.open_tree("passwords").expect("Failed to open sled tree");

    match args[1].as_str() {
        "add" => {
            if args.len() != 5 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];
            let username = &args[3];
            let password = &args[4];

            add_password(&tree, identifier, username, password, &master_password);
        }
        "show" => {
            if args.len() != 3 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];

            show_password(&tree, identifier, &master_password);
        }
        "remove" => {
            if args.len() != 3 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];

            remove_password(&tree, identifier, &master_password);
        }
        _ => {
            println!("stfu");
        }
    }
}

fn add_password(tree: &Tree, identifier: &str, username: &str, password: &str, master_password: &str) {
    let key = identifier.as_bytes();
    let value = format!("{},{}", username, password);

    let encrypted_value = encrypt_data(value.as_bytes(), master_password.as_bytes());

    tree.insert(key, encrypted_value.as_slice()).expect("Failed to add password");
    println!("Password added successfully.");
}

fn show_password(tree: &Tree, identifier: &str, master_password: &str) {
    let key = identifier.as_bytes();

    match tree.get(key) {
        Ok(Some(encrypted_value)) => {
            let decrypted_value = decrypt_data(encrypted_value, master_password.as_bytes());

            if let Ok(value_str) = String::from_utf8(decrypted_value) {
                let fields: Vec<&str> = value_str.split(',').collect();
                if fields.len() == 2 {
                    println!("Password: {}", fields[1]);
                } else {
                    println!("stfu");
                }
            } else {
                println!("stfu");
            }
        }
        Ok(None) => {
            println!("stfu");
        }
        Err(e) => {
            eprintln!("Error retrieving password: {}", e);
        }
    }
}

fn remove_password(tree: &Tree, identifier: &str, master_password: &str) {
    let key = identifier.as_bytes();

    match tree.get(key) {
        Ok(Some(_)) => {
            tree.remove(key).expect("Failed to remove password");
            println!("Password removed successfully.");
        }
        Ok(None) => {
            println!("stfu");
        }
        Err(e) => {
            eprintln!("Error retrieving password: {}", e);
        }
    }
}

fn read_master_password(config_dir: &std::path::PathBuf) -> Option<String> {
    let master_password_path = config_dir.join(MASTER_PASSWORD_FILE);

    if let Ok(password) = fs::read_to_string(&master_password_path) {
        Some(password.trim().to_string())
    } else {
        None
    }
}

fn encrypt_data(data: &[u8], key: &[u8]) -> Vec<u8> {
    let iv: [u8; 16] = [0; 16];
    let mut encryptor = crypto::aes::cbc_encryptor(
        KeySize256,
        key,
        &iv,
        NoPadding,
    );

    let mut final_result = Vec::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        match encryptor.encrypt(&mut read_buffer, &mut write_buffer, true) {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => (),
            Err(e) => panic!("Error encrypting data: {}", e),
        }

        final_result.extend(write_buffer.take_read_buffer().take_remaining());
    }

    final_result
}

fn decrypt_data(data: &[u8], key: &[u8]) -> Vec<u8> {
    let iv: [u8; 16] = [0; 16];
    let mut decryptor = crypto::aes::cbc_decryptor(
        KeySize256,
        key,
        &iv,
        NoPadding,
    );

    let mut final_result = Vec::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        match decryptor.decrypt(&mut read_buffer, &mut write_buffer, true) {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => (),
            Err(e) => panic!("Error decrypting data: {}", e),
        }

        final_result.extend(write_buffer.take_read_buffer().take_remaining());
    }

    final_result
}
