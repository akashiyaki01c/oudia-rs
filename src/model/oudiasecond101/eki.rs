use crate::{
    model::{error::Error, oudiasecond101::eki_track::EkiTrack},
    opt::{directory::Directory, node::Node, property::Property},
};

const KEY_EKI: &str = "Eki";
const KEY_EKIMEI: &str = "Ekimei";
const KEY_EKIJIKOKUKEISIKI: &str = "Ekijikokukeisiki";
const KEY_EKIKIBO: &str = "Ekikibo";
const KEY_KYOUKAISEN: &str = "Kyoukaisen";
const KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_KUDARI: &str = "DiagramRessyajouhouHyoujiKudari";
const KEY_DIAGRAM_RESSYAJOUHOU_HYOUJI_NOBORI: &str = "DiagramRessyajouhouHyoujiNobori";
const KEY_DOWN_MAIN: &str = "DownMain";
const KEY_UP_MAIN: &str = "UpMain";
const KEY_BRUNCH_CORE_EKI_INDEX: &str = "BrunchCoreEkiIndex";
const KEY_LOOP_ORIGIN_EKI_INDEX: &str = "LoopOriginEkiIndex";
const KEY_JIKOKUHYOU_TRACK_DISPLAY_KUDARI: &str = "JikokuhyouTrackDisplayKudari";
const KEY_JIKOKUHYOU_TRACK_DISPLAY_NOBORI: &str = "JikokuhyouTrackDisplayNobori";
const KEY_DIAGRAM_TRACK_DISPLAY: &str = "DiagramTrackDisplay";
const KEY_EKI_TRACK2_CONT: &str = "EkiTrack2Cont";

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
    down_main: usize,
    up_main: usize,
    brunch_core_eki_index: usize,
    loop_irigin_eki_index: usize,
    jikokuhyou_track_display_kudari: bool,
    jikokuhyou_track_display_nobori: bool,
    diagram_track_display: bool,
    eki_track2_cont: Vec<EkiTrack>,
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

            // DownMain
            if let Some(value) = dir.find(KEY_DOWN_MAIN)
                && let Node::Property(value) = value
            {
                result.down_main = value.value.parse().map_err(|_| Error::InvalidNumber {
                    field: KEY_DOWN_MAIN.to_string(),
                    value: value.value.to_string(),
                })?;
            }

            // UpMain
            if let Some(value) = dir.find(KEY_UP_MAIN)
                && let Node::Property(value) = value
            {
                result.up_main = value.value.parse().map_err(|_| Error::InvalidNumber {
                    field: KEY_UP_MAIN.to_string(),
                    value: value.value.to_string(),
                })?;
            }

            // BrunchCoreEkiIndex
            if let Some(value) = dir.find(KEY_BRUNCH_CORE_EKI_INDEX)
                && let Node::Property(value) = value
            {
                result.brunch_core_eki_index =
                    value.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_BRUNCH_CORE_EKI_INDEX.to_string(),
                        value: value.value.to_string(),
                    })?;
            }

            // LoopOriginEkiIndex
            if let Some(value) = dir.find(KEY_LOOP_ORIGIN_EKI_INDEX)
                && let Node::Property(value) = value
            {
                result.loop_irigin_eki_index =
                    value.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_LOOP_ORIGIN_EKI_INDEX.to_string(),
                        value: value.value.to_string(),
                    })?;
            }

            // JikokuhyouTrackDisplayKudari
            if let Some(value) = dir.find(KEY_JIKOKUHYOU_TRACK_DISPLAY_KUDARI)
                && let Node::Property(value) = value
            {
                result.jikokuhyou_track_display_kudari = value.value == "1";
            }

            // JikokuhyouTrackDisplayNobori
            if let Some(value) = dir.find(KEY_JIKOKUHYOU_TRACK_DISPLAY_NOBORI)
                && let Node::Property(value) = value
            {
                result.jikokuhyou_track_display_nobori = value.value == "1";
            }

            // DiagramTrackDisplay
            if let Some(value) = dir.find(KEY_DIAGRAM_TRACK_DISPLAY)
                && let Node::Property(value) = value
            {
                result.diagram_track_display = value.value == "1";
            }

            // EkiTrack2Cont
            if let Some(cont) = dir.find(KEY_EKI_TRACK2_CONT)
                && let Node::Directory(cont) = cont
            {
                let mut arr = vec![];
                for track in cont.values.iter() {
                    arr.push(EkiTrack::from_node(track)?);
                }
                result.eki_track2_cont = arr;
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
        values.push(property(KEY_DOWN_MAIN, self.down_main.to_string()));
        values.push(property(KEY_UP_MAIN, self.up_main.to_string()));
        values.push(property(
            KEY_BRUNCH_CORE_EKI_INDEX,
            self.brunch_core_eki_index.to_string(),
        ));
        values.push(property(
            KEY_LOOP_ORIGIN_EKI_INDEX,
            self.loop_irigin_eki_index.to_string(),
        ));
        if self.jikokuhyou_track_display_kudari {
            values.push(property(
                KEY_JIKOKUHYOU_TRACK_DISPLAY_KUDARI,
                self.jikokuhyou_track_display_kudari.to_string(),
            ));
        }
        if self.jikokuhyou_track_display_nobori {
            values.push(property(
                KEY_JIKOKUHYOU_TRACK_DISPLAY_NOBORI,
                self.jikokuhyou_track_display_nobori.to_string(),
            ));
        }
        if self.diagram_track_display {
            values.push(property(
                KEY_DIAGRAM_TRACK_DISPLAY,
                self.diagram_track_display.to_string(),
            ));
        }
        for track in self.eki_track2_cont.iter() {
            values.push(track.to_node());
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
    /// 下りは発着時刻、上りは発時刻のみ
    OutboundDepartureAndArrival,
    /// 下りは発時刻のみ、上りは発着時刻
    InboundDepartureAndArrival,
}
impl Ekijikokukeisiki {
    const KEY_HATSU: &str = "Jikokukeisiki_Hatsu";
    const KEY_HATSUCHAKU: &str = "Jikokukeisiki_Hatsuchaku";
    const KEY_KUDARI_CHAKU: &str = "Jikokukeisiki_KudariChaku";
    const KEY_NOBORI_CHAKU: &str = "Jikokukeisiki_NoboriChaku";
    const KEY_KUDARI_HATSUCHAKU: &str = "Jikokukeisiki_KudariHatsuchaku";
    const KEY_NOBORI_HATSUCHAKU: &str = "Jikokukeisiki_NoboriHatsuchaku";

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
            Self::KEY_KUDARI_HATSUCHAKU => Ok(Self::OutboundDepartureAndArrival),
            Self::KEY_NOBORI_HATSUCHAKU => Ok(Self::InboundDepartureAndArrival),
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
            Self::OutboundDepartureAndArrival => Self::KEY_KUDARI_HATSUCHAKU,
            Self::InboundDepartureAndArrival => Self::KEY_NOBORI_HATSUCHAKU,
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
