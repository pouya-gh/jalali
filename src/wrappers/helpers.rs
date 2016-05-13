//! This module holds the definitions of
//! wrappers which do not belong to any
//! data structures.

use std::ffi::CString;
use std::mem;
use jalali_bindings::*;

/// This function calculates whether a given year is leap.
///
/// # Arguments
///
/// * `year` - A 32 bit integer representing the year.
///
/// # Examples
///
/// ```
/// extern crate jalali;
///
/// assert_eq!(jalali::is_year_leap(1370), true);
/// assert_eq!(jalali::is_year_leap(1371), false);
///
/// ```
pub fn is_year_leap(year: i32) -> bool {
  let tmp = unsafe {
    jalali_is_jleap(year)
  };
  if tmp == 0 {
    false
  } else {
    true
  }
}

/// This function returns number of days of a specified month in a specified year.
///
/// # Arguments
///
/// * `year` - A 32 bit integer representing the year.
/// * `month` - A 32 bit integer representing the month (0_11).
///
/// # Examples
///
/// ```
/// extern crate jalali;
///
/// assert_eq!(jalali::number_of_month_days(1371, 0), 31);
/// assert_eq!(jalali::number_of_month_days(1371, 6), 30);
/// assert_eq!(jalali::number_of_month_days(1371, 11), 29);
/// assert_eq!(jalali::number_of_month_days(1370, 11), 30); // leap
///
/// ```
pub fn number_of_month_days(year: i32, month: i32) -> i32 {
  unsafe {
    jalali_year_month_days(year, month)
  }
}

/// This function converts time (specified in seconds elapsed since UTC Epoch)
/// to string.
///
/// # Arguments
///
/// * `time` - A 64 bit integer representing number of seconds passed since UTC Epoch.
///
/// # Examples
///
/// ```
/// extern crate jalali;
///
/// assert_eq!(jalali::time_to_string(719425800), "Yek Meh 26 20:00:00 1371\n".to_string());
///
/// ```
pub fn time_to_string(time: i64) -> String {
  let mut buf = Vec::<u8>::with_capacity(27);
  let result: String;
  unsafe {
    jctime_r(&time, buf.as_mut_ptr() as *mut i8);
    result = CString::from_raw(buf.as_mut_ptr() as *mut i8).into_string().unwrap();
  }
  mem::forget(buf);
  return result;
}
