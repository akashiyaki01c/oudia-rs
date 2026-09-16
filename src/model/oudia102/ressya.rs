use std::str::FromStr;

use crate::{
    model::oudia102::{ekijikoku::Ekijikoku, error::Error},
    opt::{directory::Directory, node::Node, property::Property},
};

/// 1つの列車を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ressya {
    /// 列車の方向
    houkou: Houkou,
    /// 列車種別のインデックス
    ressyasyubetsu_index: usize,
    /// 列車番号
    ressyabangou: String,
    /// 列車名
    ressyamei: String,
    /// 号数
    gousuu: String,
    /// 駅時刻のリスト
    ekijikoku: Vec<Ekijikoku>,
    /// 備考
    bikou: String,
}
impl Ressya {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // Houkou
            if let Some(Node::Property(houkou)) = dir.find("Houkou") {
                result.houkou = Houkou::from_str(&houkou.value)?;
            }

            // Syubetsu
            if let Some(Node::Property(syubetsu)) = dir.find("Syubetsu") {
                result.ressyasyubetsu_index =
                    syubetsu.value.parse().map_err(|_| Error::TodoError)?;
            }

            // Ressyabangou
            if let Some(Node::Property(ressyabangou)) = dir.find("Ressyabangou") {
                result.ressyabangou = ressyabangou.value.to_string();
            }

            // Ressyamei
            if let Some(Node::Property(ressyamei)) = dir.find("Ressyamei") {
                result.ressyamei = ressyamei.value.to_string();
            }

            // Gosuu
            if let Some(Node::Property(gosuu)) = dir.find("Gosuu") {
                result.gousuu = gosuu.value.to_string();
            }

            // Ekijikoku
            if let Some(Node::Property(ekijikoku)) = dir.find("EkiJikoku") {
                let ekijikoku: Result<Vec<Ekijikoku>, Error> = ekijikoku
                    .value
                    .split(",")
                    .map(Ekijikoku::from_str)
                    .collect();
                result.ekijikoku = ekijikoku?;
            }

            // Bikou
            if let Some(Node::Property(bikou)) = dir.find("Bikou") {
                result.bikou = bikou.value.to_string();
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        Node::Directory(Directory::new_with_value(
            "Ressya",
            vec![
                property("Houkou", self.houkou.to_string()),
                property("Syubetsu", self.ressyasyubetsu_index.to_string()),
                property("Ressyabangou", &self.ressyabangou),
                property("Ressyamei", &self.ressyamei),
                property("Gosuu", &self.gousuu),
                property(
                    "EkiJikoku",
                    self.ekijikoku
                        .iter()
                        .map(Ekijikoku::to_oudia_string)
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                property("Bikou", &self.bikou),
            ],
        ))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}

/// 列車の運転方向を表す列挙体
#[derive(Debug, Default, PartialEq, Clone)]
pub enum Houkou {
    #[default]
    Kudari,
    Nobori,
}
impl Houkou {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "Kudari" => Ok(Self::Kudari),
            "Nobori" => Ok(Self::Nobori),
            _ => Err(Error::TodoError)
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            Houkou::Kudari => "Kudari".to_string(),
            Houkou::Nobori => "Nobori".to_string(),
        }
    }
}