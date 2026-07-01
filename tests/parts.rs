use core::time::Duration;

use duration_parts::{DurationParts, Sign};

#[test]
fn duration_to_parts() {
    let duration = Duration::from_secs(93848);

    let duration_parts: DurationParts = duration.try_into().unwrap();

    assert_eq!(1, duration_parts.days());
    assert_eq!(2, duration_parts.hours());
    assert_eq!(4, duration_parts.minutes());
    assert_eq!(8, duration_parts.seconds());
    assert_eq!(93848000000000, duration_parts.to_nanoseconds());
}

#[test]
fn duration_parts_to_duration() {
    let duration_parts =
        DurationParts::new(duration_parts::Sign::Positive, 1, 2, 4, 8, 0, 0, 0).unwrap();

    let duration: Duration = duration_parts.try_into().unwrap();

    assert_eq!(Duration::from_secs(93848), duration);
}

#[test]
fn duration_parts_from_seconds() {
    let duration_parts = DurationParts::from_signed_total_seconds(93848);

    assert_eq!(
        DurationParts::new(duration_parts::Sign::Positive, 1, 2, 4, 8, 0, 0, 0).unwrap(),
        duration_parts
    );
}

#[test]
fn duration_parts_from_milliseconds() {
    let duration_parts = DurationParts::from_signed_total_milliseconds(93848000);

    assert_eq!(
        DurationParts::new(duration_parts::Sign::Positive, 1, 2, 4, 8, 0, 0, 0).unwrap(),
        duration_parts
    );
}

#[test]
fn duration_parts_from_nanoseconds() {
    let duration_parts = DurationParts::from_signed_total_nanoseconds(93848000000000).unwrap();

    assert_eq!(
        DurationParts::new(duration_parts::Sign::Positive, 1, 2, 4, 8, 0, 0, 0).unwrap(),
        duration_parts
    );
}

#[test]
fn duration_parts_abs_keeps_positive_positive() {
    let duration_parts = DurationParts::new(Sign::Positive, 1, 2, 4, 8, 0, 0, 0).unwrap();

    assert_eq!(duration_parts.clone(), duration_parts.abs());
}

#[test]
fn duration_parts_negate_flips_sign() {
    let positive = DurationParts::new(Sign::Positive, 1, 2, 4, 8, 0, 0, 0).unwrap();
    let negative = DurationParts::new(Sign::Negative, 1, 2, 4, 8, 0, 0, 0).unwrap();

    assert_eq!(negative, positive.clone().negate());
    assert_eq!(positive, negative.negate());
}

#[test]
fn duration_parts_from_signed_seconds_handles_i64_min() {
    let duration_parts = DurationParts::from_signed_total_seconds(i64::MIN);

    assert_eq!(Sign::Negative, duration_parts.sign());
    assert_eq!((i64::MIN as i128) * 1_000_000_000, duration_parts.to_nanoseconds());
}
