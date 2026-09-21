use crate::{
    model::{error::Error, oudiasecond101::Ressya},
    opt::{directory::Directory, node::Node, property::Property},
};

const KEY_DIA: &str = "Dia";
const KEY_DIA_NAME: &str = "DiaName";
const KEY_MAIN_BACK_COLOR_INDEX: &str = "MainBackColorIndex";
const KEY_SUB_BACK_COLOR_INDEX: &str = "SubBackColorIndex";
const KEY_BACK_PATTERN_INDEX: &str = "BackPatternIndex";
const KEY_KUDARI: &str = "Kudari";
const KEY_NOBORI: &str = "Nobori";

/// 1つの時刻表を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Dia {
    /// 時刻表名
    name: String,
    main_back_color_index: usize,
    sub_back_color_index: usize,
    back_pattern_index: usize,
    /// 下り列車のリスト
    outbound_trains: Vec<Ressya>,
    /// 上り列車のリスト
    inbound_trains: Vec<Ressya>,
}
impl Dia {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // DiaName [Required]
            if let Some(Node::Property(dia_name)) = dir.find(KEY_DIA_NAME) {
                if dia_name.value.is_empty() {
                    return Err(Error::InvalidValue(
                        dia_name.name.to_string(),
                        dia_name.value.to_string(),
                    ));
                }
                result.name = dia_name.value.to_string();
            } else {
                return Err(Error::KeyIsNotFound("DiaName".to_string()));
            }

            // MainBackColorIndex
            if let Some(Node::Property(prop)) = dir.find(KEY_MAIN_BACK_COLOR_INDEX) {
                result.main_back_color_index =
                    prop.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_MAIN_BACK_COLOR_INDEX.to_string(),
                        value: prop.value.to_string(),
                    })?;
            }

            // SubBackColorIndex
            if let Some(Node::Property(prop)) = dir.find(KEY_SUB_BACK_COLOR_INDEX) {
                result.sub_back_color_index =
                    prop.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_SUB_BACK_COLOR_INDEX.to_string(),
                        value: prop.value.to_string(),
                    })?;
            }

            // BackPatternIndex
            if let Some(Node::Property(prop)) = dir.find(KEY_BACK_PATTERN_INDEX) {
                result.back_pattern_index =
                    prop.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_BACK_PATTERN_INDEX.to_string(),
                        value: prop.value.to_string(),
                    })?;
            }

            // Kudari
            if let Some(Node::Directory(kudari)) = dir.find(KEY_KUDARI) {
                if !kudari.is_array() {
                    return Err(Error::ExpectedArray(KEY_KUDARI.to_string()));
                }
                let kudari: Result<Vec<Ressya>, Error> =
                    kudari.values.iter().map(Ressya::from_node).collect();
                result.outbound_trains = kudari?;
            } else {
                return Err(Error::KeyIsNotFound(KEY_KUDARI.to_string()));
            }

            // Nobori
            if let Some(Node::Directory(nobori)) = dir.find(KEY_NOBORI) {
                if !nobori.is_array() {
                    return Err(Error::ExpectedArray(KEY_NOBORI.to_string()));
                }
                let nobori: Result<Vec<Ressya>, Error> =
                    nobori.values.iter().map(Ressya::from_node).collect();
                result.inbound_trains = nobori?;
            } else {
                return Err(Error::KeyIsNotFound(KEY_NOBORI.to_string()));
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        Node::Directory(Directory::new_with_value(
            KEY_DIA,
            vec![
                property(KEY_DIA_NAME, &self.name),
                property(
                    KEY_MAIN_BACK_COLOR_INDEX,
                    &self.main_back_color_index.to_string(),
                ),
                property(
                    KEY_SUB_BACK_COLOR_INDEX,
                    &self.sub_back_color_index.to_string(),
                ),
                property(KEY_BACK_PATTERN_INDEX, &self.back_pattern_index.to_string()),
                direction(KEY_KUDARI, &self.outbound_trains),
                direction(KEY_NOBORI, &self.inbound_trains),
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
