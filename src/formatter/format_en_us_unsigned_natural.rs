use core::fmt::{self, Formatter};

use crate::{DurationFormat, DurationParts, helpers::write_sep};

#[inline]
fn write_en_us_unit<T>(
    f: &mut Formatter<'_>,
    value: T,
    singular: &str,
    plural: &str,
) -> fmt::Result
where
    T: fmt::Display + PartialEq + From<u8> + Copy, {
    if value == T::from(1) {
        write!(f, "{} {}", value, singular)
    } else {
        write!(f, "{} {}", value, plural)
    }
}

#[inline]
fn write_en_us_commas_and_sep(
    wrote: &mut bool,
    total_units: u8,
    remaining_units: u8,
    f: &mut Formatter<'_>,
) -> fmt::Result {
    let sep = if *wrote && remaining_units == 1 {
        if total_units == 2 { " and " } else { ", and " }
    } else {
        ", "
    };

    write_sep(wrote, sep, f)
}

fn count_non_zero_units(duration_parts: &DurationParts) -> u8 {
    u8::from(duration_parts.days > 0)
        + u8::from(duration_parts.hours > 0)
        + u8::from(duration_parts.minutes > 0)
        + u8::from(duration_parts.seconds > 0)
        + u8::from(duration_parts.milliseconds > 0)
        + u8::from(duration_parts.microseconds > 0)
        + u8::from(duration_parts.nanoseconds > 0)
}

/// Formats [`DurationParts`] as natural English (United States) text with commas and `and`, without writing a sign.
///
/// This formatter writes only non-zero units, ordered from days to nanoseconds.
///
/// If there are two non-zero units, `and` is inserted before the last written unit.
///
/// If there are three or more non-zero units, commas are inserted between units and `, and` is inserted before the last written unit.
///
/// English unit names use singular forms when the value is `1`, and plural forms otherwise.
///
/// If all time units are zero, the output is `0 seconds`.
///
/// The sign is not written to the output, so positive and negative values are formatted in the same way.
///
/// # Examples
///
/// ```rust
/// use duration_parts::{
///     DurationParts, FormatDurationParts, Sign,
///     formatters::EnUsUnsignedNatural,
/// };
///
/// assert_eq!(
///     "0 seconds",
///     DurationParts::from_unsigned_total_seconds(0)
///         .format::<EnUsUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "1 day",
///     DurationParts::new(Sign::Positive, 1, 0, 0, 0, 0, 0, 0)
///         .unwrap()
///         .format::<EnUsUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "1 day and 2 hours",
///     DurationParts::new(Sign::Positive, 1, 2, 0, 0, 0, 0, 0)
///         .unwrap()
///         .format::<EnUsUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "1 day, 2 hours, and 3 minutes",
///     DurationParts::new(Sign::Positive, 1, 2, 3, 0, 0, 0, 0)
///         .unwrap()
///         .format::<EnUsUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "1 day, 2 hours, 3 minutes, and 4 seconds",
///     DurationParts::new(Sign::Positive, 1, 2, 3, 4, 0, 0, 0)
///         .unwrap()
///         .format::<EnUsUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "1 day, 2 hours, 3 minutes, 4 seconds, 5 milliseconds, 6 \
///      microseconds, and 7 nanoseconds",
///     DurationParts::new(Sign::Positive, 1, 2, 3, 4, 5, 6, 7)
///         .unwrap()
///         .format::<EnUsUnsignedNatural>()
///         .to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnUsUnsignedNatural;

impl DurationFormat for EnUsUnsignedNatural {
    fn fmt(duration_parts: &DurationParts, f: &mut Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        let total_units = count_non_zero_units(duration_parts);
        let mut remaining_units = total_units;

        if duration_parts.days > 0 {
            write_en_us_commas_and_sep(&mut wrote, total_units, remaining_units, f)?;
            remaining_units -= 1;
            write_en_us_unit(f, duration_parts.days, "day", "days")?;
        }

        if duration_parts.hours > 0 {
            write_en_us_commas_and_sep(&mut wrote, total_units, remaining_units, f)?;
            remaining_units -= 1;
            write_en_us_unit(f, duration_parts.hours, "hour", "hours")?;
        }

        if duration_parts.minutes > 0 {
            write_en_us_commas_and_sep(&mut wrote, total_units, remaining_units, f)?;
            remaining_units -= 1;
            write_en_us_unit(f, duration_parts.minutes, "minute", "minutes")?;
        }

        if duration_parts.seconds > 0 {
            write_en_us_commas_and_sep(&mut wrote, total_units, remaining_units, f)?;
            remaining_units -= 1;
            write_en_us_unit(f, duration_parts.seconds, "second", "seconds")?;
        }

        if duration_parts.milliseconds > 0 {
            write_en_us_commas_and_sep(&mut wrote, total_units, remaining_units, f)?;
            remaining_units -= 1;
            write_en_us_unit(f, duration_parts.milliseconds, "millisecond", "milliseconds")?;
        }

        if duration_parts.microseconds > 0 {
            write_en_us_commas_and_sep(&mut wrote, total_units, remaining_units, f)?;
            remaining_units -= 1;
            write_en_us_unit(f, duration_parts.microseconds, "microsecond", "microseconds")?;
        }

        if duration_parts.nanoseconds > 0 {
            write_en_us_commas_and_sep(&mut wrote, total_units, remaining_units, f)?;
            write_en_us_unit(f, duration_parts.nanoseconds, "nanosecond", "nanoseconds")?;
        }

        if !wrote {
            f.write_str("0 seconds")?;
        }

        Ok(())
    }
}
