use std::collections::HashMap;

use crate::model::error::Error;

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
            result.facename = prop.to_string();
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
        format!(
            "PointTextHeight={};LogicalunitTextHeight={};LogicalunitCellHeight={};Facename={};Bold={};Itaric={};Underline={};StrikeOut={};Escapement={}",
            self.point_text_height,
            self.logicalunit_text_height,
            self.logicalunit_cell_height,
            self.facename,
            self.bold as u8,
            self.itaric as u8,
            self.underine as u8,
            self.strike_out as u8,
            self.escapement,
        )
    }
}
