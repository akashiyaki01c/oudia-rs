use crate::model::{dia::Dia, eki::Eki, jikoku::Jikoku, ressyasyubetsu::Ressyasyubetsu};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rosen {
    rosenmei: String,
    eki: Vec<Eki>,
    ressyasyubetsu: Vec<Ressyasyubetsu>,
    dia: Vec<Dia>,
    kiten_jikoku: Jikoku,
    diagram_dgr_y_zahyou_kyori_default: usize,
    comment: String,
}
