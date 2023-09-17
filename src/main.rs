mod run;
mod pass_entry;
mod pentry;

use crate::pentry::ServiceInfo;
use crate::pentry::read_passwords_from_file;


fn main() {
    let _my_service = ServiceInfo::new(String::from("Google"),
        String::from("alskjkj"),
        String::from("aslkjglkajsg")
    );
    let services = read_passwords_from_file().unwrap_or_else(|err| {
        eprintln!("Error reading passwords: {}", err);
        Vec::new()
    });

    for service in &services {
        println!("{:?}", service);
    }
   //_my_service.write_to_file();
}


