use std::collections::HashMap;
use std::fmt::Error;

#[derive(Debug)]
pub struct Storage {
    users: Vec<String>,
    groups: HashMap<String, Vec<String>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            users: vec![],
            groups: HashMap::new(),
        }
    }

    pub fn submit(&mut self, command: &Command) -> Result<(), Error> {
        match command {
            Command::Create(e) => {
                match e {
                    Entity::Group(n) => {
                        self.groups
                            .entry(n.clone())
                            .or_insert(vec![]); }
                    Entity::User(n) => self.users.push(n.clone()),
                    Entity::Relation(usr, group) => {
                        self.groups
                            .entry(group.clone())
                            .or_insert(vec![])
                            .push(usr.clone()); }
                }
                Ok(()) // creation can't fail
            }
            Command::Delete(e) => {
                match e {
                    Entity::Group(n) => {
                        if self.groups.remove(n.as_str()).is_none() {
                            return Err(Error)
                        }}
                    Entity::User(n) => {
                        if !self.users.contains(n) {
                            return Err(Error)
                        }
                        self.users.retain(|u| !u.eq(n)); }
                    Entity::Relation(usr, group) => {
                        if !self.groups.get(group.as_str()).is_some_and(|g| g.contains(usr)) {
                            return Err(Error)
                        }
                        self.groups
                            .entry(group.clone())
                            .and_modify(|v| v.retain(|u| !u.eq(usr)));
                    }
                }
                Ok(())
            }
            Command::Read(_) => {
                println!("{:?}", self);
                Ok(())
            }
        }
    }
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