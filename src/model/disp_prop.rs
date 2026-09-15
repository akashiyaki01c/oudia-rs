use crate::{
    model::{color::ColorProp, error::Error, font::FontProp},
    opt::node::Node,
};

const JIKOKUHYOUFONT_COUNT: usize = 8;

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
            for (i, font) in dir.find_all("DiaName").iter().enumerate() {
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
        } else {
            unreachable!()
        }

        Ok(result)
    }
}
