use std::str::FromStr;

use crate::{
    model::{ekijikoku::Ekijikoku, error::Error},
    opt::node::Node,
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ressya {
    ressyasyubetsu_index: usize,
    ressyabangou: String,
    ressyamei: String,
    gousuu: String,
    ekijikoku: Vec<Ekijikoku>,
    bikou: String,
}
impl Ressya {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
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
}
