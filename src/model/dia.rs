use crate::{
    model::{error::Error, ressya::Ressya},
    opt::node::Node,
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Dia {
    dia_name: String,
    kudari: Vec<Ressya>,
    nobori: Vec<Ressya>,
}
impl Dia {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // DiaName
            if let Some(Node::Property(dia_name)) = dir.find("DiaName") {
                result.dia_name = dia_name.value.to_string();
            }

            // Kudari
            if let Some(Node::Directory(kudari)) = dir.find("Kudari") {
                if !kudari.is_array() {
                    return Err(Error::TodoError);
                }
                let kudari: Result<Vec<Ressya>, Error> =
                    kudari.values.iter().map(Ressya::from_node).collect();
				result.kudari = kudari?;
            }

			// Nobori
            if let Some(Node::Directory(nobori)) = dir.find("Nobori") {
                if !nobori.is_array() {
                    return Err(Error::TodoError);
                }
                let nobori: Result<Vec<Ressya>, Error> =
                    nobori.values.iter().map(Ressya::from_node).collect();
				result.nobori = nobori?;
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }
}
