use crate::opt::{
    directory::Directory,
    error::Error,
    escape::unescape_text,
    node::Node,
    property::Property,
    source::{LocatedError, LocatedNode, NodeLocation, collect_locations},
};

pub fn deserialize_node(text: &str) -> Result<Vec<Node>, Error> {
    deserialize_node_with_locations(text)
        .map(|nodes| nodes.into_iter().map(|node| node.node).collect())
        .map_err(|error| error.error)
}

pub fn deserialize_node_with_locations(text: &str) -> Result<Vec<LocatedNode>, LocatedError> {
    let text = unescape_text(text.to_string());
    let lines: Vec<&str> = text.split_terminator("\r\n").collect();
    let mut index = 0;
    let mut nodes = vec![];
    while index < lines.len() {
        if let Some(node) = deserialize_node_inner(&lines, &mut index)? {
            nodes.push(node);
        }
    }

    Ok(nodes)
}

pub fn node_locations(nodes: &[LocatedNode]) -> Vec<NodeLocation> {
    collect_locations(nodes)
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

pub fn deserialize_node_inner(
    lines: &[&str],
    index: &mut usize,
) -> Result<Option<LocatedNode>, LocatedError> {
    let line = *index + 1;
    let first_line = lines[*index];
    *index += 1;

    if first_line.contains("=") {
        // property
        Ok(Some(LocatedNode::property(
            deserialize_property(first_line),
            line,
        )))
    } else if let Some(name) = first_line.strip_suffix(".") {
        // directory
        let mut nodes = vec![];
        let mut is_success = false;
        while *index < lines.len() {
            if lines[*index] == "." {
                is_success = true;
                *index += 1;
                break;
            }
            if let Some(node) = deserialize_node_inner(lines, index)? {
                nodes.push(node);
            }
        }
        if !is_success {
            return Err(LocatedError {
                error: Error::ContainerIsNotClosed,
                line,
            });
        }

        let directory =
            Directory::new_with_value(name, nodes.iter().map(|node| node.node.clone()).collect());
        Ok(Some(LocatedNode::directory(directory, line, nodes)))
    } else {
        if first_line.is_empty() {
            Ok(None)
        } else {
            Err(LocatedError {
                error: Error::UnknownStruct,
                line,
            })
        }
    }
}

#[test]
fn test() {
    use crate::model::oudia102::rosen_file_data::RosenFileData;

    let data = include_bytes!("../../test_data/keio.oud");
    let (data, _, _) = encoding_rs::SHIFT_JIS.decode(data);
    let result = deserialize_node(&data).unwrap();
    let _file =
        RosenFileData::from_node(&Node::Directory(Directory::new_with_value("ROOT", result)));
}

#[test]
fn test_1() {
    let data = include_bytes!("../../test_data/keio.oud");
    let (data, _, _) = encoding_rs::SHIFT_JIS.decode(data);
    let _result = deserialize_node_with_locations(&data).unwrap();
}

#[test]
fn located_nodes_keep_their_starting_lines() {
    let nodes = deserialize_node_with_locations("Root.\r\nName=value\r\n.\r\n").unwrap();

    assert_eq!(nodes[0].line, 1);
    assert_eq!(nodes[0].children[0].line, 2);
}

#[test]
fn unclosed_directory_reports_its_starting_line() {
    let error =
        deserialize_node_with_locations("Before=value\r\nRoot.\r\nName=value\r\n").unwrap_err();

    assert_eq!(error.line, 2);
    assert_eq!(error.error, Error::ContainerIsNotClosed);
}

#[test]
fn located_nodes_have_indexed_paths_for_repeated_names() {
    let nodes = deserialize_node_with_locations(
        "Rosen.\r\nEki.\r\nEkimei=A\r\n.\r\nEki.\r\nEkimei=B\r\n.\r\n.\r\n",
    )
    .unwrap();
    let locations = node_locations(&nodes);
    let paths: Vec<String> = locations
        .into_iter()
        .map(|location| location.path.to_string())
        .collect();

    assert_eq!(
        paths,
        vec![
            "ROOT.Rosen",
            "ROOT.Rosen.Eki[0]",
            "ROOT.Rosen.Eki[0].Ekimei",
            "ROOT.Rosen.Eki[1]",
            "ROOT.Rosen.Eki[1].Ekimei",
        ]
    );
}
