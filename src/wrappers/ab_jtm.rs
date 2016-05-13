//! This module holds the definitions of
//! AB_JTM related wrappers.

use libc::time_t;
use data_structs::AB_JTM;
use jalali_bindings::*;
use std::mem;

impl AB_JTM {
  /// This function initializes the struct.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// jalali::AB_JTM::new();
  ///
  /// ```
  pub fn new() -> Self {
    AB_JTM {
      ab_sec: 0,
      ab_min: 0,
      ab_hour: 0,
      ab_days: 0,
    }
  }

  /// This function initializes the struct based on the number of seconds passed since UTC Epoch.
  ///
  /// # Arguments
  ///
  /// * `secs` - A 64 bit integer representing number of seconds passed since UTC Epoch.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// jalali::AB_JTM::from_secs(719425800);
  ///
  /// ```
  pub fn from_secs(secs: i64) -> Self {
    let mut result;
    unsafe {
      result = mem::uninitialized();
      jalali_create_time_from_secs(secs as time_t, &mut result);
    }
    result
  }

  /// This function initializes the struct based on the number of seconds passed since UTC Epoch.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let j = jalali::AB_JTM::from_secs(719425800);
  ///
  /// assert_eq!(j.to_secs(), 719425800);
  /// ```
  pub fn to_secs(&self) -> i64 {
    unsafe {
      jalali_create_secs_from_time(self)
    }
  }
}
