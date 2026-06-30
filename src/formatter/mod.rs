#[cfg(feature = "format-en-us-unsigned-natural")]
mod format_en_us_unsigned_natural;
#[cfg(feature = "format-zh-tw-unsigned-natural")]
mod format_zh_tw_unsigned_natural;
/// Utility functions for formatting durations.
pub mod helpers;

use core::fmt::{self, Display, Formatter};

/// Formatter presets are selected by type, so formatting uses static dispatch and writes directly into [`core::fmt::Formatter`].
pub mod formatters {
    #[cfg(feature = "format-en-us-unsigned-natural")]
    pub use super::format_en_us_unsigned_natural::*;
    #[cfg(feature = "format-zh-tw-unsigned-natural")]
    pub use super::format_zh_tw_unsigned_natural::*;
}

use crate::DurationParts;

/// A zero-sized formatter wrapper returned by [`FormatDurationParts::format`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormattedDuration<'a, F> {
    parts:   &'a DurationParts,
    _format: core::marker::PhantomData<F>,
}

impl<'a, F> FormattedDuration<'a, F> {
    /// Create a formatted duration wrapper.
    #[inline]
    pub const fn new(parts: &'a DurationParts) -> Self {
        Self {
            parts,
            _format: core::marker::PhantomData,
        }
    }
}

impl<'a, F> AsRef<DurationParts> for FormattedDuration<'a, F> {
    #[inline]
    fn as_ref(&self) -> &DurationParts {
        self.parts
    }
}

/// A fixed duration formatter preset.
pub trait DurationFormat {
    /// Write `parts` into `f`.
    fn fmt(parts: &DurationParts, f: &mut Formatter<'_>) -> fmt::Result;
}

/// Formats [`DurationParts`] with a type-selected formatter preset.
pub trait FormatDurationParts {
    /// Format the duration parts using formatter `F`.
    fn format<'a, F>(&'a self) -> FormattedDuration<'a, F>
    where
        F: DurationFormat;
}

impl FormatDurationParts for DurationParts {
    #[inline]
    fn format<'a, F>(&'a self) -> FormattedDuration<'a, F>
    where
        F: DurationFormat, {
        FormattedDuration::new(self)
    }
}

impl<'a, F> Display for FormattedDuration<'a, F>
where
    F: DurationFormat,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        F::fmt(self.parts, f)
    }
}
