use crate::model::oudia102::{error::Error, jikoku::Jikoku};
use std::str::FromStr;

/// 一つの駅時刻を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ekijikoku {
    /// 駅扱いの種別
    ekiatsukai: usize,
    /// 到着時刻
    chaku: Jikoku,
    /// 発車時刻
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

impl Ekijikoku {
    pub(crate) fn to_oudia_string(&self) -> String {
        let times = match (
            self.chaku.to_oudia_string().is_empty(),
            self.hatsu.to_oudia_string().is_empty(),
        ) {
            (true, true) => String::new(),
            (true, false) => self.hatsu.to_oudia_string(),
            (false, true) => format!("{}/", self.chaku.to_oudia_string()),
            (false, false) => format!(
                "{}/{}",
                self.chaku.to_oudia_string(),
                self.hatsu.to_oudia_string()
            ),
        };
        if times.is_empty() && self.ekiatsukai == 0 {
            String::new()
        } else {
            format!("{};{}", self.ekiatsukai, times)
        }
    }
}
