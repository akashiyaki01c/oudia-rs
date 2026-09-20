use crate::model::error::Error;

const TOTAL_SECONDS_PER_DAY: usize = 24 * 60 * 60;

/// 1つの秒単位の時刻情報を表す構造体
#[derive(Debug, Default, PartialEq, Clone, Copy, Eq)]
pub struct Jikoku {
    /// 0:00:00からの経過秒
    total_seconds: Option<usize>,
}

impl Jikoku {
    pub fn adjust_total_seconds(self) -> Self {
        Self {
            total_seconds: self
                .total_seconds
                .map(|total_seconds| total_seconds % TOTAL_SECONDS_PER_DAY),
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

                let h: usize = str_h.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.hour".to_string(),
                    value: str_h.to_string(),
                })?;
                let m: usize = str_m.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.minute".to_string(),
                    value: str_m.to_string(),
                })?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60),
                })
            }
            4 => {
                // hhmm
                let str_h = &value[0..2];
                let str_m = &value[2..4];

                let h: usize = str_h.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.hour".to_string(),
                    value: str_h.to_string(),
                })?;
                let m: usize = str_m.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.minute".to_string(),
                    value: str_m.to_string(),
                })?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60),
                })
            }
            5 => {
                // hmmss
                let str_h = &value[0..1];
                let str_m = &value[1..3];
                let str_s = &value[3..5];

                let h: usize = str_h.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.hour".to_string(),
                    value: str_h.to_string(),
                })?;
                let m: usize = str_m.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.minute".to_string(),
                    value: str_m.to_string(),
                })?;
                let s: usize = str_s.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.second".to_string(),
                    value: str_s.to_string(),
                })?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60 + s),
                })
            }
            6 => {
                // hhmmss
                let str_h = &value[0..2];
                let str_m = &value[2..4];
                let str_s = &value[4..6];

                let h: usize = str_h.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.hour".to_string(),
                    value: str_h.to_string(),
                })?;
                let m: usize = str_m.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.minute".to_string(),
                    value: str_m.to_string(),
                })?;
                let s: usize = str_s.parse().map_err(|_| Error::InvalidNumber {
                    field: "Jikoku.second".to_string(),
                    value: str_s.to_string(),
                })?;

                Ok(Self {
                    total_seconds: Some(h * 60 * 60 + m * 60 + s),
                })
            }
            _ => Err(Error::InvalidFormat {
                context: "Jikoku".to_string(),
                value: value.to_string(),
            }),
        }
    }

    pub fn to_oudia_string(self) -> String {
        self.total_seconds
            .map(|total_seconds| {
                let hours = total_seconds / 3600;
                let minutes = total_seconds / 60 % 60;
                let seconds = total_seconds % 60;
                if seconds == 0 {
                    if hours < 10 {
                        format!("{hours}{minutes:02}")
                    } else {
                        format!("{hours:02}{minutes:02}")
                    }
                } else {
                    if hours < 10 {
                        format!("{hours}{minutes:02}{seconds:02}")
                    } else {
                        format!("{hours:02}{minutes:02}{seconds:02}")
                    }
                }
            })
            .unwrap_or_default()
    }
}
