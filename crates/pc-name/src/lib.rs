use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub const MAX_PC_NAME_LENGTH: usize = 15;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    Empty,
    TooLong { length: usize },
    InvalidCharacter { character: char },
    MissingAlphabeticCharacter,
    EdgeHyphen,
}

impl Display for ValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "PC名が空です"),
            Self::TooLong { length } => write!(
                formatter,
                "PC名は{MAX_PC_NAME_LENGTH}文字以内にしてください（現在: {length}文字）"
            ),
            Self::InvalidCharacter { character } => write!(
                formatter,
                "PC名に使用できない文字 '{character}' があります（半角英数字とハイフンのみ使用可）"
            ),
            Self::MissingAlphabeticCharacter => {
                write!(formatter, "PC名には英字を1文字以上含めてください")
            }
            Self::EdgeHyphen => write!(formatter, "PC名の先頭・末尾にハイフンは使用できません"),
        }
    }
}

impl Error for ValidationError {}

pub fn validate(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::Empty);
    }

    let length = name.chars().count();
    if length > MAX_PC_NAME_LENGTH {
        return Err(ValidationError::TooLong { length });
    }

    if let Some(character) = name
        .chars()
        .find(|character| !character.is_ascii_alphanumeric() && *character != '-')
    {
        return Err(ValidationError::InvalidCharacter { character });
    }

    if !name
        .chars()
        .any(|character| character.is_ascii_alphabetic())
    {
        return Err(ValidationError::MissingAlphabeticCharacter);
    }

    if name.starts_with('-') || name.ends_with('-') {
        return Err(ValidationError::EdgeHyphen);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate, ValidationError};

    #[test]
    fn accepts_valid_pc_names() {
        for name in ["PC-001", "ROOM-A12", "A", "PC1234567890123"] {
            assert!(validate(name).is_ok(), "{name} should be valid");
        }
    }

    #[test]
    fn rejects_invalid_pc_names() {
        let cases = [
            ("", ValidationError::Empty),
            ("12345", ValidationError::MissingAlphabeticCharacter),
            ("-PC001", ValidationError::EdgeHyphen),
            ("PC001-", ValidationError::EdgeHyphen),
            (
                "PC_001",
                ValidationError::InvalidCharacter { character: '_' },
            ),
            (
                "PC NAME",
                ValidationError::InvalidCharacter { character: ' ' },
            ),
            ("PC12345678901234", ValidationError::TooLong { length: 16 }),
        ];

        for (name, expected) in cases {
            assert_eq!(validate(name), Err(expected), "{name} should be invalid");
        }
    }
}
