use std::io;
use crate::lib_collections::{Command, Storage};

mod lib_collections;

// solving last problem at https://doc.rust-lang.org/book/ch08-03-hash-maps.html
fn main() {
    let mut input = String::new();
    let mut storage = Storage::new();

    loop {

        println!("Write a prompt to interact with the user datastore:
- change groups `add/remove/detail group [name-of-group]'
- change users `add/remove/detail user [name-of-user]'
- change membership between users and groups 'add/remove/detail membership [name-of-user] into/into/into [name of group]'");

        io::stdin().read_line(&mut input).unwrap();

        let command = Command::new(input.as_str());
        input.clear();

        let command = match command {
            Ok(c) => {
                println!("Submitting command: {:?}", c);
                c
            },
            Err(_) => {
                println!("Wrong input formatting! Please try again...");
                continue;
            }
        };

        match storage.submit(&command) {
            Ok(_) => {println!("Success!")}
            Err(_) => {println!("Failed to perform command...")}
        };
    }
}