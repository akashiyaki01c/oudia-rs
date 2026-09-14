use crate::model::{error::Error, jikoku::Jikoku};
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ekijikoku {
    ekiatsukai: usize,
    chaku: Jikoku,
    hatsu: Jikoku,
}

impl FromStr for Ekijikoku {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Ok(Self {
                ekiatsukai: 0,
                chaku: Jikoku::default(),
                hatsu: Jikoku::default(),
            });
        }

        // ';' で「駅扱い」と「時刻部分」に分離
        let (str_ekiatsukai, rest) = value.split_once(';').unwrap_or((value, ""));

        // 時刻部分を '/' で「着」と「発」に分離
        let (str_chaku, str_hatsu) = if rest.is_empty() {
            ("", "")
        } else if let Some((chaku, hatsu)) = rest.split_once('/') {
            (chaku, hatsu) // 着発あり
        } else {
            ("", rest) // 発のみ
        };

        Ok(Self {
            ekiatsukai: str_ekiatsukai.parse().map_err(|_| Error::TodoError)?,
            chaku: Jikoku::from_str(str_chaku)?,
            hatsu: Jikoku::from_str(str_hatsu)?,
        })
    }
}
