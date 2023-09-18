mod pass_entry;
mod pentry;
mod run;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;

fn main() {
    let services = read_passwords_from_file().unwrap_or_else(|err| {
        eprintln!("Error reading passwords: {}", err);
        Vec::new()
    });

    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                println!("Entry added successfully.");
                entry.write_to_file();
            }
            "2" => {
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!("Entry = {:?}", item);
                }
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
    for service in &services {
        println!("{:?}", service);
    }
    //_my_service.write_to_file();
}
