#[derive(Debug, Default, PartialEq, Clone)]
pub struct Eki {
	ekimei: String,
	ekijikokukeisiki: Ekijikokukeisiki,
	ekikibo: Ekikibo,
	kyoukaisen: bool,
	diagram_ressyajouhou_hyouji_kudari: DiagramRessyajouhouHyouji,
	diagram_ressyajouhou_hyouji_nobori: DiagramRessyajouhouHyouji,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Ekijikokukeisiki {
	#[default]
	Hatsu,
	Hatsuchaku,
	KudariChaku,
	NoboriChaku,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Ekikibo {
	#[default]
	Ippan,
	Syuyou,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum DiagramRessyajouhouHyouji {
	#[default]
	Origin,
	Anytime,
	Not,
}
