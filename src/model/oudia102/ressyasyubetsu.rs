use std::str::FromStr;

use crate::{
    model::oudia102::{color::ColorProp, error::Error},
    opt::{directory::Directory, node::Node, property::Property},
};

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
            if let Some(syubetsumei) = dir.find("Syubetsumei") {
                if let Node::Property(syubetsumei) = syubetsumei {
                    result.syubetsumei = syubetsumei.value.clone();
                }
            } else {
                todo!();
            }

            // Ryakusyou
            if let Some(ryakusyou) = dir.find("Ryakusyou")
                && let Node::Property(ryakusyou) = ryakusyou
            {
                result.ryakusyou = ryakusyou.value.clone();
            }

            // JikokuhyouMojiColor
            if let Some(color) = dir.find("JikokuhyouMojiColor")
                && let Node::Property(color) = color
            {
                result.jikokuhyou_moji_color = ColorProp::from_str(&color.value)?;
            }

            // JikokuhyouFontIndex
            if let Some(index) = dir.find("JikokuhyouFontIndex")
                && let Node::Property(index) = index
            {
                result.jikokuhyou_font_index = index.value.parse().map_err(|_| Error::TodoError)?;
            }

            // DiagramSenColor
            if let Some(color) = dir.find("DiagramSenColor")
                && let Node::Property(color) = color
            {
                result.diagram_sen_color = ColorProp::from_str(&color.value)?;
            }

            // DiagramSenStyle
            if let Some(style) = dir.find("DiagramSenStyle")
                && let Node::Property(style) = style
            {
                result.diagram_sen_style = SenStype::from_str(&style.value)?;
            }

            // DiagramSenIsBold
            if let Some(style) = dir.find("DiagramSenIsBold")
                && let Node::Property(style) = style
            {
                result.diagram_sen_is_bold = style.value == "1";
            }

            // StopMarkDrawType
            if let Some(style) = dir.find("StopMarkDrawType")
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
        let mut values = vec![property("Syubetsumei", &self.syubetsumei)];
        if !self.ryakusyou.is_empty() {
            values.push(property("Ryakusyou", &self.ryakusyou));
        }
        values.extend([
            property(
                "JikokuhyouMojiColor",
                self.jikokuhyou_moji_color.to_string(),
            ),
            property(
                "JikokuhyouFontIndex",
                self.jikokuhyou_font_index.to_string(),
            ),
            property("DiagramSenColor", self.diagram_sen_color.to_string()),
            property("DiagramSenStyle", self.diagram_sen_style.to_oudia_string()),
        ]);
        if self.diagram_sen_is_bold {
            values.push(property("DiagramSenIsBold", "1"));
        }
        values.push(property(
            "StopMarkDrawType",
            self.stop_mark_draw_type.to_oudia_string(),
        ));
        Node::Directory(Directory::new_with_value(
            "Ressyasyubetsu",
            values,
        ))
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
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "SenStyle_Jissen" => Ok(Self::Jissen),
            "SenStyle_Hasen" => Ok(Self::Hasen),
            "SenStyle_Tensen" => Ok(Self::Tensen),
            "SenStyle_Ittensasen" => Ok(Self::Ittensasen),
            _ => Err(Error::TodoError),
        }
    }

    fn to_oudia_string(&self) -> &'static str {
        match self {
            Self::Jissen => "SenStyle_Jissen",
            Self::Hasen => "SenStyle_Hasen",
            Self::Tensen => "SenStyle_Tensen",
            Self::Ittensasen => "SenStyle_Ittensasen",
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
    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "EStopMarkDrawType_DrawOnStop" => Ok(Self::DrawOnStop),
            "EStopMarkDrawType_Nothing" => Ok(Self::Nothing),
            "EStopMarkDrawType_DrawOnPass" => Ok(Self::DrawOnPass),
            _ => Err(Error::TodoError),
        }
    }

    fn to_oudia_string(&self) -> &'static str {
        match self {
            Self::DrawOnStop => "EStopMarkDrawType_DrawOnStop",
            Self::Nothing => "EStopMarkDrawType_Nothing",
            Self::DrawOnPass => "EStopMarkDrawType_DrawOnPass",
        }
    }
}
