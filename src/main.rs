mod run;
mod pass_entry;
mod pentry;
use std::collections::HashMap;
use crate::pentry::ServiceInfo;


fn main() {
    let my_service = ServiceInfo::new(String::from("example_service"),
        String::from("my_username"),
        String::from("my_password")
    );
    

    my_service.write_to_file();
}


