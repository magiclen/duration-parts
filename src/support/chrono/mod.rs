use chrono::TimeDelta;

use crate::{DurationParts, DurationPartsConversionError};

impl TryFrom<TimeDelta> for DurationParts {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(time_delta: TimeDelta) -> Result<Self, Self::Error> {
        DurationParts::from_signed_total_nanoseconds(
            time_delta.num_nanoseconds().ok_or(DurationPartsConversionError)? as i128,
        )
    }
}

impl TryFrom<DurationParts> for TimeDelta {
    type Error = DurationPartsConversionError;

    #[inline]
    fn try_from(duration_parts: DurationParts) -> Result<Self, Self::Error> {
        let nanos = duration_parts.to_nanoseconds();

        if !(i64::MIN as i128..=i64::MAX as i128).contains(&nanos) {
            return Err(DurationPartsConversionError);
        }

        Ok(TimeDelta::nanoseconds(nanos as i64))
    }
}
