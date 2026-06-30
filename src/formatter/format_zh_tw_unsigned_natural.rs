use core::fmt::{self, Formatter};

use crate::{DurationFormat, DurationParts, helpers::write_sep};

/// Formats [`DurationParts`] as natural Traditional Chinese (Taiwan) text without writing a sign.
///
/// This formatter writes only non-zero units, ordered from days to nanoseconds.
///
/// Units are separated by one ASCII space.
///
/// If all time units are zero, the output is `0 秒`.
///
/// The sign is not written to the output, so positive and negative values are formatted in the same way.
///
/// # Examples
///
/// ```rust
/// use duration_parts::{
///     DurationParts, FormatDurationParts, Sign,
///     formatters::ZhTwUnsignedNatural,
/// };
///
/// assert_eq!(
///     "0 秒",
///     DurationParts::from_unsigned_total_seconds(0)
///         .format::<ZhTwUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "1 天又 2 小時 3 分鐘",
///     DurationParts::new(Sign::Positive, 1, 2, 3, 0, 0, 0, 0)
///         .unwrap()
///         .format::<ZhTwUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "2 小時 3 分 4 秒",
///     DurationParts::new(Sign::Positive, 0, 2, 3, 4, 0, 0, 0)
///         .unwrap()
///         .format::<ZhTwUnsignedNatural>()
///         .to_string()
/// );
///
/// assert_eq!(
///     "1 天又 2 小時 3 分 4 秒 5 毫秒 6 微秒 7 奈秒",
///     DurationParts::new(Sign::Positive, 1, 2, 3, 4, 5, 6, 7)
///         .unwrap()
///         .format::<ZhTwUnsignedNatural>()
///         .to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZhTwUnsignedNatural;

impl DurationFormat for ZhTwUnsignedNatural {
    fn fmt(duration_parts: &DurationParts, f: &mut Formatter<'_>) -> fmt::Result {
        let mut wrote = false;

        if duration_parts.days > 0 {
            write_sep(&mut wrote, " ", f)?;

            write!(f, "{} 天", duration_parts.days)?;

            if duration_parts.hours > 0
                || duration_parts.minutes > 0
                || duration_parts.seconds > 0
                || duration_parts.milliseconds > 0
                || duration_parts.microseconds > 0
                || duration_parts.nanoseconds > 0
            {
                f.write_str("又")?;
            }
        }

        if duration_parts.hours > 0 {
            write_sep(&mut wrote, " ", f)?;
            write!(f, "{} 小時", duration_parts.hours)?;
        }

        let mut end_with_minute = if duration_parts.minutes > 0 {
            write_sep(&mut wrote, " ", f)?;
            write!(f, "{} 分", duration_parts.minutes)?;

            true
        } else {
            false
        };

        if duration_parts.seconds > 0 {
            write_sep(&mut wrote, " ", f)?;
            write!(f, "{} 秒", duration_parts.seconds)?;

            end_with_minute = false;
        }

        if duration_parts.milliseconds > 0 {
            write_sep(&mut wrote, " ", f)?;
            write!(f, "{} 毫秒", duration_parts.milliseconds)?;

            end_with_minute = false;
        }

        if duration_parts.microseconds > 0 {
            write_sep(&mut wrote, " ", f)?;
            write!(f, "{} 微秒", duration_parts.microseconds)?;

            end_with_minute = false;
        }

        if duration_parts.nanoseconds > 0 {
            write_sep(&mut wrote, " ", f)?;
            write!(f, "{} 奈秒", duration_parts.nanoseconds)?;

            end_with_minute = false;
        }

        if wrote {
            if end_with_minute {
                f.write_str("鐘")?;
            }
        } else {
            f.write_str("0 秒")?;
        }

        Ok(())
    }
}
