use std::str::FromStr;

use crate::model::error::Error;

/// 色をRGBの3色で表す構造体
#[derive(Debug, Default, PartialEq, Clone, Copy, Eq)]
pub struct ColorProp {
    /// 赤色成分 [0-255]
    red: u8,
    /// 緑色成分 [0-255]
    green: u8,
    /// 青色成分 [0-255]
    blue: u8,
}
impl FromStr for ColorProp {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value: usize = usize::from_str_radix(value, 16).map_err(|_| Error::TodoError)?;
        let red = (value >> 16 & 0xff) as u8;
        let green = ((value >> 8) & 0xff) as u8;
        let blue = ((value) & 0xff) as u8;
        Ok(Self { red, green, blue })
    }
}
impl ToString for ColorProp {
    fn to_string(&self) -> String {
        format!("00{:X}{:X}{:X}", self.red, self.green, self.blue)
    }
}

#[test]
fn color_props_to_string() {
    let color = ColorProp {
        red: 0x1f,
        green: 0x7f,
        blue: 0xff,
    };

    let str = color.to_string();
    assert_eq!(&str, "001F7FFF");
    let parsed = ColorProp::from_str(&str).unwrap();
    assert_eq!(color, parsed)
}
