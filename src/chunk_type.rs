use anyhow::{Error, Result};
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ChunkTypeError {
    #[error("Bytes have to be within the ASCII range.")]
    InvalidEncoding,

    #[error("Expected 4 bytes, got {found:?} bytes.")]
    InvalidLength { found: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        if value.is_ascii() {
            Ok(Self { bytes: value })
        } else {
            Err(ChunkTypeError::InvalidEncoding.into())
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Ok(bytes) = <[u8; 4]>::try_from(s.as_bytes()) else {
            return Err(ChunkTypeError::InvalidLength {
                found: s.as_bytes().len().to_string(),
            }
            .into());
        };

        Ok(Self::try_from(bytes)?)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match std::str::from_utf8(&self.bytes) {
            Ok(value) => value.to_string(),
            Err(_) => "\u{FFFD}".repeat(4),
        };

        write!(f, "{}", string)
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn is_valid() {
        let chunk_from_bytes = ChunkType::try_from([82, 117, 83, 116]);
        let chunk_from_string = ChunkType::from_str("RuSt");

        assert!(chunk_from_bytes.is_ok());
        assert!(chunk_from_string.is_ok());
        assert_eq!(chunk_from_bytes.unwrap(), chunk_from_string.unwrap());
    }

    #[test]
    pub fn is_invalid_encoding() {
        let chunk = ChunkType::from_str("R華");

        assert!(chunk.is_err());
        assert_eq!(
            chunk.err().unwrap().downcast::<ChunkTypeError>().unwrap(),
            ChunkTypeError::InvalidEncoding
        );
    }

    #[test]
    pub fn is_invalid_size() {
        let chunk = ChunkType::from_str("RuStt");

        assert!(chunk.is_err());
        assert_eq!(
            chunk.err().unwrap().downcast::<ChunkTypeError>().unwrap(),
            ChunkTypeError::InvalidLength {
                found: "5".to_string()
            }
        );
    }

    #[test]
    pub fn is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }
}
