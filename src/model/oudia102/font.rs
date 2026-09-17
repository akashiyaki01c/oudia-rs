use std::collections::HashMap;

use crate::model::oudia102::error::Error;

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
                Ok::<(&str, &str), Error>((
                    a.next().ok_or(Error::TodoError)?,
                    a.next().ok_or(Error::TodoError)?,
                ))
            })
            .collect::<Result<_, _>>()?;

        let mut result = FontProp::default();

        // PointTextHeight
        if let Some(prop) = connected_string.get("PointTextHeight") {
            result.point_text_height = prop.parse().map_err(|_| Error::TodoError)?;
        }

        // LogicalunitTextHeight
        if let Some(prop) = connected_string.get("LogicalunitTextHeight") {
            result.logicalunit_text_height = prop.parse().map_err(|_| Error::TodoError)?;
        }

        // LogicalunitCellHeight
        if let Some(prop) = connected_string.get("LogicalunitCellHeight") {
            result.logicalunit_cell_height = prop.parse().map_err(|_| Error::TodoError)?;
        }

        // Facename
        if let Some(prop) = connected_string.get("Facename") {
            if prop.is_empty() {
                return Err(Error::EmptyValue("Facename".to_string()));
            }
            result.facename = prop.to_string();
        } else {
            return Err(Error::KeyIsNotFound("Facename".to_string()));
        }

        // Bold
        if let Some(prop) = connected_string.get("Bold") {
            result.bold = *prop == "1";
        }

        // Itaric
        if let Some(prop) = connected_string.get("Itaric") {
            result.itaric = *prop == "1";
        }

        // Underline
        if let Some(prop) = connected_string.get("Underline") {
            result.underine = *prop == "1";
        }

        // StrikeOut
        if let Some(prop) = connected_string.get("StrikeOut") {
            result.strike_out = *prop == "1";
        }

        // Escapement
        if let Some(prop) = connected_string.get("Escapement") {
            result.escapement = prop.parse().map_err(|_| Error::TodoError)?;
        }

        Ok(result)
    }

    pub fn to_oudia_string(&self) -> String {
        let mut values = Vec::new();
        if self.point_text_height != 0 {
            values.push(format!("PointTextHeight={}", self.point_text_height));
        }
        if self.logicalunit_text_height != 0 {
            values.push(format!(
                "LogicalunitTextHeight={}",
                self.logicalunit_text_height
            ));
        }
        if self.logicalunit_cell_height != 0 {
            values.push(format!(
                "LogicalunitCellHeight={}",
                self.logicalunit_cell_height
            ));
        }
        values.push(format!("Facename={}", self.facename));
        if self.bold {
            values.push("Bold=1".to_string());
        }
        if self.itaric {
            values.push("Itaric=1".to_string());
        }
        if self.underine {
            values.push("Underline=1".to_string());
        }
        if self.strike_out {
            values.push("StrikeOut=1".to_string());
        }
        if self.escapement != 0 {
            values.push(format!("Escapement={}", self.escapement));
        }
        values.join(";")
    }
}
