#[derive(Debug, Default, PartialEq, Clone)]
pub struct FontProp {
	point_text_height: usize,
	logicalunit_text_height: usize,
	logicalunit_cell_height: usize,
	facename: String,
	bold: bool,
	itaric: bool,
	underine: bool,
	strike_out: bool,	
}