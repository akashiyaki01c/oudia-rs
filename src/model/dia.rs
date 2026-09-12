use crate::model::ressya::Ressya;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Dia {
	dia_name: String,
	kudari: Vec<Ressya>,
	nobori: Vec<Ressya>,
}