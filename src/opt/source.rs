use crate::opt::{directory::Directory, error::Error, node::Node, property::Property};
use std::fmt;

/// A syntax error together with the 1-based source line where it was detected.
#[derive(Debug, PartialEq)]
pub struct LocatedError {
    pub error: Error,
    pub line: usize,
}

/// A parsed node together with its source line and located children.
#[derive(Debug, PartialEq, Clone)]
pub struct LocatedNode {
    pub node: Node,
    pub line: usize,
    pub children: Vec<Self>,
}

/// A path to a node in an OuPropertiesText document.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodePath {
    segments: Vec<NodePathSegment>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum NodePathSegment {
    Name(String),
    Index(usize),
}

impl NodePath {
    pub fn root() -> Self {
        Self {
            segments: vec![NodePathSegment::Name("ROOT".to_string())],
        }
    }

    fn child(&self, name: &str, index: Option<usize>) -> Self {
        let mut segments = self.segments.clone();
        segments.push(NodePathSegment::Name(name.to_string()));
        if let Some(index) = index {
            segments.push(NodePathSegment::Index(index));
        }
        Self { segments }
    }
}

impl fmt::Display for NodePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, segment) in self.segments.iter().enumerate() {
            match segment {
                NodePathSegment::Name(name) => {
                    if position > 0 {
                        formatter.write_str(".")?;
                    }
                    formatter.write_str(name)?;
                }
                NodePathSegment::Index(index) => write!(formatter, "[{index}]")?,
            }
        }
        Ok(())
    }
}

/// A source location for a parsed node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NodeLocation {
    pub line: usize,
    pub path: NodePath,
}

impl LocatedNode {
    pub(crate) fn property(property: Property, line: usize) -> Self {
        Self {
            node: Node::Property(property),
            line,
            children: vec![],
        }
    }

    pub(crate) fn directory(directory: Directory, line: usize, children: Vec<Self>) -> Self {
        Self {
            node: Node::Directory(directory),
            line,
            children,
        }
    }

    fn collect_locations(
        &self,
        parent_path: &NodePath,
        index: usize,
        siblings: &[Self],
        locations: &mut Vec<NodeLocation>,
    ) {
        let name = self.node.get_name();
        let same_name_count = siblings
            .iter()
            .filter(|sibling| sibling.node.get_name() == name)
            .count();
        let same_name_index = if same_name_count > 1 {
            Some(
                siblings[..index]
                    .iter()
                    .filter(|sibling| sibling.node.get_name() == name)
                    .count(),
            )
        } else {
            None
        };
        let path = parent_path.child(name, same_name_index);
        locations.push(NodeLocation {
            line: self.line,
            path: path.clone(),
        });

        for (child_index, child) in self.children.iter().enumerate() {
            child.collect_locations(&path, child_index, &self.children, locations);
        }
    }
}

pub(crate) fn collect_locations(nodes: &[LocatedNode]) -> Vec<NodeLocation> {
    let root = NodePath::root();
    let mut locations = vec![];
    for (index, node) in nodes.iter().enumerate() {
        node.collect_locations(&root, index, nodes, &mut locations);
    }
    locations
}

pub(crate) fn root_location(nodes: &[LocatedNode]) -> Option<NodeLocation> {
    nodes.first().map(|node| NodeLocation {
        line: node.line,
        path: NodePath::root(),
    })
}

pub(crate) fn find_location_by_name(nodes: &[LocatedNode], name: &str) -> Option<NodeLocation> {
    find_location(nodes, |node| node.get_name() == name)
}

pub(crate) fn find_location<F>(nodes: &[LocatedNode], predicate: F) -> Option<NodeLocation>
where
    F: Fn(&Node) -> bool,
{
    fn find_in<F>(
        node: &LocatedNode,
        parent_path: &NodePath,
        index: usize,
        siblings: &[LocatedNode],
        predicate: &F,
    ) -> Option<NodeLocation>
    where
        F: Fn(&Node) -> bool,
    {
        let name = node.node.get_name();
        let same_name_count = siblings
            .iter()
            .filter(|sibling| sibling.node.get_name() == name)
            .count();
        let same_name_index = (same_name_count > 1).then(|| {
            siblings[..index]
                .iter()
                .filter(|sibling| sibling.node.get_name() == name)
                .count()
        });
        let path = parent_path.child(name, same_name_index);

        if predicate(&node.node) {
            return Some(NodeLocation {
                line: node.line,
                path,
            });
        }

        for (child_index, child) in node.children.iter().enumerate() {
            if let Some(location) = find_in(child, &path, child_index, &node.children, predicate) {
                return Some(location);
            }
        }
        None
    }

    let root = NodePath::root();
    for (index, node) in nodes.iter().enumerate() {
        if let Some(location) = find_in(node, &root, index, nodes, &predicate) {
            return Some(location);
        }
    }
    None
}

pub(crate) fn find_parent_location_for_missing(
    nodes: &[LocatedNode],
    missing_name: &str,
    parent_name: &str,
) -> Option<NodeLocation> {
    fn find_in(
        node: &LocatedNode,
        parent_path: &NodePath,
        index: usize,
        siblings: &[LocatedNode],
        missing_name: &str,
        parent_name: &str,
    ) -> Option<NodeLocation> {
        let name = node.node.get_name();
        let same_name_count = siblings
            .iter()
            .filter(|sibling| sibling.node.get_name() == name)
            .count();
        let same_name_index = (same_name_count > 1).then(|| {
            siblings[..index]
                .iter()
                .filter(|sibling| sibling.node.get_name() == name)
                .count()
        });
        let path = parent_path.child(name, same_name_index);

        if let Node::Directory(directory) = &node.node
            && node.node.get_name() == parent_name
            && directory
                .values
                .iter()
                .all(|child| child.get_name() != missing_name)
        {
            return Some(NodeLocation {
                line: node.line,
                path,
            });
        }

        for (child_index, child) in node.children.iter().enumerate() {
            if let Some(location) = find_in(
                child,
                &path,
                child_index,
                &node.children,
                missing_name,
                parent_name,
            ) {
                return Some(location);
            }
        }
        None
    }

    let root = NodePath::root();
    for (index, node) in nodes.iter().enumerate() {
        if let Some(location) = find_in(node, &root, index, nodes, missing_name, parent_name) {
            return Some(location);
        }
    }
    None
}
