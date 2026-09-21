use std::{fmt, str::FromStr};

use crate::{
    model::{error::Error, oudia102::ekijikoku::Ekijikoku},
    opt::{directory::Directory, node::Node, property::Property},
};

const KEY_RESSYA: &str = "Ressya";
const KEY_HOUKOU: &str = "Houkou";
const KEY_SYUBETSU: &str = "Syubetsu";
const KEY_RESSYABANGOU: &str = "Ressyabangou";
const KEY_RESSYAMEI: &str = "Ressyamei";
const KEY_GOSUU: &str = "Gosuu";
const KEY_EKI_JIKOKU: &str = "EkiJikoku";
const KEY_BIKOU: &str = "Bikou";

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
            if let Some(Node::Property(houkou)) = dir.find(KEY_HOUKOU) {
                result.houkou = Houkou::from_str(&houkou.value)?;
            } else {
                result.houkou = Houkou::Null;
                return Ok(result);
            }

            // Syubetsu
            if let Some(Node::Property(syubetsu)) = dir.find(KEY_SYUBETSU) {
                result.ressyasyubetsu_index =
                    syubetsu.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_SYUBETSU.to_string(),
                        value: syubetsu.value.to_string(),
                    })?;
            }

            // Ressyabangou
            if let Some(Node::Property(ressyabangou)) = dir.find(KEY_RESSYABANGOU) {
                result.ressyabangou = ressyabangou.value.to_string();
            }

            // Ressyamei
            if let Some(Node::Property(ressyamei)) = dir.find(KEY_RESSYAMEI) {
                result.ressyamei = ressyamei.value.to_string();
            }

            // Gosuu
            if let Some(Node::Property(gosuu)) = dir.find(KEY_GOSUU) {
                result.gousuu = gosuu.value.to_string();
            }

            // Ekijikoku
            if let Some(Node::Property(ekijikoku)) = dir.find(KEY_EKI_JIKOKU) {
                let ekijikoku: Result<Vec<Ekijikoku>, Error> = ekijikoku
                    .value
                    .split(",")
                    .map(Ekijikoku::from_str)
                    .collect();
                result.ekijikoku = ekijikoku?;
            }

            // Bikou
            if let Some(Node::Property(bikou)) = dir.find(KEY_BIKOU) {
                result.bikou = bikou.value.to_string();
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        if self.houkou == Houkou::Null {
            return Node::Directory(Directory::new_with_value(KEY_RESSYA, vec![]));
        }
        let mut values = vec![
            property(KEY_HOUKOU, self.houkou.to_string()),
            property(KEY_SYUBETSU, self.ressyasyubetsu_index.to_string()),
        ];
        if !self.ressyabangou.is_empty() {
            values.push(property(KEY_RESSYABANGOU, &self.ressyabangou));
        }
        if !self.ressyamei.is_empty() {
            values.push(property(KEY_RESSYAMEI, &self.ressyamei));
        }
        if !self.gousuu.is_empty() {
            values.push(property(KEY_GOSUU, &self.gousuu));
        }
        values.push(property(
            KEY_EKI_JIKOKU,
            self.ekijikoku
                .iter()
                .map(Ekijikoku::to_oudia_string)
                .collect::<Vec<_>>()
                .join(","),
        ));
        if !self.bikou.is_empty() {
            values.push(property(KEY_BIKOU, &self.bikou));
        }
        Node::Directory(Directory::new_with_value(KEY_RESSYA, values))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}

const KEY_KUDARI: &str = "Kudari";
const KEY_NOBORI: &str = "Nobori";

/// 列車の運転方向を表す列挙体
#[derive(Debug, Default, PartialEq, Clone)]
pub enum Houkou {
    #[default]
    Kudari,
    Nobori,
    Null,
}
impl FromStr for Houkou {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            KEY_KUDARI => Ok(Self::Kudari),
            KEY_NOBORI => Ok(Self::Nobori),
            _ => Err(Error::InvalidEnum {
                field: "Houkou".to_string(),
                value: value.to_string(),
            }),
        }
    }

}

impl fmt::Display for Houkou {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Houkou::Kudari => formatter.write_str(KEY_KUDARI),
            Houkou::Nobori => formatter.write_str(KEY_NOBORI),
            Houkou::Null => unreachable!(),
        }
    }
}
