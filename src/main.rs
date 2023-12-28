
use rocksdb::{DB, Options};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("stfu");
        return;
    }

    let db = DB::open_default("rocksdb_data").expect("Failed to open RocksDB");

    match args[1].as_str() {
        "add" => {
            if args.len() != 5 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];
            let username = &args[3];
            let password = &args[4];

            add_password(&db, identifier, username, password);
        }
        "show" => {
            if args.len() != 3 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];

            show_password(&db, identifier);
        }
        "remove" => {
            if args.len() != 3 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];

            remove_password(&db, identifier);
        }
        _ => {
            println!("stfu");
        }
    }
}

fn add_password(db: &DB, identifier: &str, username: &str, password: &str) {
    let key = identifier.as_bytes();
    let value = format!("{},{}", username, password);

    db.put(key, value.as_bytes()).expect("Failed to add password");
    println!("Password added successfully.");
}

fn show_password(db: &DB, identifier: &str) {
    let key = identifier.as_bytes();

    match db.get(key) {
        Ok(Some(value)) => {
            if let Ok(value_str) = String::from_utf8(value) {
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

fn remove_password(db: &DB, identifier: &str) {
    let key = identifier.as_bytes();

    match db.get(key) {
        Ok(Some(_)) => {
            db.delete(key).expect("Failed to remove password");
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
