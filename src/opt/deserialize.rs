use std::{iter::Peekable, str::Split};

use crate::{model::rosen_file_data::RosenFileData, opt::{directory::Directory, error::Error, node::Node, property::Property}};

pub fn deserialize_node(text: &str) -> Result<Vec<Node>, Error> {
    let mut lines = text.split("\r\n").peekable();
	let mut nodes = vec![];
	while lines.peek().is_some() {
		if let Some(node) = deserialize_node_inner(&mut lines)? {
			nodes.push(node);
		}
	}
    if let Some(_) = lines.peek() {
        return Err(Error::ContainerAborted);
    }

    Ok(nodes)
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

pub fn deserialize_node_inner(iter: &mut Peekable<Split<&str>>) -> Result<Option<Node>, Error> {
    let first_line = iter.next().unwrap();

    if first_line.ends_with(".") {
		println!("[Directory] {} [/Directory]", first_line);
        let name = &first_line[..first_line.len() - 1];
        // directory
        let mut nodes = vec![];
        let mut is_success = false;
        while let Some(next_line) = iter.peek() {
            if *next_line == "." {
                is_success = true;
				iter.next();
				println!("end directory {}", name);
                break;
            }
			if let Some(node) = deserialize_node_inner(iter)? {
				nodes.push(node);
			}
        }
        if !is_success {
            return Err(Error::ContainerIsNotClosed);
        }

        Ok(Some(Node::Directory(Directory::new_with_value(name, nodes))))
    } else if first_line.contains("=") {
        // property
		println!("{} [Property]", first_line);
        Ok(Some(Node::Property(deserialize_property(first_line))))
    } else {
        Ok(None)
    }
}

#[test]
fn test() {
	let data = include_bytes!("../../test_data/kto.oud");
    let (data, _, _) = encoding_rs::SHIFT_JIS.decode(data);
	let result = deserialize_node(&data).unwrap();
    let file = RosenFileData::from_node(&Node::Directory(Directory::new_with_value("ROOT", result)));
	println!("{:?}", file);
}