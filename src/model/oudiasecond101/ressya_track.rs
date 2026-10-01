use std::str::FromStr;

use crate::model::{error::Error, oudiasecond101::Jikoku};

/// 列車の発着番線を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RessyaTrack {
    /// 発着する番線のインデックス
    track_index: Option<usize>,
    /// 駅での作業を表す
    sagyou: Sagyou,
}
impl FromStr for RessyaTrack {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Ok(Self::default());
        }

        if !value.contains(";") {
            // 発着番線のみ
            // {unsigned integer}
            return Ok(RessyaTrack {
                track_index: Some(value.parse().map_err(|_| Error::InvalidNumber {
                    field: "RessyaTrack".to_string(),
                    value: value.to_string(),
                })?),
                sagyou: Sagyou::None,
            });
        }
        // {unsigned integer};{unsigned integer}
        let semicolon_index = value.find(";").unwrap();
        let track = &value[..semicolon_index];
        let other = &value[semicolon_index + 1..];

        if !other.contains("/") {
            // 発着番線、始終着駅の作業情報のみ
            // {unsigned integer};{unsigned integer}
            let kind = other;
            let sagyou = match kind {
                "0" => Sagyou::None,
                "1" => {
                    return Err(Error::InvalidFormat {
                        context: "".to_string(),
                        value: value.to_string(),
                    });
                }
                "2" => Sagyou::Nyusyukku(NyusyukkuSagyo {
                    unyo_number: "".to_string(),
                }),
                _ => {
                    return Err(Error::InvalidFormat {
                        context: "".to_string(),
                        value: value.to_string(),
                    });
                }
            };
            return Ok(RessyaTrack {
                track_index: Some(track.parse().map_err(|_| Error::InvalidNumber {
                    field: "RessyaTrack".to_string(),
                    value: track.to_string(),
                })?),
                sagyou,
            });
        }
        // {unsigned integer};{unsigned integer}/{string}
        let slash_index = other.find("/").unwrap();
        let kind = &other[..slash_index];
        let argument = &other[slash_index + 1..];
        let (argument, departure_time, arrival_time) = match argument.split_once('$') {
            Some((argument, times)) => {
                let (departure_time, arrival_time) = match times.split_once('/') {
                    Some((departure_time, arrival_time)) => {
                        (departure_time.parse()?, arrival_time.parse()?)
                    }
                    None => (times.parse()?, Jikoku::default()),
                };
                (argument, departure_time, arrival_time)
            }
            None => (argument, Jikoku::default(), Jikoku::default()),
        };
        let sagyou = match kind {
            "1" => Sagyou::Irekae(IrekaeSagyo {
                track_index: argument.parse().map_err(|_| Error::InvalidNumber {
                    field: "RessyaTrack.Irekae.track_index".to_string(),
                    value: argument.to_string(),
                })?,
                departure_time,
                arrival_time,
            }),
            "2" if departure_time == Jikoku::default() && arrival_time == Jikoku::default() => {
                Sagyou::Nyusyukku(NyusyukkuSagyo {
                    unyo_number: argument.to_string(),
                })
            }
            _ => {
                return Err(Error::InvalidFormat {
                    context: "RessyaTrack".to_string(),
                    value: value.to_string(),
                });
            }
        };

        Ok(RessyaTrack {
            track_index: Some(track.parse().map_err(|_| Error::InvalidNumber {
                field: "RessyaTrack".to_string(),
                value: track.to_string(),
            })?),
            sagyou,
        })
    }
}

impl RessyaTrack {
    pub(crate) fn to_oudia_string(&self) -> String {
        match &self.sagyou {
            Sagyou::None => self
                .track_index
                .map(|track_index| track_index.to_string())
                .unwrap_or_default(),
            Sagyou::Irekae(sagyou) => {
                let times = match (
                    sagyou.departure_time.to_oudia_string().is_empty(),
                    sagyou.arrival_time.to_oudia_string().is_empty(),
                ) {
                    (true, true) => String::new(),
                    (false, true) => format!("${}", sagyou.departure_time.to_oudia_string()),
                    (true, false) => format!("$/{}", sagyou.arrival_time.to_oudia_string()),
                    (false, false) => format!(
                        "${}/{}",
                        sagyou.departure_time.to_oudia_string(),
                        sagyou.arrival_time.to_oudia_string()
                    ),
                };
                format!(
                    "{};1/{}{}",
                    self.track_index.unwrap_or_default(), sagyou.track_index, times
                )
            }
            Sagyou::Nyusyukku(sagyou) => {
                if sagyou.unyo_number.is_empty() {
                    format!("{};2", self.track_index.unwrap_or_default())
                } else {
                    format!(
                        "{};2/{}",
                        self.track_index.unwrap_or_default(),
                        sagyou.unyo_number
                    )
                }
            }
        }
    }
}

/// 列車の作業を表す
#[derive(Debug, Default, PartialEq, Clone)]
pub enum Sagyou {
    /// 作業が無いことを表す。
    #[default]
    None,
    /// 入換作業を表す
    Irekae(IrekaeSagyo),
    /// 入出区作業を表す (始終着駅でのみ有効)
    Nyusyukku(NyusyukkuSagyo),
}

/// 入換作業を表す
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IrekaeSagyo {
    /// 入換番線のインデックス
    track_index: usize,
    /// 入換発時刻
    departure_time: Jikoku,
    /// 入換着時刻
    arrival_time: Jikoku,
}

/// 入出区作業を表す
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NyusyukkuSagyo {
    /// 運用番号 (出区の場合)
    unyo_number: String,
}

#[cfg(test)]
mod tests {
    use super::{RessyaTrack, Sagyou};

    #[test]
    fn parses_irekae_without_times() {
        let result: RessyaTrack = "3;1/4".parse().unwrap();

        assert!(matches!(result.sagyou, Sagyou::Irekae(_)));
    }

    #[test]
    fn parses_irekae_with_departure_and_arrival_times() {
        let result: RessyaTrack = "3;1/4$1230/1245".parse().unwrap();

        assert!(matches!(result.sagyou, Sagyou::Irekae(_)));
    }

    #[test]
    fn parses_nyusyukku_with_unyo_number() {
        let result: RessyaTrack = "3;2/unyo1".parse().unwrap();

        assert!(matches!(result.sagyou, Sagyou::Nyusyukku(_)));
    }

    #[test]
    fn parses_an_empty_track() {
        assert_eq!("".parse::<RessyaTrack>().unwrap(), RessyaTrack::default());
    }
}
