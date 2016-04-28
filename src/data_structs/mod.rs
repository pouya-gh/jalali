use libc::{c_int, c_char, c_long, time_t, size_t};
use std::fmt::Debug;

#[derive(Debug)]
#[repr(C)]
pub struct JTM {
  pub tm_sec: c_int,       /* Seconds. (0-59) */
  pub tm_min: c_int,       /* Minutes. (0-59) */
  pub tm_hour: c_int,      /* Hours. (0-59) */
  pub tm_mday: c_int,      /* Day of the month. (1-31) */
  pub tm_mon: c_int,       /* Month. (0-11) */
  pub tm_year: c_int,      /* Year. */
  pub tm_wday: c_int,      /* Day of the week. (0-6) */
  pub tm_yday: c_int,      /* Day in the year. (0-365) */
  pub tm_isdst: c_int,     /* Daylight saving time is in effect. */
  pub tm_gmtoff: c_long,   /* Seconds east of UTC. */
  tm_zone: *const c_char,  /* Timezone abbreviation.  */
}

#[derive(Debug)]
#[repr(C)]
pub struct AB_JTM {
  pub ab_sec: c_int,
  pub ab_min: c_int,
  pub ab_hour: c_int,
  pub ab_days: c_int,
}

#[derive(Debug)]
#[repr(C)]
pub struct JYInfo {
  pub lf: c_int,          /* leap indicator flag */
  pub y: c_int,           /* year */
  pub r: c_int,           /* reamining years in grand cycle */
  pub p: c_int,           /* passed years from grand cycle*/
  pub rl: c_int,          /* remaining leap years in grand cycle */
  pub pl: c_int,          /* passed leap years in grand cycle */
  pub apl: c_int,         /* absolute passed leaps */
}
