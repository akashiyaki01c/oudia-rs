use std::str::FromStr;

use crate::{
    model::{error::Error, oudia102::color::ColorProp},
    opt::{directory::Directory, node::Node, property::Property},
};

const KEY_RESSYASYUBETSU: &str = "Ressyasyubetsu";
const KEY_SYUBETSUMEI: &str = "Syubetsumei";
const KEY_RYAKUSYOU: &str = "Ryakusyou";
const KEY_JIKOKUHYOU_MOJI_COLOR: &str = "JikokuhyouMojiColor";
const KEY_JIKOKUHYOU_FONT_INDEX: &str = "JikokuhyouFontIndex";
const KEY_DIAGRAM_SEN_COLOR: &str = "DiagramSenColor";
const KEY_DIAGRAM_SEN_STYLE: &str = "DiagramSenStyle";
const KEY_DIAGRAM_SEN_IS_BOLD: &str = "DiagramSenIsBold";
const KEY_STOP_MARK_DRAW_TYPE: &str = "StopMarkDrawType";

/// 1つの列車種別を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ressyasyubetsu {
    /// 列車種別名
    syubetsumei: String,
    /// 列車種別名の略称
    ryakusyou: String,
    /// 時刻表における文字色
    jikokuhyou_moji_color: ColorProp,
    /// 時刻表におけるフォントの設定
    jikokuhyou_font_index: usize,
    /// ダイヤグラム上の列車線色
    diagram_sen_color: ColorProp,
    /// ダイヤグラム上の列車線種
    diagram_sen_style: SenStype,
    /// ダイヤグラム上の線が太いか
    diagram_sen_is_bold: bool,
    /// ダイヤグラム上の停車マーク種別
    stop_mark_draw_type: StopMarkDrawType,
}
impl Ressyasyubetsu {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // Syubetsumei
            if let Some(syubetsumei) = dir.find(KEY_SYUBETSUMEI) {
                if let Node::Property(syubetsumei) = syubetsumei {
                    if syubetsumei.value.is_empty() {
                        return Err(Error::InvalidValue(
                            syubetsumei.name.to_string(),
                            syubetsumei.value.to_string(),
                        ));
                    }
                    result.syubetsumei = syubetsumei.value.clone();
                } else {
                    return Err(Error::NodeTypeError);
                }
            } else {
                return Err(Error::KeyIsNotFound(KEY_SYUBETSUMEI.to_string()));
            }

            // Ryakusyou
            if let Some(ryakusyou) = dir.find(KEY_RYAKUSYOU)
                && let Node::Property(ryakusyou) = ryakusyou
            {
                result.ryakusyou = ryakusyou.value.clone();
            }

            // JikokuhyouMojiColor
            if let Some(color) = dir.find(KEY_JIKOKUHYOU_MOJI_COLOR)
                && let Node::Property(color) = color
            {
                result.jikokuhyou_moji_color = ColorProp::from_str(&color.value)?;
            }

