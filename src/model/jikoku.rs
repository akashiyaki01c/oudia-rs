use crate::model::error::Error;

const TOTAL_SECONDS_PER_DAY: usize = 24 * 60 * 60;

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq)]
pub struct Jikoku {
    total_seconds: Option<usize>,
}

impl Jikoku {
    pub fn adjust_total_seconds(self) -> Self {
        Self {
            total_seconds: self.total_seconds.map(|total_seconds| total_seconds % TOTAL_SECONDS_PER_DAY),
        }
    }

    pub fn from_str(value: &str) -> Result<Self, Error> {
        match value.len() {
            0 => Ok(Self {
                total_seconds: None,
            }),
            3 => {
                // hmm
                let str_h = &value[0..1];
                let str_m = &value[1..3];

                let h: usize = str_h.parse().map_err(|_| Error::TodoError)?;
                let m: usize = str_m.parse().map_err(|_| Error::TodoError)?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60),
                })
            }
            4 => {
                // hhmm
                let str_h = &value[0..2];
                let str_m = &value[2..4];

                let h: usize = str_h.parse().map_err(|_| Error::TodoError)?;
                let m: usize = str_m.parse().map_err(|_| Error::TodoError)?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60),
                })
            }
            5 => {
                // hmmss
                let str_h = &value[0..1];
                let str_m = &value[1..3];
                let str_s = &value[3..5];

                let h: usize = str_h.parse().map_err(|_| Error::TodoError)?;
                let m: usize = str_m.parse().map_err(|_| Error::TodoError)?;
                let s: usize = str_s.parse().map_err(|_| Error::TodoError)?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60 + s),
                })
            }
            6 => {
                // hhmmss
                let str_h = &value[0..2];
                let str_m = &value[2..4];
                let str_s = &value[4..6];

                let h: usize = str_h.parse().map_err(|_| Error::TodoError)?;
                let m: usize = str_m.parse().map_err(|_| Error::TodoError)?;
                let s: usize = str_s.parse().map_err(|_| Error::TodoError)?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60 + s),
                })
            }
            _ => Err(Error::TodoError),
        }
    }
}
