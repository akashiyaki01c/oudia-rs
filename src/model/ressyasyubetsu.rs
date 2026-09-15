use crate::{
    model::{color::ColorProp, error::Error},
    opt::node::Node,
};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ressyasyubetsu {
    syubetsumei: String,
    ryakusyou: String,
    jikokuhyou_moji_color: ColorProp,
    jikokuhyou_font_index: usize,
    diagram_sen_color: ColorProp,
    diagram_sen_style: SenStype,
    diagram_sen_is_bold: bool,
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
                && let Node::Property(ryakusyou) = ryakusyou {
                    result.ryakusyou = ryakusyou.value.clone();
                }

            // JikokuhyouMojiColor
            if let Some(color) = dir.find("JikokuhyouMojiColor")
                && let Node::Property(color) = color {
                    result.jikokuhyou_moji_color = ColorProp::from_str(&color.value)?;
                }

            // JikokuhyouFontIndex
            if let Some(index) = dir.find("JikokuhyouFontIndex")
                && let Node::Property(index) = index {
                    result.jikokuhyou_font_index =
                        index.value.parse().map_err(|_| Error::TodoError)?;
                }

            // DiagramSenColor
            if let Some(color) = dir.find("DiagramSenColor")
                && let Node::Property(color) = color {
                    result.diagram_sen_color = ColorProp::from_str(&color.value)?;
                }

            // DiagramSenStyle
            if let Some(style) = dir.find("DiagramSenStyle")
                && let Node::Property(style) = style {
                    result.diagram_sen_style = SenStype::from_str(&style.value)?;
                }

            // DiagramSenIsBold
            if let Some(style) = dir.find("DiagramSenIsBold")
                && let Node::Property(style) = style {
                    result.diagram_sen_is_bold = style.value == "1";
                }

            // StopMarkDrawType
            if let Some(style) = dir.find("StopMarkDrawType")
                && let Node::Property(style) = style {
                    result.stop_mark_draw_type = StopMarkDrawType::from_str(&style.value)?;
                }
        } else {
            unreachable!()
        }

        Ok(result)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum SenStype {
    #[default]
    Jissen,
    Hasen,
    Tensen,
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
}

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
}
