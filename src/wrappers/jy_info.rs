//! This module holds the definitions of
//! JYInfo related wrappers.

use data_structs::JYInfo;
use jalali_bindings::*;

impl JYInfo {
  /// This function initializes the struct.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// jalali::JYInfo::new();
  ///
  /// ```
  pub fn new() -> Self {
    JYInfo {
      lf: 0,
      y: 0,
      r: 0,
      p: 0,
      rl: 0,
      pl: 0,
      apl: 0,
    }
  }

  /// This function initializes the struct based on the provided year.
  ///
  /// # Arguments
  ///
  /// * `year` - A 32 bit integer representing jalali year.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// jalali::JYInfo::get_jalali_year_info(1371);
  ///
  /// ```
  pub fn get_jalali_year_info(year: i32) -> Self {
    let mut result = Self::new();
    result.y = year;
    unsafe {
      jalali_get_jyear_info(&mut result);
    }
    result
  }
}
