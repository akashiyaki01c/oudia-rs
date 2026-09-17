use std::str::FromStr;

use crate::{
    model::oudia102::{color::ColorProp, error::Error, font::FontProp},
    opt::{directory::Directory, node::Node, property::Property},
};

/// 時刻表のフォント設定数
const JIKOKUHYOUFONT_COUNT: usize = 8;

/// ダイヤグラムファイルの表示設定を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DispProp {
    jikokuhyou_font: [FontProp; JIKOKUHYOUFONT_COUNT],
    jikokuhyou_v_font: FontProp,
    dia_ekimei_font: FontProp,
    dia_jikoku_font: FontProp,
    dia_ressya_font: FontProp,
    comment_font: FontProp,
    dia_moji_color: ColorProp,
    dia_haikei_color: ColorProp,
    dia_ressya_color: ColorProp,
    dia_jiku_color: ColorProp,
    ekimei_length: usize,
    jikokuhyou_ressya_width: usize,
    dia_ressyajouhou_hyouji_eki_order_kudari: usize,
    dia_ressyajouhou_hyouji_eki_order_nobori: usize,
}
impl DispProp {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // JikokuhyouFont
            if dir.find_all("JikokuhyouFont").is_empty() {
                return Err(Error::KeyIsNotFound("JikokukyouFont".to_string()))
            }
            for (i, font) in dir.find_all("JikokuhyouFont").iter().enumerate() {
                if let Node::Property(font) = font
                    && let Some(v) = result.jikokuhyou_font.get_mut(i)
                {
                    *v = FontProp::from_str(&font.value)?;
                }
            }

            // JikokuhyouVFont
            if let Some(Node::Property(font)) = dir.find("JikokuhyouVFont") {
                result.jikokuhyou_v_font = FontProp::from_str(&font.value)?;
            }

            // DiaEkimeiFont
            if let Some(Node::Property(font)) = dir.find("DiaEkimeiFont") {
                result.dia_ekimei_font = FontProp::from_str(&font.value)?;
            }

            // DiaJikokuFont
            if let Some(Node::Property(font)) = dir.find("DiaJikokuFont") {
                result.dia_jikoku_font = FontProp::from_str(&font.value)?;
            }

            // DiaRessyaFont
            if let Some(Node::Property(font)) = dir.find("DiaRessyaFont") {
                result.dia_ressya_font = FontProp::from_str(&font.value)?;
            }

            // CommentFont
            if let Some(Node::Property(font)) = dir.find("CommentFont") {
                result.comment_font = FontProp::from_str(&font.value)?;
            }

            // DiaMojiColor
            if let Some(Node::Property(font)) = dir.find("DiaMojiColor") {
                result.dia_moji_color = ColorProp::from_str(&font.value)?;
            }

            // DiaHaikeiColor
            if let Some(Node::Property(font)) = dir.find("DiaHaikeiColor") {
                result.dia_haikei_color = ColorProp::from_str(&font.value)?;
            }

            // DiaRessyaColor
            if let Some(Node::Property(font)) = dir.find("DiaRessyaColor") {
                result.dia_ressya_color = ColorProp::from_str(&font.value)?;
            }

            // DiaJikuColor
            if let Some(Node::Property(font)) = dir.find("DiaJikuColor") {
                result.dia_jiku_color = ColorProp::from_str(&font.value)?;
            }

            // EkimeiLength
            if let Some(Node::Property(font)) = dir.find("EkimeiLength") {
                result.ekimei_length = font.value.parse().map_err(|_| Error::TodoError)?;
            }

            // JikokuhyouRessyaWidth
            if let Some(Node::Property(font)) = dir.find("JikokuhyouRessyaWidth") {
                result.jikokuhyou_ressya_width =
                    font.value.parse().map_err(|_| Error::TodoError)?;
            }

            // DiaRessyajouhouHyoujiEkiOrderKudari
            if let Some(Node::Property(order)) = dir.find("DiaRessyajouhouHyoujiEkiOrderKudari") {
                result.dia_ressyajouhou_hyouji_eki_order_kudari =
                    order.value.parse().map_err(|_| Error::TodoError)?;
            }

            // DiaRessyajouhouHyoujiEkiOrderNobori
            if let Some(Node::Property(order)) = dir.find("DiaRessyajouhouHyoujiEkiOrderNobori") {
                    result.dia_ressyajouhou_hyouji_eki_order_nobori =
                    order.value.parse().map_err(|_| Error::TodoError)?;
            }
        } else {
            unreachable!()
        }

        Ok(result)
    }

    pub(crate) fn to_node(&self) -> Node {
        let mut values = self
            .jikokuhyou_font
            .iter()
            .map(|font| property("JikokuhyouFont", font.to_oudia_string()))
            .collect::<Vec<_>>();
        values.extend([
            property("JikokuhyouVFont", self.jikokuhyou_v_font.to_oudia_string()),
            property("DiaEkimeiFont", self.dia_ekimei_font.to_oudia_string()),
            property("DiaJikokuFont", self.dia_jikoku_font.to_oudia_string()),
            property("DiaRessyaFont", self.dia_ressya_font.to_oudia_string()),
            property("CommentFont", self.comment_font.to_oudia_string()),
            property("DiaMojiColor", self.dia_moji_color.to_string()),
            property("DiaHaikeiColor", self.dia_haikei_color.to_string()),
            property("DiaRessyaColor", self.dia_ressya_color.to_string()),
            property("DiaJikuColor", self.dia_jiku_color.to_string()),
            property("EkimeiLength", self.ekimei_length.to_string()),
            property(
                "JikokuhyouRessyaWidth",
                self.jikokuhyou_ressya_width.to_string(),
            ),
        ]);
        Node::Directory(Directory::new_with_value("DispProp", values))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}
