use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum HeadingDepth {
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
    Level6 = 6,
}

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("todo")]
    InvalidValue(u8),
}

impl TryFrom<u8> for HeadingDepth {
    type Error = ConvertError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            4 => Self::Level4,
            5 => Self::Level5,
            6 => Self::Level6,
            _ => return Err(ConvertError::InvalidValue(value)),
        })
    }
}
