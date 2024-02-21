use std::fmt::Error;

pub trait RunCommand {
    fn submit(command: &str) -> Result<String, Error>;
}

pub enum Entity {
    Group(String),
    User(String),
    Relation(String, String),
}

pub enum Command {
    Create(Entity),
    Delete(Entity),
    Read(Entity),
}

impl Command {
    fn new(text: &str) -> Result<Self, Error> {

        let mut words = text.split_whitespace();

        let command = words.next().ok_or(Error)?;
        let entity = words.next().ok_or(Error)?;
        let name1: String = words.next().ok_or(Error)?.parse().unwrap();
        let connector = words.next();
        let name2 = words.next();


        let entity = match entity {
            "group" => Entity::Group(name1),
            "user" => Entity::User(name1),
            "relation" => {
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