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
        const MIN_NANOS: i128 = Duration::MIN.whole_nanoseconds();
        const MAX_NANOS: i128 = Duration::MAX.whole_nanoseconds();

        let nanos = duration_parts.to_nanoseconds();

        if !(MIN_NANOS..=MAX_NANOS).contains(&nanos) {
            return Err(DurationPartsConversionError);
        }

        Ok(Duration::nanoseconds_i128(nanos))
    }
}
