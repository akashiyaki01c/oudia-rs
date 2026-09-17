use crate::{
    model::oudia102::{Error, Ressya},
    opt::{directory::Directory, node::Node, property::Property},
};

/// 1つの時刻表を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Dia {
    /// 時刻表名
    dia_name: String,
    /// 下り列車のリスト
    kudari: Vec<Ressya>,
    /// 上り列車のリスト
    nobori: Vec<Ressya>,
}
impl Dia {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // DiaName [Required]
            if let Some(Node::Property(dia_name)) = dir.find("DiaName") {
                if dia_name.value.is_empty() {
                    return Err(Error::InvalidValue(
                        dia_name.name.to_string(),
                        dia_name.value.to_string(),
                    ));
                }
                result.dia_name = dia_name.value.to_string();
            } else {
                return Err(Error::KeyIsNotFound("DiaName".to_string()));
            }

            // Kudari
            if let Some(Node::Directory(kudari)) = dir.find("Kudari") {
                if !kudari.is_array() {
                    return Err(Error::TodoError);
                }
                let kudari: Result<Vec<Ressya>, Error> =
                    kudari.values.iter().map(Ressya::from_node).collect();
                result.kudari = kudari?;
            } else {
                return Err(Error::KeyIsNotFound("Kudari".to_string()));
            }

            // Nobori
            if let Some(Node::Directory(nobori)) = dir.find("Nobori") {
                if !nobori.is_array() {
                    return Err(Error::TodoError);
                }
                let nobori: Result<Vec<Ressya>, Error> =
                    nobori.values.iter().map(Ressya::from_node).collect();
                result.nobori = nobori?;
            } else {
                return Err(Error::KeyIsNotFound("Nobori".to_string()));
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        Node::Directory(Directory::new_with_value(
            "Dia",
            vec![
                property("DiaName", &self.dia_name),
                direction("Kudari", &self.kudari),
                direction("Nobori", &self.nobori),
            ],
        ))
    }
}

fn direction(name: &str, ressya: &[Ressya]) -> Node {
    Node::Directory(Directory::new_with_value(
        name,
        ressya.iter().map(Ressya::to_node).collect(),
    ))
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}
