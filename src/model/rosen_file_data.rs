use crate::model::{disp_prop::DispProp, rosen::Rosen};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RosenFileData {
	rosen: Rosen,
	disp_prop: DispProp,
	
}

#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
	OuDia02,
	OuDia03,
	OuDia05,
	/// `OuDia.6`
	OuDia06,
	/// `OuDia.1.01`
	OuDia101,
	/// `OuDia.1.02`
	OuDia102,
}