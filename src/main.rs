mod run;
mod pass_entry;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Entry(String, String);

fn main(){

    let mut passwords : HashMap<String , Entry> = HashMap::new();
    passwords.insert(String::from("Google"),Entry(String::from("advait@gmail.com"),String::from("iamyofathr")));

    println!("{:?}",passwords);
    println!("Hello World, babyyy!");

}

