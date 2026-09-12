use std::{iter::Peekable, str::Split};

use crate::opt::{directory::Directory, error::Error, node::Node, property::Property};

pub fn deserialize_node(text: &str) -> Result<Node, Error> {
    let mut lines = text.split("\n").peekable();
    let result = deserialize_node_inner(&mut lines);
    if let Some(_) = lines.peek() {
        return Err(Error::ContainerAborted);
    }

    result
}

pub fn deserialize_property(text: &str) -> Property {
    assert!(text.contains("="));

    let equal_index = text.find("=").unwrap();
    let name = &text[..equal_index];
    let value = &text[&equal_index + 1..];

    Property {
        name: name.to_string(),
        value: value.to_string(),
    }
}

pub fn deserialize_node_inner(iter: &mut Peekable<Split<&str>>) -> Result<Node, Error> {
    let first_line = iter.next().unwrap();

    if first_line.ends_with(".") {
        let name = &first_line[..first_line.len() - 2];
        // directory
        let mut nodes = vec![];
        let mut is_success = false;
        while let Some(next_line) = iter.next() {
            if next_line == "." {
                iter.next();
                is_success = true;
                break;
            }
            nodes.push(deserialize_node_inner(iter)?);
        }
        if !is_success {
            return Err(Error::ContainerIsNotClosed);
        }

        Ok(Node::Directory(Directory::new_with_value(name, nodes)))
    } else if first_line.contains("=") {
        // property
        Ok(Node::Property(deserialize_property(first_line)))
    } else {
        unreachable!()
    }
}
