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

fn main() {
    let duration = Duration::from_secs(3662);

    let duration_parts: DurationParts = duration.try_into().unwrap();

    println!("{}", duration_parts.format::<JaJpDemo>());

    // 1時間1分2秒
}
