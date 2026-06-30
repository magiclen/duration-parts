use time::Duration;

use crate::{DurationParts, DurationPartsConversionError};

impl TryFrom<Duration> for DurationParts {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        DurationParts::from_signed_total_nanoseconds(duration.whole_nanoseconds())
    }
}

impl TryFrom<DurationParts> for Duration {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(duration_parts: DurationParts) -> Result<Self, Self::Error> {
        Ok(Duration::nanoseconds_i128(duration_parts.to_nanoseconds()))
    }
}
