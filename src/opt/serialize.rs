use crate::opt::{escape::escape_text, node::Node};

pub fn serialize_node(node: &Node) -> String {
    match node {
        Node::Property(property) => format!(
            "{}={}",
            escape_text(property.name.clone()),
            escape_text(property.value.clone())
        ),
        Node::Directory(directory) => {
            let mut result = String::new();
            result += &format!("{}.\r\n", escape_text(directory.name.clone()));
            for node in &directory.values {
                result += &serialize_node(node);
                result += "\r\n";
            }
            result += ".\r\n";
            result
        }
    }
}

#[test]
fn test() {
    use crate::opt::{directory::Directory, property::Property};
    let root = Node::Directory(Directory::new_with_value(
        "Station",
        vec![
            Node::Property(Property::new_with_value("Name", "梅田".to_string())),
            Node::Property(Property::new_with_value("Name", "福島".to_string())),
        ],
    ));

    println!("{}", serialize_node(&root));
}
