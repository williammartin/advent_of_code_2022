pub mod input;
pub mod part1;
pub mod part2;

use std::str::FromStr;

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

#[derive(Debug, Clone, Copy)]
pub struct NodeIndex(usize);

#[derive(Debug)]
pub enum Node {
    File {
        name: String,
        size: u32,
        parent: NodeIndex,
    },
    Directory {
        name: String,
        children: Vec<NodeIndex>,
        parent: NodeIndex,
    },
}

#[derive(Debug)]
pub struct Filesystem(Vec<Node>);

impl Filesystem {
    fn node_at(&self, i: NodeIndex) -> Option<&Node> {
        self.0.get(i.0)
    }

    // Assumption that we never navigate to the same dir twice, and a dir can never have two parents
    fn add_dir(&mut self, name: String, parent_node_idx: NodeIndex) -> NodeIndex {
        let insert_idx = self.0.len();
        let parent_node = self.0.get_mut(parent_node_idx.0).expect(&format!(
            "to have a node at parent index {:?}",
            parent_node_idx
        ));

        match parent_node {
            Node::File { .. } => panic!("cannot add children to a file"), // probably a better way to avoid this at compilation time
            Node::Directory { children, .. } => children.push(NodeIndex(insert_idx)),
        }

        self.0.push(Node::Directory {
            name,
            children: vec![],
            parent: parent_node_idx,
        });

        NodeIndex(insert_idx)
    }

    // I'm sure I can reduce this duplication but ehhhh
    fn add_file(&mut self, name: String, size: u32, parent_node_idx: NodeIndex) -> NodeIndex {
        let insert_idx = self.0.len();
        let parent_node = self.0.get_mut(parent_node_idx.0).expect(&format!(
            "to have a node at parent index {:?}",
            parent_node_idx
        ));

        match parent_node {
            Node::File { .. } => panic!("cannot add children to a file"), // probably a better way to avoid this at compilation time
            Node::Directory { children, .. } => children.push(NodeIndex(insert_idx)),
        }

        self.0.push(Node::File {
            name,
            size,
            parent: parent_node_idx,
        });

        NodeIndex(insert_idx)
    }

    fn size_of(&self, node_idx: NodeIndex) -> u32 {
        let node = self
            .0
            .get(node_idx.0)
            .expect(&format!("to have a node at index {:?}", node_idx));

        match node {
            Node::File { size, .. } => *size,
            Node::Directory { children, .. } => children
                .iter()
                .fold(0, |acc, child_idx| acc + self.size_of(*child_idx)),
        }
    }

    fn dirs(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(idx, node)| match node {
                Node::File { .. } => None,
                Node::Directory { .. } => Some(NodeIndex(idx)),
            })
    }
}

impl FromIterator<Line> for Filesystem {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let mut fs = Filesystem(vec![Node::Directory {
            name: "/".to_owned(),
            children: Vec::new(),
            parent: NodeIndex(0), // parent of root is always root
        }]);
        let mut current_node_idx: NodeIndex = NodeIndex(0); // Create a filesystem with the root node

        for i in iter {
            match i {
                Line::Command(cmd) => match cmd {
                    Command::ChangeDirectory { to } => {
                        match to.as_str() {
                            "/" => {
                                current_node_idx = NodeIndex(0);
                            }
                            ".." => {
                                let current_node = fs
                                    .node_at(current_node_idx)
                                    .expect(&format!("to have a node at {:?}", current_node_idx));

                                match current_node {
                                    Node::File { .. } => {
                                        panic!("files should not be marked as a parent")
                                    } // probably a better way to avoid this at compilation time
                                    Node::Directory { parent, .. } => current_node_idx = *parent, // hmm
                                }
                            }
                            dir => {
                                current_node_idx = fs.add_dir(dir.to_owned(), current_node_idx);
                            }
                        };
                    }
                    Command::List => continue,
                },
                Line::Directory(name) => {
                    // do nothing I guess, cause we create lazily on cd
                }
                Line::File(file) => {
                    fs.add_file(file.name, file.size, current_node_idx);
                }
            }
        }

        fs
    }
}

pub type Input = Filesystem;

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
