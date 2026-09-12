use crate::model::color::ColorProp;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ressyasyubetsu {
	syubetsumei: String,
	ryakusyou: String,
	jikokuhyou_moji_color: ColorProp,
	jikokuhyou_font_index: usize,
	diagram_line_style: DiagramLineStyle,
	stop_mark_draw_type: StopMarkDrawType,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DiagramLineStyle {
	diagram_sen_color: ColorProp,
	diagram_sen_style: SenStype,
	diagram_sen_is_bold: bool,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum SenStype {
	#[default]
	Jissen,
	Hasen,
	Tensen,
	Ittensasen,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum StopMarkDrawType {
	DrawOnStop,
	#[default]
	Nothing,
	DrawOnPass,
}

