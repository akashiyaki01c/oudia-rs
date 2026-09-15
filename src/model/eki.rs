use crate::{model::error::Error, opt::node::Node};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Eki {
    ekimei: String,
    ekijikokukeisiki: Ekijikokukeisiki,
    ekikibo: Ekikibo,
    kyoukaisen: bool,
    diagram_ressyajouhou_hyouji_kudari: DiagramRessyajouhouHyouji,
    diagram_ressyajouhou_hyouji_nobori: DiagramRessyajouhouHyouji,
}
impl Eki {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // Ekimei
            if let Some(ekimei) = dir.find("Ekimei") {
                if let Node::Property(ekimei) = ekimei {
                    result.ekimei = ekimei.value.clone();
                }
            } else {
                todo!();
            }

            // Ekijikokukeisiki
            if let Some(ekimei) = dir.find("Ekijikokukeisiki") {
                if let Node::Property(ekimei) = ekimei {
                    result.ekijikokukeisiki = Ekijikokukeisiki::from_str(&ekimei.value)?;
                }
            } else {
                todo!();
            }

            // Ekikibo
            if let Some(ekikibo) = dir.find("Ekikibo") {
                if let Node::Property(ekikibo) = ekikibo {
                    result.ekikibo = Ekikibo::from_str(&ekikibo.value)?;
                }
            } else {
                todo!();
            }

            // Kyoukaisen
            if let Some(kyoukaisen) = dir.find("Kyoukaisen")
                && let Node::Property(kyoukaisen) = kyoukaisen
            {
                result.kyoukaisen = kyoukaisen.value == "1";
            }

            // m_bDiagramRessyajouhouHyoujiKudari
            if let Some(hyouji) = dir.find("DiagramRessyajouhouHyoujiKudari")
                && let Node::Property(hyouji) = hyouji
            {
                result.diagram_ressyajouhou_hyouji_kudari =
                    DiagramRessyajouhouHyouji::from_str(&hyouji.value)?;
            }

            // m_bDiagramRessyajouhouHyoujiNobori
            if let Some(hyouji) = dir.find("DiagramRessyajouhouHyoujiNobori")
                && let Node::Property(hyouji) = hyouji
            {
                result.diagram_ressyajouhou_hyouji_kudari =
                    DiagramRessyajouhouHyouji::from_str(&hyouji.value)?;
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Ekijikokukeisiki {
    #[default]
    Hatsu,
    Hatsuchaku,
    KudariChaku,
    NoboriChaku,
}
impl Ekijikokukeisiki {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "Jikokukeisiki_Hatsu" => Ok(Self::Hatsu),
            "Jikokukeisiki_Hatsuchaku" => Ok(Self::Hatsuchaku),
            "Jikokukeisiki_KudariChaku" => Ok(Self::KudariChaku),
            "Jikokukeisiki_NoboriChaku" => Ok(Self::NoboriChaku),
            _ => Err(Error::TodoError),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Ekikibo {
    #[default]
    Ippan,
    Syuyou,
}
impl Ekikibo {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "Ekikibo_Ippan" => Ok(Self::Ippan),
            "Ekikibo_Syuyou" => Ok(Self::Syuyou),
            _ => Err(Error::TodoError),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum DiagramRessyajouhouHyouji {
    #[default]
    Origin,
    Anytime,
    Not,
}
impl DiagramRessyajouhouHyouji {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "" => Ok(Self::Origin),
            "DiagramRessyajouhouHyouji_Anytime" => Ok(Self::Anytime),
            "DiagramRessyajouhouHyouji_Not" => Ok(Self::Not),
            _ => Err(Error::TodoError),
        }
    }
}
