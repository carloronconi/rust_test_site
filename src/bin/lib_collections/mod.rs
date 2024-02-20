use std::fmt::Error;
use regex::Regex;

const COMMAND_FORMAT: &str = "^(add membership .* to .*|remove membership .* from .*|detail membership .* from .*)$";

pub trait RunCommand {
    fn submit(command: &str) -> Result<String, Error>;
}

pub enum Entity {
    Group,
    User,
    Relation,
}

pub enum Command {
    Create(Entity),
    Delete(Entity),
    Read(Entity),
}

impl Command {
    fn new(text: &str) -> Result<Self, Error> {
        let reg = Regex::new(COMMAND_FORMAT).unwrap();
        if !reg.is_match(text) { return Err(Error) }

        let mut words = text.split_whitespace();

        let command = words.next().unwrap();
        let entity = words.next().unwrap();

        let entity = match entity {
            "group" => Entity::Group,
            "user" => Entity::User,
            "relation" => Entity::Relation,
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