            // JikokuhyouFontIndex
            if let Some(index) = dir.find(KEY_JIKOKUHYOU_FONT_INDEX)
                && let Node::Property(index) = index
            {
                result.jikokuhyou_font_index =
                    index.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_JIKOKUHYOU_FONT_INDEX.to_string(),
                        value: index.value.to_string(),
                    })?;
            }

            // DiagramSenColor
            if let Some(color) = dir.find(KEY_DIAGRAM_SEN_COLOR)
                && let Node::Property(color) = color
            {
                result.diagram_sen_color = ColorProp::from_str(&color.value)?;
            }

            // DiagramSenStyle
            if let Some(style) = dir.find(KEY_DIAGRAM_SEN_STYLE)
                && let Node::Property(style) = style
            {
                result.diagram_sen_style = SenStype::from_str(&style.value)?;
            }

            // DiagramSenIsBold
            if let Some(style) = dir.find(KEY_DIAGRAM_SEN_IS_BOLD)
                && let Node::Property(style) = style
            {
                result.diagram_sen_is_bold = style.value == "1";
            }

            // StopMarkDrawType
            if let Some(style) = dir.find(KEY_STOP_MARK_DRAW_TYPE)
                && let Node::Property(style) = style
            {
                result.stop_mark_draw_type = StopMarkDrawType::from_str(&style.value)?;
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        let mut values = vec![property(KEY_SYUBETSUMEI, &self.syubetsumei)];
        if !self.ryakusyou.is_empty() {
            values.push(property(KEY_RYAKUSYOU, &self.ryakusyou));
        }
        values.extend([
            property(
                KEY_JIKOKUHYOU_MOJI_COLOR,
                self.jikokuhyou_moji_color.to_string(),
            ),
            property(
                KEY_JIKOKUHYOU_FONT_INDEX,
                self.jikokuhyou_font_index.to_string(),
            ),
            property(KEY_DIAGRAM_SEN_COLOR, self.diagram_sen_color.to_string()),
            property(
                KEY_DIAGRAM_SEN_STYLE,
                self.diagram_sen_style.to_oudia_string(),
            ),
        ]);
        if self.diagram_sen_is_bold {
            values.push(property(KEY_DIAGRAM_SEN_IS_BOLD, "1"));
        }
        values.push(property(
            KEY_STOP_MARK_DRAW_TYPE,
            self.stop_mark_draw_type.to_oudia_string(),
        ));
        Node::Directory(Directory::new_with_value(KEY_RESSYASYUBETSU, values))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}

/// ダイヤグラム上での列車線種
#[derive(Debug, Default, PartialEq, Clone)]
pub enum SenStype {
    /// 実線
    #[default]
    Jissen,
    /// 破線
    Hasen,
    /// 点線
    Tensen,
    /// 一点鎖線
    Ittensasen,
}
impl SenStype {
    const KEY_JISSEN: &str = "SenStyle_Jissen";
    const KEY_HASEN: &str = "SenStyle_Hasen";
    const KEY_TENSEN: &str = "SenStyle_Tensen";
    const KEY_ITTENSASEN: &str = "SenStyle_Ittensasen";

    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "" => Err(Error::InvalidEnum {
                field: "SenStyle".to_string(),
                value: value.to_string(),
            }),
            Self::KEY_JISSEN => Ok(Self::Jissen),
            Self::KEY_HASEN => Ok(Self::Hasen),
            Self::KEY_TENSEN => Ok(Self::Tensen),
            Self::KEY_ITTENSASEN => Ok(Self::Ittensasen),
            _ => Err(Error::InvalidEnum {
                field: "SenStyle".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn to_oudia_string(&self) -> &'static str {
        match self {
            Self::Jissen => Self::KEY_JISSEN,
            Self::Hasen => Self::KEY_HASEN,
            Self::Tensen => Self::KEY_TENSEN,
            Self::Ittensasen => Self::KEY_ITTENSASEN,
        }
    }
}

/// ダイヤグラム上の停車マーク種別
#[derive(Debug, Default, PartialEq, Clone)]
pub enum StopMarkDrawType {
    DrawOnStop,
    #[default]
    Nothing,
    DrawOnPass,
}
impl StopMarkDrawType {
    const KEY_DRAW_ON_STOP: &str = "EStopMarkDrawType_DrawOnStop";
    const KEY_NOTHING: &str = "EStopMarkDrawType_Nothing";
    const KEY_DRAW_ON_PASS: &str = "EStopMarkDrawType_DrawOnPass";

    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "" => Err(Error::InvalidEnum {
                field: "EStopMarkDrawType".to_string(),
                value: value.to_string(),
            }),
            Self::KEY_DRAW_ON_STOP => Ok(Self::DrawOnStop),
            Self::KEY_NOTHING => Ok(Self::Nothing),
            Self::KEY_DRAW_ON_PASS => Ok(Self::DrawOnPass),
            _ => Err(Error::InvalidEnum {
                field: "EStopMarkDrawType".to_string(),
                value: value.to_string(),
            }),
        }
    }

    fn to_oudia_string(&self) -> &'static str {
        match self {
            Self::DrawOnStop => Self::KEY_DRAW_ON_STOP,
            Self::Nothing => Self::KEY_NOTHING,
            Self::DrawOnPass => Self::KEY_DRAW_ON_PASS,
        }
    }
}
