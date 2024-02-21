use std::fmt::Error;

pub trait RunCommand {
    fn submit(command: &str) -> Result<String, Error>;
}

#[derive(Debug)]
pub enum Entity {
    Group(String),
    User(String),
    Relation(String, String),
}

#[derive(Debug)]
pub enum Command {
    Create(Entity),
    Delete(Entity),
    Read(Entity),
}

impl Command {
    pub fn new(text: &str) -> Result<Self, Error> {

        let mut words = text.split_whitespace();

        // ((add|remove|detail) (group|user) [name])|(add/remove/detail membership [name-of-user] into [name-of-group])
        let command = words.next().ok_or(Error)?;
        let entity = words.next().ok_or(Error)?;
        let name1: String = words.next().ok_or(Error)?.parse().unwrap();
        let connector = words.next();
        let name2 = words.next();


        let entity = match entity {
            "group" => Entity::Group(name1),
            "user" => Entity::User(name1),
            "membership" => {
                if !connector.ok_or(Error)?.eq("into") { return Err(Error); }
                let name2 = name2.ok_or(Error)?.parse().unwrap();
                Entity::Relation(name1, name2) },
            _ => return Err(Error),
        };

        let command = match command {
            "add" => Command::Create(entity),
            "remove" => Command::Delete(entity),
            "detail" => Command::Read(entity),
            _ => return Err(Error),
        };

        Ok(command)
    }
}