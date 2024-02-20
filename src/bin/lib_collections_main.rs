use std::io;

mod lib_collections;

// solving last problem at https://doc.rust-lang.org/book/ch08-03-hash-maps.html
fn main() {
    let mut input = String::new();

    loop {
        // ((add|remove|detail) (group|user) [name])|(add/remove/detail membership [name-of-user] to/from [name-of-group])

        println!("Write a prompt to interact with the user datastore:\n
            change groups `add/remove/detail group [name-of-group]'\n
            change users `add/remove/detail user [name-of-user]'\n
            change membership between users and groups 'add/remove/detail membership [name-of-user]\
             to/from/from [name of group]'");

        io::stdin().read_line(&mut input).unwrap();

    }
}