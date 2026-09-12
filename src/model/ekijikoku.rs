use crate::model::jikoku::Jikoku;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ekijikoku {
	ekiatsukai: usize,
	chaku: Jikoku,
	hatsu: Jikoku,
}