/*!
# Duration Parts

A lightweight duration decomposition and formatting crate.

## Supported Date-Time Crates

Date-time crate support is enabled through Cargo features. Enable only the providers your project uses.

| Feature | Type |
| --- | --- |
| `chrono` | `chrono::TimeDelta` |
| `jiff` | `jiff::SignedDuration` |
| `time` | `time::Duration` |

## Usage

```rust
# #[cfg(feature = "format-en-us-unsigned-natural")]
# {
use core::time::Duration;

use duration_parts::{DurationParts, FormatDurationParts, formatters::EnUsUnsignedNatural};

let duration = Duration::from_secs(3662);

let duration_parts: DurationParts = duration.try_into().unwrap();

assert_eq!("1 hour, 1 minute, and 2 seconds", duration_parts.format::<EnUsUnsignedNatural>().to_string());
# }
```

See the `duration_parts::formatters` module for more built-in formatters.

You can make your own formatter by implementing the `DurationFormat` trait. For example,

```rust
use core::{
    fmt::{self, Formatter},
    time::Duration,
};

use duration_parts::{DurationFormat, DurationParts, FormatDurationParts};

struct JaJpDemo;

impl DurationFormat for JaJpDemo {
    fn fmt(duration_parts: &DurationParts, f: &mut Formatter<'_>) -> fmt::Result {
        if duration_parts.days() > 0 {
            write!(f, "{}日", duration_parts.days())?;
        }

        if duration_parts.hours() > 0 {
            write!(f, "{}時間", duration_parts.hours())?;
        }

        if duration_parts.minutes() > 0 {
            write!(f, "{}分", duration_parts.minutes())?;
        }

        if duration_parts.seconds() > 0 {
            write!(f, "{}秒", duration_parts.seconds())?;
        }

        Ok(())
    }
}

let duration = Duration::from_secs(3662);

let duration_parts: DurationParts = duration.try_into().unwrap();

assert_eq!("1時間1分2秒", duration_parts.format::<JaJpDemo>().to_string());
```
*/

#![cfg_attr(docsrs, feature(doc_cfg))]

mod formatter;
mod parts;
mod support;

pub use formatter::*;
pub use parts::*;
