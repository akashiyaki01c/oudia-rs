use std::collections::HashMap;

use crate::model::error::Error;

const KEY_POINT_TEXT_HEIGHT: &str = "PointTextHeight";
const KEY_LOGICALUNIT_TEXT_HEIGHT: &str = "LogicalunitTextHeight";
const KEY_LOGICALUNIT_CELL_HEIGHT: &str = "LogicalunitCellHeight";
const KEY_FACENAME: &str = "Facename";
const KEY_BOLD: &str = "Bold";
const KEY_ITARIC: &str = "Itaric";
const KEY_UNDERLINE: &str = "Underline";
const KEY_STRIKE_OUT: &str = "StrikeOut";
const KEY_ESCAPEMENT: &str = "Escapement";

/// 1つのフォント設定を表す構造体
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FontProp {
    point_text_height: usize,
    logicalunit_text_height: usize,
    logicalunit_cell_height: usize,
    facename: String,
    bold: bool,
    itaric: bool,
    underine: bool,
    strike_out: bool,
    escapement: usize,
}
impl FontProp {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        let connected_string: HashMap<&str, &str> = value
            .split(";")
            .map(|v| {
                let mut a = v.split("=");
                let key = a.next().ok_or_else(|| Error::InvalidFormat {
                    context: "font property".to_string(),
                    value: v.to_string(),
                })?;
                let value = a.next().ok_or_else(|| Error::InvalidFormat {
                    context: "font property".to_string(),
                    value: v.to_string(),
                })?;
                Ok::<(&str, &str), Error>((key, value))
            })
            .collect::<Result<_, _>>()?;

        let mut result = FontProp::default();

        // PointTextHeight
        if let Some(prop) = connected_string.get(KEY_POINT_TEXT_HEIGHT) {
            result.point_text_height = prop.parse().map_err(|_| Error::InvalidNumber {
                field: KEY_POINT_TEXT_HEIGHT.to_string(),
                value: prop.to_string(),
            })?;
        }

        // LogicalunitTextHeight
        if let Some(prop) = connected_string.get(KEY_LOGICALUNIT_TEXT_HEIGHT) {
            result.logicalunit_text_height = prop.parse().map_err(|_| Error::InvalidNumber {
                field: KEY_LOGICALUNIT_TEXT_HEIGHT.to_string(),
                value: prop.to_string(),
            })?;
        }

        // LogicalunitCellHeight
        if let Some(prop) = connected_string.get(KEY_LOGICALUNIT_CELL_HEIGHT) {
            result.logicalunit_cell_height = prop.parse().map_err(|_| Error::InvalidNumber {
                field: KEY_LOGICALUNIT_CELL_HEIGHT.to_string(),
                value: prop.to_string(),
            })?;
        }

        // Facename
        if let Some(prop) = connected_string.get(KEY_FACENAME) {
            if prop.is_empty() {
                return Err(Error::EmptyValue(KEY_FACENAME.to_string()));
            }
            result.facename = prop.to_string();
        } else {
            return Err(Error::KeyIsNotFound(KEY_FACENAME.to_string()));
        }

        // Bold
        if let Some(prop) = connected_string.get(KEY_BOLD) {
            result.bold = *prop == "1";
        }

        // Itaric
        if let Some(prop) = connected_string.get(KEY_ITARIC) {
            result.itaric = *prop == "1";
        }

        // Underline
        if let Some(prop) = connected_string.get(KEY_UNDERLINE) {
            result.underine = *prop == "1";
        }

        // StrikeOut
        if let Some(prop) = connected_string.get(KEY_STRIKE_OUT) {
            result.strike_out = *prop == "1";
        }

        // Escapement
        if let Some(prop) = connected_string.get(KEY_ESCAPEMENT) {
            result.escapement = prop.parse().map_err(|_| Error::InvalidNumber {
                field: KEY_ESCAPEMENT.to_string(),
                value: prop.to_string(),
            })?;
        }

        Ok(result)
    }

    pub fn to_oudia_string(&self) -> String {
        let mut values = Vec::new();
        if self.point_text_height != 0 {
            values.push(format!(
                "{KEY_POINT_TEXT_HEIGHT}={}",
                self.point_text_height
            ));
        }
        if self.logicalunit_text_height != 0 {
            values.push(format!(
                "{KEY_LOGICALUNIT_TEXT_HEIGHT}={}",
                self.logicalunit_text_height
            ));
        }
        if self.logicalunit_cell_height != 0 {
            values.push(format!(
                "{KEY_LOGICALUNIT_CELL_HEIGHT}={}",
                self.logicalunit_cell_height
            ));
        }
        values.push(format!("{KEY_FACENAME}={}", self.facename));
        if self.bold {
            values.push(format!("{KEY_BOLD}={}", 1));
        }
        if self.itaric {
            values.push(format!("{KEY_ITARIC}={}", 1));
        }
        if self.underine {
            values.push(format!("{KEY_UNDERLINE}={}", 1));
        }
        if self.strike_out {
            values.push(format!("{KEY_STRIKE_OUT}={}", 1));
        }
        if self.escapement != 0 {
            values.push(format!("{KEY_ESCAPEMENT}={}", self.escapement));
        }
        values.join(";")
    }
}
