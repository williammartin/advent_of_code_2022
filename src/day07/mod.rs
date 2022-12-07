pub mod input;
pub mod part1;
pub mod part2;

use std::{
    borrow::BorrowMut,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
    str::FromStr,
};

use crate::{Output, Part};

#[derive(Debug)]
pub enum Command {
    ChangeDirectory { to: String },
    List,
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Directory(String),
    File(File),
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(command) = s.strip_prefix("$ ") {
            if let Some(dir) = command.strip_prefix("cd ") {
                Ok(Line::Command(Command::ChangeDirectory {
                    to: dir.to_owned(),
                }))
            } else {
                Ok(Line::Command(Command::List))
            }
        } else if let Some(dir) = s.strip_prefix("dir ") {
            Ok(Line::Directory(dir.to_owned()))
        } else {
            let (size, name) = s
                .split_once(" ")
                .ok_or("failed to parse line, expected a file")?;
            let size = size
                .parse::<u32>()
                .map_err(|_| "failed to parse file size")?;

            Ok(Line::File(File {
                size,
                name: name.to_owned(),
            }))
        }
    }
}

#[derive(Debug)]
pub struct Directory {
    parent: Option<Rc<Directory>>,
    directories: HashMap<String, Directory>,
    files: Vec<File>,
}

impl Directory {
    fn new(name: &str, parent: Option<Rc<Directory>>) -> Directory {
        Directory {
            parent,
            directories: HashMap::new(),
            files: Vec::new(),
        }
    }

    fn add_dir(&mut self, name: &str) -> &mut Directory {
        let new_dir = Directory::new(name, Some(Rc::new(*self)));
        self.directories.entry(name.to_owned()).or_insert(new_dir)
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.files.push(File {
            name: name.to_owned(),
            size,
        })
    }
}

impl FromIterator<Line> for Directory {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let mut root = Directory::new("/", None);
        let mut pwd = Rc::new(&mut root);

        for i in iter {
            match i {
                Line::Command(cmd) => match cmd {
                    Command::ChangeDirectory { to } => {
                        if to == "/" {
                            pwd = Rc::new(&mut root);
                        }

                        if to == ".." {
                            pwd = pwd.parent.expect("to have a parent").borrow_mut();
                            // pwd_stack.pop(); // ignore popped dir
                        }

                        let new_dir = pwd_stack
                            .last_mut()
                            .expect("to always have a pwd")
                            .add_dir(to.as_str());
                        pwd_stack.push(new_dir);
                    }
                    Command::List => continue,
                },
                Line::Directory(name) => {
                    pwd_stack
                        .last_mut()
                        .expect("to always have a pwd")
                        .add_dir(name.as_str()); // no need to push onto pwd_stack, just noting there is a dir
                }
                Line::File(file) => {
                    pwd_stack
                        .last_mut()
                        .expect("to always have a pwd")
                        .add_file(file.name, file.size);
                }
            }
        }

        root
    }
}

pub type Input = Directory;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
