use core::fmt::{self, Formatter};

/// Writes a separator string to the formatter if `wrote` is `true`, and sets `wrote` to `true`. This is useful for formatting lists of items with separators.
#[inline]
pub fn write_sep(wrote: &mut bool, sep: &str, f: &mut Formatter<'_>) -> fmt::Result {
    if *wrote {
        f.write_str(sep)?;
    } else {
        *wrote = true;
    }

    Ok(())
}
