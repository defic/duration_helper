//! `DurationHelper` - A Rust crate to simplify the creation of Duration objects.
//!
//! This library provides a set of helper methods to easily construct `std::time::Duration` objects from integers and floating point numbers.
//! By mimicking the naming conventions from `std::time::Duration`, it aims to provide an intuitive way to create durations.
//!
//! # Examples
//! 
//! ```rust
//! use duration_helper::DurationHelper;
//!
//! let five_seconds = 5.secs();
//! let two_hours = 2.hours();
//! let half_a_day = 0.5.days();
//! ```
//!
//! # Features
//! - Convert numbers directly to Duration objects.
//! - Intuitive naming aligned with `std::time::Duration`.
//! - Support for a wide range of time units, from nanoseconds to years.
//! - Both integer and floating point number types are supported.
//!
//! Note: This library makes some assumptions, particularly for larger durations. For example, a month is considered as 30 days and a year is considered as 365 days.

use std::time::Duration;

pub trait DurationHelper {
    fn nanos(self) -> Duration;
    fn micros(self) -> Duration;
    fn millis(self) -> Duration;
    fn secs(self) -> Duration;
    fn hours(self) -> Duration;
    fn days(self) -> Duration;
    fn weeks(self) -> Duration;
    /// Assuming a month is 30 days
    fn months(self) -> Duration;
    /// Assuming a year is 365 days
    fn years(self) -> Duration;
}

impl DurationHelper for u64 {
    fn nanos(self) -> Duration { Duration::from_nanos(self) }
    fn micros(self) -> Duration { Duration::from_micros(self) }
    fn millis(self) -> Duration { Duration::from_millis(self) }
    fn secs(self) -> Duration { Duration::from_secs(self) }
    fn hours(self) -> Duration { Duration::from_secs(60 * 60 * self) }
    fn days(self) -> Duration { Duration::from_secs(60 * 60 * 24 * self) }
    fn weeks(self) -> Duration { Duration::from_secs(60 * 60 * 24 * 7 * self) }
    fn months(self) -> Duration { Duration::from_secs(60 * 60 * 24 * 30 * self) }
    fn years(self) -> Duration { Duration::from_secs(60 * 60 * 24 * 365 * self) }
}

impl DurationHelper for f64 {
    fn nanos(self) -> Duration { Duration::new(0, (self) as u32) }
    fn micros(self) -> Duration { Duration::new(0, (self * 1_000.0) as u32) }
    fn millis(self) -> Duration { Duration::new(0, (self * 1_000_000.0) as u32) }
    fn secs(self) -> Duration { Duration::from_secs_f64(self) }
    fn hours(self) -> Duration { Duration::from_secs_f64(60.0 * 60.0 * self) }
    fn days(self) -> Duration { Duration::from_secs_f64(60.0 * 60.0 * 24.0 * self) }
    fn weeks(self) -> Duration { Duration::from_secs_f64(60.0 * 60.0 * 24.0 * 7.0 * self) }
    fn months(self) -> Duration { Duration::from_secs_f64(60.0 * 60.0 * 24.0 * 30.0 * self) }
    fn years(self) -> Duration { Duration::from_secs_f64(60.0 * 60.0 * 24.0 * 365.0 * self) }
}


#[cfg(test)]
mod tests {
    use super::DurationHelper;
    use std::time::Duration;

    #[test]
    fn test_nanos() {
        assert_eq!(0.5.nanos(), Duration::from_nanos(0));  // f64: It's less than 1 nanosecond.
        assert_eq!(3.5.nanos(), Duration::from_nanos(3)); 
        assert_eq!(5.nanos(), Duration::from_nanos(5)); 
    }

    #[test]
    fn test_micros() {
        assert_eq!(0.5.micros(), Duration::from_nanos(500));
        assert_eq!(5.micros(), Duration::from_micros(5));
    }

    #[test]
    fn test_millis() {
        assert_eq!(0.5.millis(), Duration::from_micros(500));
        assert_eq!(5.millis(), Duration::from_millis(5));
    }

    #[test]
    fn test_secs() {
        assert_eq!(0.5.secs(), Duration::from_millis(500));
        assert_eq!(5.secs(), Duration::from_secs(5));
    }

    #[test]
    fn test_hours() {
        assert_eq!(0.5.hours(), Duration::from_secs(30 * 60));
        assert_eq!(5.hours(), Duration::from_secs(5 * 60 * 60));
    }

    #[test]
    fn test_days() {
        assert_eq!(0.5.days(), Duration::from_secs(12 * 60 * 60));
        assert_eq!(5.days(), Duration::from_secs(5 * 24 * 60 * 60));
    }

    #[test]
    fn test_weeks() {
        assert_eq!(0.5.weeks(), Duration::from_secs(3 * 24 * 60 * 60 + 12 * 60 * 60));
        assert_eq!(5.weeks(), Duration::from_secs(5 * 7 * 24 * 60 * 60));
    }

    #[test]
    fn test_months() {
        assert_eq!(0.5.months(), Duration::from_secs(15 * 24 * 60 * 60));
        assert_eq!(5.months(), Duration::from_secs(5 * 30 * 24 * 60 * 60)); 
    }

    #[test]
    fn test_years() {
        assert_eq!(0.5.years(), Duration::from_secs(182 * 24 * 60 * 60 + 12 * 60 * 60));
        assert_eq!(5.years(), Duration::from_secs(5 * 365 * 24 * 60 * 60));
    }
}
