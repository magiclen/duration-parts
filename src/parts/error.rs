use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// A zero-sized error type indicating a failed conversion between [`DurationParts`](crate::DurationParts) and other duration-like types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DurationPartsConversionError;

impl Display for DurationPartsConversionError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DurationParts conversion error")
    }
}

impl Error for DurationPartsConversionError {}
