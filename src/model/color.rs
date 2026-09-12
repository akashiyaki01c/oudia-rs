use crate::model::error::Error;

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq)]
pub struct ColorProp {
    red: u8,
    green: u8,
    blue: u8,
}
impl ColorProp {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        let value: usize = usize::from_str_radix(value, 16).map_err(|_| Error::TodoError)?;
        let red = ((value >> 0) & 0xff) as u8;
        let green = ((value >> 8) & 0xff) as u8;
        let blue = ((value >> 16) & 0xff) as u8;
        Ok(Self { red, green, blue })
    }
}
