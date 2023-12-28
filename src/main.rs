use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("stfu");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() != 5 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];
            let username = &args[3];
            let password = &args[4];

            add_password(identifier, username, password);
        }
        "show" => {
            if args.len() != 3 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];

            show_password(identifier);
        }
        "remove" => {
            if args.len() != 3 {
                println!("stfu");
                return;
            }

            let identifier = &args[2];

            remove_password(identifier);
        }
        _ => {
            println!("stfu");
        }
    }
}

fn add_password(identifier: &str, username: &str, password: &str) {
    let entry = format!("{},{},{}\n", identifier, username, password);

    if let Ok(mut file) = File::create("pass.csv") {
        if let Err(e) = file.write_all(entry.as_bytes()) {
            eprintln!("Error writing to file: {}", e);
        }
        println!("Password added successfully.");
    } else {
        eprintln!("Error creating file.");
    }
}

fn show_password(identifier: &str) {
    if let Ok(file) = File::open("pass.csv") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(entry) = line {
                let fields: Vec<&str> = entry.split(',').collect();
                if fields.len() == 3 && fields[0] == identifier {
                    println!("Password: {}", fields[2]);
                    return;
                }
            }
        }

        println!("stfu");
    } else {
        println!("stfu");
    }
}

fn remove_password(identifier: &str) {
    let temp_file = "pass_temp.csv";

    if let Ok(file) = File::open("pass.csv") {
        let reader = BufReader::new(file);
        let mut writer = File::create(temp_file).expect("Error creating temp file.");

        for line in reader.lines() {
            if let Ok(entry) = line {
                let fields: Vec<&str> = entry.split(',').collect();
                if fields.len() == 3 && fields[0] == identifier {
                    continue; // Skip the entry to be removed
                }

                if let Err(e) = writeln!(writer, "{}", entry) {
                    eprintln!("Error writing to temp file: {}", e);
                }
            }
        }
    } else {
        eprintln!("Error opening file.");
        return;
    }

    // Replace the original file with the temp file
    if let Err(e) = fs::rename(temp_file, "pass.csv") {
        eprintln!("Error renaming temp file: {}", e);
    }

    println!("Password removed successfully.");
}
