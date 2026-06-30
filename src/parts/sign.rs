/// Sign of a duration-like value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    /// Zero or positive duration.
    Positive,
    /// Negative duration.
    Negative,
}

impl Sign {
    /// Returns `true` for [`Sign::Negative`].
    #[inline]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Negative)
    }

    /// Returns `true` for [`Sign::Positive`].
    #[inline]
    pub const fn is_positive(self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns the inverted sign.
    #[inline]
    pub const fn invert(self) -> Self {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}
