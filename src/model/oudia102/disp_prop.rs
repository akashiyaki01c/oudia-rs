use std::str::FromStr;

use crate::{
    model::{
        error::Error,
        oudia102::{color::ColorProp, font::FontProp},
    },
    opt::{directory::Directory, node::Node, property::Property},
};

const KEY_DISP_PROP: &str = "DispProp";
const KEY_JIKOKUHYOU_FONT: &str = "JikokuhyouFont";
const KEY_JIKOKUHYOU_V_FONT: &str = "JikokuhyouVFont";
const KEY_DIA_EKIMEI_FONT: &str = "DiaEkimeiFont";
const KEY_DIA_JIKOKU_FONT: &str = "DiaJikokuFont";
const KEY_DIA_RESSYA_FONT: &str = "DiaRessyaFont";
const KEY_COMMENT_FONT: &str = "CommentFont";
const KEY_DIA_MOJI_COLOR: &str = "DiaMojiColor";
const KEY_DIA_HAIKEI_COLOR: &str = "DiaHaikeiColor";
const KEY_DIA_RESSYA_COLOR: &str = "DiaRessyaColor";
const KEY_DIA_JIKU_COLOR: &str = "DiaJikuColor";
const KEY_EKIMEI_LENGTH: &str = "EkimeiLength";
const KEY_JIKOKUHYOU_RESSYA_WIDTH: &str = "JikokuhyouRessyaWidth";
const KEY_DIA_RESSYAJOUHOU_HYOUJI_EKI_ORDER_KUDARI: &str = "DiaRessyajouhouHyoujiEkiOrderKudari";
const KEY_DIA_RESSYAJOUHOU_HYOUJI_EKI_ORDER_NOBORI: &str = "DiaRessyajouhouHyoujiEkiOrderNobori";

/// 時刻表のフォント設定数
const JIKOKUHYOUFONT_COUNT: usize = 8;

/// ダイヤグラムファイルの表示設定を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DisplayProperties {
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
impl DisplayProperties {
    pub fn from_node(node: &Node) -> Result<Self, Error> {
        let mut result = Self::default();

        if let Node::Property(_) = node {
            return Err(Error::NodeTypeError);
        } else if let Node::Directory(dir) = node {
            // JikokuhyouFont
            if dir.find_all(KEY_JIKOKUHYOU_FONT).is_empty() {
                return Err(Error::KeyIsNotFound(KEY_JIKOKUHYOU_FONT.to_string()));
            }
            for (i, font) in dir.find_all(KEY_JIKOKUHYOU_FONT).iter().enumerate() {
                if let Node::Property(font) = font
                    && let Some(v) = result.jikokuhyou_font.get_mut(i)
                {
                    *v = FontProp::from_str(&font.value)?;
                }
            }

            // JikokuhyouVFont
            if let Some(Node::Property(font)) = dir.find(KEY_JIKOKUHYOU_V_FONT) {
                result.jikokuhyou_v_font = FontProp::from_str(&font.value)?;
            }

            // DiaEkimeiFont
            if let Some(Node::Property(font)) = dir.find(KEY_DIA_EKIMEI_FONT) {
                result.dia_ekimei_font = FontProp::from_str(&font.value)?;
            } else {
                return Err(Error::KeyIsNotFound(KEY_DIA_EKIMEI_FONT.to_string()));
            }

            // DiaJikokuFont
            if let Some(Node::Property(font)) = dir.find(KEY_DIA_JIKOKU_FONT) {
                result.dia_jikoku_font = FontProp::from_str(&font.value)?;
            } else {
                return Err(Error::KeyIsNotFound(KEY_DIA_JIKOKU_FONT.to_string()));
            }

            // DiaRessyaFont
            if let Some(Node::Property(font)) = dir.find(KEY_DIA_RESSYA_FONT) {
                result.dia_ressya_font = FontProp::from_str(&font.value)?;
            }

            // CommentFont
            if let Some(Node::Property(font)) = dir.find(KEY_COMMENT_FONT) {
                result.comment_font = FontProp::from_str(&font.value)?;
            } else {
                return Err(Error::KeyIsNotFound(KEY_COMMENT_FONT.to_string()));
            }

            // DiaMojiColor
            if let Some(Node::Property(font)) = dir.find(KEY_DIA_MOJI_COLOR) {
                result.dia_moji_color = ColorProp::from_str(&font.value)?;
            }

            // DiaHaikeiColor
            if let Some(Node::Property(font)) = dir.find(KEY_DIA_HAIKEI_COLOR) {
                result.dia_haikei_color = ColorProp::from_str(&font.value)?;
            }

            // DiaRessyaColor
            if let Some(Node::Property(font)) = dir.find(KEY_DIA_RESSYA_COLOR) {
                result.dia_ressya_color = ColorProp::from_str(&font.value)?;
            }

            // DiaJikuColor
            if let Some(Node::Property(font)) = dir.find(KEY_DIA_JIKU_COLOR) {
                result.dia_jiku_color = ColorProp::from_str(&font.value)?;
            }

            // EkimeiLength
            if let Some(Node::Property(font)) = dir.find(KEY_EKIMEI_LENGTH) {
                result.ekimei_length = font.value.parse().map_err(|_| Error::InvalidNumber {
                    field: KEY_EKIMEI_LENGTH.to_string(),
                    value: font.value.to_string(),
                })?;
            }

            // JikokuhyouRessyaWidth
            if let Some(Node::Property(font)) = dir.find(KEY_JIKOKUHYOU_RESSYA_WIDTH) {
                result.jikokuhyou_ressya_width =
                    font.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_JIKOKUHYOU_RESSYA_WIDTH.to_string(),
                        value: font.value.to_string(),
                    })?;
            }

