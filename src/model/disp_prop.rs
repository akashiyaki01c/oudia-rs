use crate::model::{color::ColorProp, font::FontProp};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DispProp {
	jikokuhyou_font: FontProp,
	dia_ekimei_font: FontProp,
	dia_jikoku_font: FontProp,
	dia_ressya_font: FontProp,
	comment_font: FontProp,
	dia_moji_color: ColorProp,
	dia_haikei_color: ColorProp,
	dia_ressya_color: ColorProp,
	dia_jiku_color: ColorProp,
	ekimei_length: usize,
	dia_ressyajouhou_hyouji_eki_order_kudari: usize,
	dia_ressyajouhou_hyouji_eki_order_nobori: usize,
}