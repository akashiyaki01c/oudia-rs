use crate::{
    model::{error::Error, oudia102::{dia::Dia, eki::Station, jikoku::Jikoku, ressyasyubetsu::Ressyasyubetsu}}, opt::{directory::Directory, node::Node, property::Property},
};

/// 路線を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rosen {
    /// 路線名
    rosenmei: String,
    /// 駅の一覧
    eki: Vec<Station>,
    /// 列車種別の一覧
    ressyasyubetsu: Vec<Ressyasyubetsu>,
    /// 時刻表の一覧
    dia: Vec<Dia>,
    /// ダイヤグラムの起点時刻
    kiten_jikoku: Jikoku,
    /// ダイヤグラムにおける駅間座標の既定値
    diagram_dgr_y_zahyou_kyori_default: usize,
    ///　コメント
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
                result.eki.push(Station::from_node(node)?);
            }

            // Ressyasyubetsu[]
            for node in dir.find_all("Ressyasyubetsu") {
                result.ressyasyubetsu.push(Ressyasyubetsu::from_node(node)?);
            }

            // Dia[]
            for node in dir.find_all("Dia") {
                result.dia.push(Dia::from_node(node)?);
            }

            // KitenJikoku
            if let Some(kitenjikoku) = dir.find("KitenJikoku")
                && let Node::Property(kitenjikoku) = kitenjikoku
            {
                result.kiten_jikoku = Jikoku::from_str(&kitenjikoku.value)?;
            }

            // DiagramDgrYZahyouKyoriDefault
            result.diagram_dgr_y_zahyou_kyori_default = 60;
            if let Some(zahyou_kyori) = dir.find("DiagramDgrYZahyouKyoriDefault")
                && let Node::Property(zahyou_kyori) = zahyou_kyori
            {
                result.diagram_dgr_y_zahyou_kyori_default = zahyou_kyori
                    .value
                    .parse()
                    .map_err(|_| Error::InvalidNumber {
                        field: "DiagramDgrYZahyouKyoriDefault".to_string(),
                        value: zahyou_kyori.value.to_string(),
                    })?
            }

            // Comment
            if let Some(comment) = dir.find("Comment")
                && let Node::Property(comment) = comment
            {
                result.comment = comment.value.clone();
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        let mut values = vec![property("Rosenmei", &self.rosenmei)];
        values.extend(self.eki.iter().map(Station::to_node));
        values.extend(self.ressyasyubetsu.iter().map(Ressyasyubetsu::to_node));
        values.extend(self.dia.iter().map(Dia::to_node));
        values.push(property("KitenJikoku", self.kiten_jikoku.to_oudia_string()));
        values.push(property(
            "DiagramDgrYZahyouKyoriDefault",
            self.diagram_dgr_y_zahyou_kyori_default.to_string(),
        ));
        values.push(property("Comment", &self.comment));
        Node::Directory(Directory::new_with_value("Rosen", values))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}
