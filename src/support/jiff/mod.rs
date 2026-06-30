use jiff::SignedDuration;

use crate::{DurationParts, DurationPartsConversionError};

impl TryFrom<SignedDuration> for DurationParts {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(signed_duration: SignedDuration) -> Result<Self, Self::Error> {
        DurationParts::from_signed_total_nanoseconds(signed_duration.as_nanos())
    }
}

impl TryFrom<DurationParts> for SignedDuration {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(duration_parts: DurationParts) -> Result<Self, Self::Error> {
        SignedDuration::try_from_nanos_i128(duration_parts.to_nanoseconds())
            .ok_or(DurationPartsConversionError)
    }
}
