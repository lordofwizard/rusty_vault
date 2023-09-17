
use std::io;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug,Serialize,Deserialize)]
pub struct ServiceInfo {
    service: String,
    username: String,
    password: String,
}
impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    pub fn from_user_input() -> Self {
        println!("Enter Password Entry:");
        let mut service = String::new();
        io::stdin().read_line(&mut service).expect("Failed to read line");

        println!("Enter Username:");
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read line");

        println!("Enter Password:");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read line");

        ServiceInfo::new(service.trim().to_string(), username.trim().to_string(), password.trim().to_string())
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }
    pub fn write_to_file(&self) {
        let json_output = self.to_json();

        match File::create("passwords.json") {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("Successfully wrote to passwords.json");
                }
            }
            Err(e) => eprintln!("Error creating file: {}", e),
        }
    }

}