            // DiaRessyajouhouHyoujiEkiOrderKudari
            if let Some(Node::Property(order)) =
                dir.find(KEY_DIA_RESSYAJOUHOU_HYOUJI_EKI_ORDER_KUDARI)
            {
                result.dia_ressyajouhou_hyouji_eki_order_kudari =
                    order.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_DIA_RESSYAJOUHOU_HYOUJI_EKI_ORDER_KUDARI.to_string(),
                        value: order.value.to_string(),
                    })?;
            }

            // DiaRessyajouhouHyoujiEkiOrderNobori
            if let Some(Node::Property(order)) =
                dir.find(KEY_DIA_RESSYAJOUHOU_HYOUJI_EKI_ORDER_NOBORI)
            {
                result.dia_ressyajouhou_hyouji_eki_order_nobori =
                    order.value.parse().map_err(|_| Error::InvalidNumber {
                        field: KEY_DIA_RESSYAJOUHOU_HYOUJI_EKI_ORDER_NOBORI.to_string(),
                        value: order.value.to_string(),
                    })?;
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
            .map(|font| property(KEY_JIKOKUHYOU_FONT, font.to_oudia_string()))
            .collect::<Vec<_>>();
        values.extend([
            property(
                KEY_JIKOKUHYOU_V_FONT,
                self.jikokuhyou_v_font.to_oudia_string(),
            ),
            property(KEY_DIA_EKIMEI_FONT, self.dia_ekimei_font.to_oudia_string()),
            property(KEY_DIA_JIKOKU_FONT, self.dia_jikoku_font.to_oudia_string()),
            property(KEY_DIA_RESSYA_FONT, self.dia_ressya_font.to_oudia_string()),
            property(KEY_COMMENT_FONT, self.comment_font.to_oudia_string()),
            property(KEY_DIA_MOJI_COLOR, self.dia_moji_color.to_string()),
            property(KEY_DIA_HAIKEI_COLOR, self.dia_haikei_color.to_string()),
            property(KEY_DIA_RESSYA_COLOR, self.dia_ressya_color.to_string()),
            property(KEY_DIA_JIKU_COLOR, self.dia_jiku_color.to_string()),
            property(KEY_EKIMEI_LENGTH, self.ekimei_length.to_string()),
            property(
                KEY_JIKOKUHYOU_RESSYA_WIDTH,
                self.jikokuhyou_ressya_width.to_string(),
            ),
        ]);
        Node::Directory(Directory::new_with_value(KEY_DISP_PROP, values))
    }
}

fn property(name: &str, value: impl Into<String>) -> Node {
    Node::Property(Property::new_with_value(name, value.into()))
}
