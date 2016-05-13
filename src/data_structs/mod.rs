//! This module holds definitions of data structures used in this library.

use libc::{c_int, c_char, c_long};

/// Broken-down jalali time
///
#[derive(Debug)]
#[repr(C)]
pub struct JTM {
  /// Seconds. (0-59)
  pub tm_sec: c_int,
  /// Minutes. (0-59)
  pub tm_min: c_int,
  /// Hours. (0-59)
  pub tm_hour: c_int,
  /// Day of the month. (1-31)
  pub tm_mday: c_int,
  /// Month. (0-11)
  pub tm_mon: c_int,
  /// Year.
  pub tm_year: c_int,
  /// Day of the week. (0-6)
  pub tm_wday: c_int,
  /// Day in the year. (0-365)
  pub tm_yday: c_int,
  /// Daylight saving time is in effect.
  pub tm_isdst: c_int,
  /// Seconds east of UTC.
  pub tm_gmtoff: c_long,
  /// Timezone abbreviation.
  pub tm_zone: *const c_char,
}

/// Information about passed days since Epoch
#[derive(Debug)]
#[repr(C)]
pub struct AB_JTM {
  pub ab_sec: c_int,
  pub ab_min: c_int,
  pub ab_hour: c_int,
  pub ab_days: c_int,
}

/// Information about a certain year in jalali system
///
#[derive(Debug)]
#[repr(C)]
pub struct JYInfo {
  /// leap indicator flag
  pub lf: c_int,
  /// year
  pub y: c_int,
  /// reamining years in grand cycle
  pub r: c_int,
  /// passed years from grand cycle
  pub p: c_int,
  /// remaining leap years in grand cycle
  pub rl: c_int,
  /// passed leap years in grand cycle
  pub pl: c_int,
  /// absolute passed leaps
  pub apl: c_int,
}
