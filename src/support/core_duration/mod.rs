use core::time::Duration;

use crate::{DurationParts, DurationPartsConversionError};

impl TryFrom<Duration> for DurationParts {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        DurationParts::from_unsigned_total_nanoseconds(duration.as_nanos())
    }
}

impl TryFrom<DurationParts> for Duration {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(duration_parts: DurationParts) -> Result<Self, Self::Error> {
        let nanos = duration_parts.to_nanoseconds();

        if nanos < 0 {
            return Err(DurationPartsConversionError);
        }

        const MAX_NANOS: u128 = Duration::MAX.as_nanos();

        let nanos = nanos as u128;

        if nanos > MAX_NANOS {
            return Err(DurationPartsConversionError);
        }

        Ok(Duration::from_nanos_u128(nanos))
    }
}
