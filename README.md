# Duration Extensions

A tiny Rust utility to provide a more concise way to define `std::time::Duration`

## Usage

```rust
use duration_helper::DurationHelper;

let half_a_second = 0.5.secs();
let five_hours = 5.hours();
let day_and_a_half = 1.5.days();
```

## Features

- Extensions for:
  - Nanoseconds: `nanos()`
  - Microseconds: `micros()`
  - Milliseconds: `millis()`
  - Seconds: `secs()`
  - Hours: `hours()`
  - Days: `days()`
  - Weeks: `weeks()`
  - Months (approximated as 30 days): `months()`
  - Years (approximated as 365 days): `years()`

  