use crate::{DurationPartsConversionError, parts::sign::Sign};

/// A normalized, fixed-duration decomposition.
///
/// The fields are normalized as follows:
///
/// * `days`: unbounded except for `u64` storage
/// * `hours`: `0..=23`
/// * `minutes`: `0..=59`
/// * `seconds`: `0..=59`
/// * `milliseconds`: `0..=999`
/// * `microseconds`: `0..=999`
/// * `nanoseconds`: `0..=999`
///
/// `DurationParts` deliberately has no year or month fields because those are calendar-dependent units.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DurationParts {
    /// Sign of the original duration-like value.
    pub(crate) sign:         Sign,
    /// Whole days.
    pub(crate) days:         u64,
    /// Remainder hours after removing whole days.
    pub(crate) hours:        u8,
    /// Remainder minutes after removing whole hours.
    pub(crate) minutes:      u8,
    /// Remainder seconds after removing whole minutes.
    pub(crate) seconds:      u8,
    /// Remainder milliseconds after removing whole seconds.
    pub(crate) milliseconds: u16,
    /// Remainder microseconds after removing whole milliseconds.
    pub(crate) microseconds: u16,
    /// Remainder nanoseconds after removing whole microseconds.
    pub(crate) nanoseconds:  u16,
}

impl DurationParts {
    #[allow(clippy::too_many_arguments)]
    /// Creates a new `DurationParts` instance.
    ///
    /// Numerical values are validated to ensure they are within the expected ranges. If any value is out of range, `None` is returned.
    ///
    /// * `days`: unbounded except for `u64` storage
    /// * `hours`: `0..=23`
    /// * `minutes`: `0..=59`
    /// * `seconds`: `0..=59`
    /// * `milliseconds`: `0..=999`
    /// * `microseconds`: `0..=999`
    /// * `nanoseconds`: `0..=999`
    #[inline]
    pub const fn new(
        sign: Sign,
        days: u64,
        hours: u8,
        minutes: u8,
        seconds: u8,
        milliseconds: u16,
        microseconds: u16,
        nanoseconds: u16,
    ) -> Option<Self> {
        if hours > 23
            || minutes > 59
            || seconds > 59
            || milliseconds > 999
            || microseconds > 999
            || nanoseconds > 999
        {
            return None;
        }

        Some(unsafe {
            Self::new_unchecked(
                sign,
                days,
                hours,
                minutes,
                seconds,
                milliseconds,
                microseconds,
                nanoseconds,
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    /// Creates a new `DurationParts` instance.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided values are valid and normalized.
    #[inline]
    pub const unsafe fn new_unchecked(
        sign: Sign,
        days: u64,
        hours: u8,
        minutes: u8,
        seconds: u8,
        milliseconds: u16,
        microseconds: u16,
        nanoseconds: u16,
    ) -> Self {
        Self {
            sign,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        }
    }

    /// Creates a new `DurationParts` instance from a signed nanosecond count.
    #[inline]
    pub const fn from_signed_total_nanoseconds(
        total_nanoseconds: i128,
    ) -> Result<Self, DurationPartsConversionError> {
        let (sign, total_nanoseconds_unsigned) = if total_nanoseconds < 0 {
            (Sign::Negative, total_nanoseconds.unsigned_abs())
        } else {
            (Sign::Positive, total_nanoseconds as u128)
        };

        let mut parts = match Self::from_unsigned_total_nanoseconds(total_nanoseconds_unsigned) {
            Ok(parts) => parts,
            Err(_) => return Err(DurationPartsConversionError),
        };

        parts.sign = sign;

        Ok(parts)
    }

    /// Creates a new `DurationParts` instance from an unsigned nanosecond count. (always positive)
    pub const fn from_unsigned_total_nanoseconds(
        total_nanoseconds: u128,
    ) -> Result<Self, DurationPartsConversionError> {
        let sign = Sign::Positive;

        pub(crate) const NANOS_PER_MICROSECOND: u128 = 1_000;
        pub(crate) const NANOS_PER_MILLISECOND: u128 = 1_000_000;
        pub(crate) const NANOS_PER_SECOND: u128 = 1_000_000_000;
        pub(crate) const NANOS_PER_MINUTE: u128 = 60 * NANOS_PER_SECOND;
        pub(crate) const NANOS_PER_HOUR: u128 = 60 * NANOS_PER_MINUTE;
        pub(crate) const NANOS_PER_DAY: u128 = 24 * NANOS_PER_HOUR;

        let days = total_nanoseconds / NANOS_PER_DAY;

        let days = if days > u64::MAX as u128 {
            return Err(DurationPartsConversionError);
        } else {
            days as u64
        };

        let rem = total_nanoseconds % NANOS_PER_DAY;

        let hours = rem / NANOS_PER_HOUR;
        let rem = rem % NANOS_PER_HOUR;

        let minutes = rem / NANOS_PER_MINUTE;
        let rem = rem % NANOS_PER_MINUTE;

        let seconds = rem / NANOS_PER_SECOND;
        let rem = rem % NANOS_PER_SECOND;

        let milliseconds = rem / NANOS_PER_MILLISECOND;
        let rem = rem % NANOS_PER_MILLISECOND;

        let microseconds = rem / NANOS_PER_MICROSECOND;
        let nanoseconds = rem % NANOS_PER_MICROSECOND;

        Ok(unsafe {
            Self::new_unchecked(
                sign,
                days,
                hours as u8,
                minutes as u8,
                seconds as u8,
                milliseconds as u16,
                microseconds as u16,
                nanoseconds as u16,
            )
        })
    }

    /// Creates a new `DurationParts` instance from a signed millisecond count.
    ///
    /// Returns `None` if the total milliseconds cannot be represented as a `DurationParts`.
    #[inline]
    pub const fn from_signed_total_milliseconds(total_milliseconds: i64) -> Self {
        let (sign, total_milliseconds_unsigned) = if total_milliseconds < 0 {
            (Sign::Negative, total_milliseconds.unsigned_abs())
        } else {
            (Sign::Positive, total_milliseconds as u64)
        };

        let mut parts = Self::from_unsigned_total_milliseconds(total_milliseconds_unsigned);

        parts.sign = sign;

        parts
    }

    /// Creates a new `DurationParts` instance from an unsigned millisecond count. (always positive)
    ///
    /// Returns `None` if the total milliseconds cannot be represented as a `DurationParts`.
    pub const fn from_unsigned_total_milliseconds(total_milliseconds: u64) -> Self {
        let sign = Sign::Positive;

        pub(crate) const MILLS_PER_SECOND: u64 = 1_000;
        pub(crate) const MILLS_PER_MINUTE: u64 = 60 * MILLS_PER_SECOND;
        pub(crate) const MILLS_PER_HOUR: u64 = 60 * MILLS_PER_MINUTE;
        pub(crate) const MILLS_PER_DAY: u64 = 24 * MILLS_PER_HOUR;

        let days = total_milliseconds / MILLS_PER_DAY;
        let rem = total_milliseconds % MILLS_PER_DAY;

        let hours = rem / MILLS_PER_HOUR;
        let rem = rem % MILLS_PER_HOUR;

        let minutes = rem / MILLS_PER_MINUTE;
        let rem = rem % MILLS_PER_MINUTE;

        let seconds = rem / MILLS_PER_SECOND;
        let milliseconds = rem % MILLS_PER_SECOND;

        unsafe {
            Self::new_unchecked(
                sign,
                days,
                hours as u8,
                minutes as u8,
                seconds as u8,
                milliseconds as u16,
                0,
                0,
            )
        }
    }

    /// Creates a new `DurationParts` instance from a signed second count.
    ///
    /// Returns `None` if the total seconds cannot be represented as a `DurationParts`.
    #[inline]
    pub const fn from_signed_total_seconds(total_seconds: i64) -> Self {
        let (sign, total_seconds_unsigned) = if total_seconds < 0 {
            (Sign::Negative, total_seconds.unsigned_abs())
        } else {
            (Sign::Positive, total_seconds as u64)
        };

        let mut parts = Self::from_unsigned_total_milliseconds(total_seconds_unsigned * 1_000);

        parts.sign = sign;

        parts
    }

    /// Creates a new `DurationParts` instance from an unsigned second count. (always positive)
    ///
    /// Returns `None` if the total seconds cannot be represented as a `DurationParts`.
    pub const fn from_unsigned_total_seconds(total_seconds: u64) -> Self {
        let sign = Sign::Positive;

        pub(crate) const SECONDS_PER_MINUTE: u64 = 60;
        pub(crate) const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
        pub(crate) const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;

        let days = total_seconds / SECONDS_PER_DAY;
        let rem = total_seconds % SECONDS_PER_DAY;

        let hours = rem / SECONDS_PER_HOUR;
        let rem = rem % SECONDS_PER_HOUR;

        let minutes = rem / SECONDS_PER_MINUTE;
        let seconds = rem % SECONDS_PER_MINUTE;

        unsafe {
            Self::new_unchecked(sign, days, hours as u8, minutes as u8, seconds as u8, 0, 0, 0)
        }
    }
}

impl DurationParts {
    /// Returns the sign of the original duration-like value.
    #[inline]
    pub const fn sign(&self) -> Sign {
        self.sign
    }

    /// Returns the whole days.
    #[inline]
    pub const fn days(&self) -> u64 {
        self.days
    }

    /// Returns the remainder hours after removing whole days.
    #[inline]
    pub const fn hours(&self) -> u8 {
        self.hours
    }

    /// Returns the remainder minutes after removing whole hours.
    #[inline]
    pub const fn minutes(&self) -> u8 {
        self.minutes
    }

    /// Returns the remainder seconds after removing whole minutes.
    #[inline]
    pub const fn seconds(&self) -> u8 {
        self.seconds
    }

    /// Returns the remainder milliseconds after removing whole seconds.
    #[inline]
    pub const fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    /// Returns the remainder microseconds after removing whole milliseconds.
    #[inline]
    pub const fn microseconds(&self) -> u16 {
        self.microseconds
    }

    /// Returns the remainder nanoseconds after removing whole microseconds.
    #[inline]
    pub const fn nanoseconds(&self) -> u16 {
        self.nanoseconds
    }
}

impl DurationParts {
    /// Returns `true` if all time units are zero.
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
            && self.milliseconds == 0
            && self.microseconds == 0
            && self.nanoseconds == 0
    }

    /// Returns a new `DurationParts` instance with the sign inverted.
    #[inline]
    pub const fn abs(mut self) -> Self {
        self.sign = self.sign.invert();

        self
    }

    /// Returns the total duration represented by this `DurationParts` in nanoseconds.
    #[inline]
    pub const fn to_nanoseconds(&self) -> i128 {
        pub(crate) const NANOS_PER_MICROSECOND: i128 = 1_000;
        pub(crate) const NANOS_PER_MILLISECOND: i128 = 1_000_000;
        pub(crate) const NANOS_PER_SECOND: i128 = 1_000_000_000;
        pub(crate) const NANOS_PER_MINUTE: i128 = 60 * NANOS_PER_SECOND;
        pub(crate) const NANOS_PER_HOUR: i128 = 60 * NANOS_PER_MINUTE;
        pub(crate) const NANOS_PER_DAY: i128 = 24 * NANOS_PER_HOUR;

        let mut total_nanoseconds = self.nanoseconds as i128;

        total_nanoseconds += (self.microseconds as i128) * NANOS_PER_MICROSECOND;
        total_nanoseconds += (self.milliseconds as i128) * NANOS_PER_MILLISECOND;
        total_nanoseconds += (self.seconds as i128) * NANOS_PER_SECOND;
        total_nanoseconds += (self.minutes as i128) * NANOS_PER_MINUTE;
        total_nanoseconds += (self.hours as i128) * NANOS_PER_HOUR;
        total_nanoseconds += (self.days as i128) * NANOS_PER_DAY;

        match self.sign {
            Sign::Positive => total_nanoseconds,
            Sign::Negative => -total_nanoseconds,
        }
    }
}
