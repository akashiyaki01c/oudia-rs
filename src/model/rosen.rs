use crate::{
    model::{dia::Dia, eki::Eki, error::Error, jikoku::Jikoku, ressyasyubetsu::Ressyasyubetsu},
    opt::node::Node,
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rosen {
    rosenmei: String,
    eki: Vec<Eki>,
    ressyasyubetsu: Vec<Ressyasyubetsu>,
    dia: Vec<Dia>,
    kiten_jikoku: Jikoku,
    diagram_dgr_y_zahyou_kyori_default: usize,
    comment: String,
}
impl Rosen {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // Rosenmei
            if let Some(rosenmei) = dir.find("Rosenmei") {
                if let Node::Property(rosenmei) = rosenmei {
                    result.rosenmei = rosenmei.value.clone();
                }
            } else {
                todo!();
            }

            // Eki[]
            for node in dir.find_all("Eki") {
                result.eki.push(Eki::from_node(node)?);
            }

            // Ressyasyubetsu[]
            for node in dir.find_all("Ressyasyubetsu") {
                result.ressyasyubetsu.push(Ressyasyubetsu::from_node(node)?);
            }

            // Dia[]
            for node in dir.find_all("Dia") {
                // result.dia.push(Dia::from_node(node)?);
            }

            // KitenJikoku
            if let Some(kitenjikoku) = dir.find("KitenJikoku") {
                if let Node::Property(kitenjikoku) = kitenjikoku {
                    // result.kiten_jikoku = Jikoku::parse(kitenjikoku.value);
                }
            }

            // DiagramDgrYZahyouKyoriDefault
            result.diagram_dgr_y_zahyou_kyori_default = 60;
            if let Some(zahyou_kyori) = dir.find("DiagramDgrYZahyouKyoriDefault") {
                if let Node::Property(zahyou_kyori) = zahyou_kyori {
                    result.diagram_dgr_y_zahyou_kyori_default = zahyou_kyori
                        .value
                        .parse()
                        .map_err(|_| Error::DiagramDgrYZahyouKyoriDefaultError)?
                }
            }

            // Comment
            if let Some(comment) = dir.find("Comment") {
                if let Node::Property(comment) = comment {
                    result.comment = comment.value.clone();
                }
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }
}
