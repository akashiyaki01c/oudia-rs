use crate::model::ekijikoku::Ekijikoku;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ressya {
	ressyasyubetsu_index: usize,
	ressyabangou: String,
	ressyamei: String,
	gousuu: String,
	ekijikoku: Vec<Ekijikoku>,
	bikou: String,
}
