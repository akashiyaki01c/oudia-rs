use crate::{
    model::error::Error,
    opt::{directory::Directory, node::Node, property::Property},
};

const KEY_EKI: &str = "Eki";
const KEY_EKIMEI: &str = "Ekimei";
const KEY_EKIJIKOKUKEISIKI: &str = "Ekijikokukeisiki";
const KEY_EKIKIBO: &str = "Ekikibo";
const KEY_KYOUKAISEN: &str = "Kyoukaisen";
const KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_KUDARI: &str = "DiagramRessyajouhouHyoujiKudari";
const KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_NOBORI: &str = "DiagramRessyajouhouHyoujiNobori";

/// 一つの駅を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Station {
    /// 駅名
    name: String,
    /// 駅時刻形式
    timetable_display_format: Ekijikokukeisiki,
    /// 駅規模
    sta_scale: StationScale,
    /// 境界線を引くか
    kyoukaisen: bool,
    diagram_ressyajouhou_hyouji_kudari: DiagramRessyajouhouHyouji,
    diagram_ressyajouhou_hyouji_nobori: DiagramRessyajouhouHyouji,
}
impl Station {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // Ekimei
            if let Some(ekimei) = dir.find(KEY_EKIMEI)
                && let Node::Property(ekimei) = ekimei {
                    result.name = ekimei.value.clone();
                }

            // Ekijikokukeisiki
            if let Some(ekimei) = dir.find(KEY_EKIJIKOKUKEISIKI)
                && let Node::Property(ekimei) = ekimei {
                    result.timetable_display_format = Ekijikokukeisiki::from_str(&ekimei.value)?;
                }

            // Ekikibo
            if let Some(ekikibo) = dir.find(KEY_EKIKIBO)
                && let Node::Property(ekikibo) = ekikibo {
                    result.sta_scale = StationScale::from_str(&ekikibo.value)?;
                }

            // Kyoukaisen
            if let Some(kyoukaisen) = dir.find(KEY_KYOUKAISEN)
                && let Node::Property(kyoukaisen) = kyoukaisen
            {
                result.kyoukaisen = kyoukaisen.value == "1";
            }

            // DiagramRessyajouhouHyoujiKudari
            if let Some(hyouji) = dir.find(KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_KUDARI)
                && let Node::Property(hyouji) = hyouji
            {
                result.diagram_ressyajouhou_hyouji_kudari =
                    DiagramRessyajouhouHyouji::from_str(&hyouji.value)?;
            }

            // DiagramRessyajouhouHyoujiNobori
            if let Some(hyouji) = dir.find(KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_NOBORI)
                && let Node::Property(hyouji) = hyouji
            {
                result.diagram_ressyajouhou_hyouji_nobori =
                    DiagramRessyajouhouHyouji::from_str(&hyouji.value)?;
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        let mut values = vec![
            property(KEY_EKIMEI, &self.name),
            property(
                KEY_EKIJIKOKUKEISIKI,
                self.timetable_display_format.to_oudia_string(),
            ),
            property(KEY_EKIKIBO, self.sta_scale.to_oudia_string()),
        ];
        if self.kyoukaisen {
            values.push(property(KEY_KYOUKAISEN, "1"));
        }
        if !matches!(
            self.diagram_ressyajouhou_hyouji_kudari,
            DiagramRessyajouhouHyouji::Origin
        ) {
            values.push(property(
                KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_KUDARI,
                self.diagram_ressyajouhou_hyouji_kudari.to_oudia_string(),
            ));
        }
        if !matches!(
            self.diagram_ressyajouhou_hyouji_nobori,
            DiagramRessyajouhouHyouji::Origin
        ) {
            values.push(property(
                KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_NOBORI,
                self.diagram_ressyajouhou_hyouji_nobori.to_oudia_string(),
            ));
        }
        Node::Directory(Directory::new_with_value(KEY_EKI, values))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}

/// 駅表示形式を表す
#[derive(Debug, Default, PartialEq, Clone)]
pub enum Ekijikokukeisiki {
    /// 発車時刻のみ
    #[default]
    DepartureOnly,
    /// 発車時刻・到着時刻
    DepartureAndArrival,
    /// 下りは着時刻のみ、上りは発時刻のみ
    OutboundArrival,
    /// 下りは発時刻のみ、上りは着時刻のみ
    InboundArrival,
}
impl Ekijikokukeisiki {
    const KEY_HATSU: &str = "Jikokukeisiki_Hatsu";
    const KEY_HATSUCHAKU: &str = "Jikokukeisiki_Hatsuchaku";
    const KEY_KUDARI_CHAKU: &str = "Jikokukeisiki_KudariChaku";
    const KEY_NOBORI_CHAKU: &str = "Jikokukeisiki_NoboriChaku";

    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "" => Err(Error::InvalidEnum {
                field: "Jikokukeisiki".to_string(),
                value: value.to_string(),
            }),
            Self::KEY_HATSU => Ok(Self::DepartureOnly),
            Self::KEY_HATSUCHAKU => Ok(Self::DepartureAndArrival),
            Self::KEY_KUDARI_CHAKU => Ok(Self::OutboundArrival),
            Self::KEY_NOBORI_CHAKU => Ok(Self::InboundArrival),
            _ => Err(Error::InvalidEnum {
                field: "Jikokukeisiki".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn to_oudia_string(&self) -> &'static str {
        match self {
            Self::DepartureOnly => Self::KEY_HATSU,
            Self::DepartureAndArrival => Self::KEY_HATSUCHAKU,
            Self::OutboundArrival => Self::KEY_KUDARI_CHAKU,
            Self::InboundArrival => Self::KEY_NOBORI_CHAKU,
        }
    }
}

/// 駅規模
#[derive(Debug, Default, PartialEq, Clone)]
pub enum StationScale {
    /// 一般駅
    #[default]
    Normal,
    /// 主要駅
    Terminal,
}
impl StationScale {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "" => Err(Error::InvalidEnum {
                field: "Ekikibo".to_string(),
                value: value.to_string(),
            }),
            "Ekikibo_Ippan" => Ok(Self::Normal),
            "Ekikibo_Syuyou" => Ok(Self::Terminal),
            _ => Err(Error::InvalidEnum {
                field: "Ekikibo".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn to_oudia_string(&self) -> &'static str {
        match self {
            Self::Normal => "Ekikibo_Ippan",
            Self::Terminal => "Ekikibo_Syuyou",
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
            _ => Err(Error::InvalidEnum {
                field: "DiagramRessyajouhouHyouji".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn to_oudia_string(&self) -> &'static str {
        match self {
            Self::Origin => "",
            Self::Anytime => "DiagramRessyajouhouHyouji_Anytime",
            Self::Not => "DiagramRessyajouhouHyouji_Not",
        }
    }
}
