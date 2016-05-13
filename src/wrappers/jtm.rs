//! This module holds the definitions of JTMBuilder and
//! JTM related bindings.

use libc::{c_int, time_t, size_t};
use data_structs::{JTM};
use jalali_bindings::*;
use std::ffi::CString;
use std::{mem, ptr};

/// This struct serves as a builder for JTM struct.
///
/// # Examples
///
/// ```
/// extern crate jalali;
///
/// jalali::JTMBuilder::new().set_sec(11).
///                           set_min(22).
///                           set_hour(33).
///                           set_mday(44).
///                           set_mon(55).
///                           set_year(66).
///                           set_wday(77).
///                           set_yday(88).
///                           set_isdst(1).
///                           set_gmtoff(99).
///                           set_zone("IRST".to_string()).
///                           build();
///
/// ```
pub struct JTMBuilder {
  pub tm_sec: i32,
  pub tm_min: i32,
  pub tm_hour: i32,
  pub tm_mday: i32,
  pub tm_mon: i32,
  pub tm_year: i32,
  pub tm_wday: i32,
  pub tm_yday: i32,
  pub tm_isdst: i32,
  pub tm_gmtoff: i64,
  pub tm_zone: String,
}

impl JTMBuilder {
  /// This function initializes a JTMBuilder struct.
  ///
  pub fn new() -> Self {
    JTMBuilder {
      tm_sec: 0,
      tm_min: 0,
      tm_hour: 0,
      tm_mday: 0,
      tm_mon: 0,
      tm_year: 0,
      tm_wday: 0,
      tm_yday: 0,
      tm_isdst: 0,
      tm_gmtoff: 0,
      tm_zone: "".to_string(),
    }
  }

  /// This method sets tm_sec of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `sec` - A 32 bit integer providing the value for *tm_sec*.
  ///
  pub fn set_sec(&mut self, sec: i32) -> &mut Self {
    self.tm_sec = sec;
    self
  }

  /// This method sets tm_min of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `min` - A 32 bit integer providing the value for *tm_min*.
  ///
  pub fn set_min(&mut self, min: i32) -> &mut Self {
    self.tm_min = min;
    self
  }

  /// This method sets tm_hour of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `hour` - A 32 bit integer providing the value for *tm_hour*.
  ///
  pub fn set_hour(&mut self, hour: i32) -> &mut Self {
    self.tm_hour = hour;
    self
  }

  /// This method sets tm_mday of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `mday` - A 32 bit integer providing the value for *tm_mday*.
  ///
  pub fn set_mday(&mut self, mday: i32) -> &mut Self {
    self.tm_mday = mday;
    self
  }

  /// This method sets tm_mon of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `mon` - A 32 bit integer providing the value for *tm_mon*.
  ///
  pub fn set_mon(&mut self, mon: i32) -> &mut Self {
    self.tm_mon = mon;
    self
  }

  /// This method sets tm_year of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `year` - A 32 bit integer providing the value for *tm_year*.
  ///
  pub fn set_year(&mut self, year: i32) -> &mut Self {
    self.tm_year = year;
    self
  }

  /// This method sets tm_wday of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `wday` - A 32 bit integer providing the value for *tm_wday*.
  ///
  pub fn set_wday(&mut self, wday: i32) -> &mut Self {
    self.tm_wday = wday;
    self
  }

  /// This method sets tm_yday of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `yday` - A 32 bit integer providing the value for *tm_yday*.
  ///
  pub fn set_yday(&mut self, yday: i32) -> &mut Self {
    self.tm_yday = yday;
    self
  }

  /// This method sets tm_isdst of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `isdst` - A 32 bit integer providing the value for *tm_isdst*.
  ///
  pub fn set_isdst(&mut self, isdst: i32) -> &mut Self {
    self.tm_isdst = isdst;
    self
  }

  /// This method sets tm_gmtoff of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `gmtoff` - A 64 bit integer providing the value for *tm_gmtoff*.
  ///
  pub fn set_gmtoff(&mut self, gmtoff: i64) -> &mut Self {
    self.tm_gmtoff = gmtoff;
    self
  }

  /// This method sets tm_zone of a JTMBuilder struct.
  ///
  /// # Arguments
  ///
  /// * `zone` - A String providing the value for *tm_zone*.
  ///
  pub fn set_zone(&mut self, zone: String) -> &mut Self {
    self.tm_zone = zone;
    self
  }

  /// This method builds a JTM and returns it.
  ///
  pub fn build(&self) -> JTM {
    JTM {
      tm_sec: self.tm_sec,
      tm_min: self.tm_min,
      tm_hour: self.tm_hour,
      tm_mday: self.tm_mday,
      tm_mon: self.tm_mon,
      tm_year: self.tm_year,
      tm_wday: self.tm_wday,
      tm_yday: self.tm_yday,
      tm_isdst: self.tm_isdst,
      tm_gmtoff: self.tm_gmtoff,
      tm_zone: CString::new(self.tm_zone.clone().into_bytes()).
                                                 unwrap().
                                                 into_raw(),
    }
  }
}

impl JTM {
  /// This function initializes a JTM from seconds passed since Epoch.
  ///
  /// # Arguments
  ///
  /// * `secs` - A 64 bit integer representing seconds passed since Epoch.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// jalali::JTM::from_secs(719425800);
  ///
  /// ```
  pub fn from_secs(secs: i64) -> Self {
    let mut result: Self;
    unsafe {
      result = mem::uninitialized();
      jlocaltime_r(&secs as *const time_t, &mut result);
    }
    result
  }

  /// This function initializes a JTM from days passed since Epoch.
  ///
  /// # Arguments
  ///
  /// * `days` - A 32 bit integer representing days passed since Epoch.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// jalali::JTM::from_days(8326);
  ///
  /// ```
  pub fn from_days(days: i32) -> Self {
    let mut result: Self;
    unsafe {
      result = mem::uninitialized();
      jalali_get_date(days as c_int, &mut result);
    }
    result
  }

  /// This function initializes a JTM from seconds passed since Epoch expressed in Coordinated Universal Time (UTC).
  ///
  /// # Arguments
  ///
  /// * `secs` - A 64 bit integer representing seconds passed since Epoch.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// jalali::JTM::from_days(8326);
  ///
  /// ```
  pub fn from_secs_to_utc(secs: i64) -> Self {
    let mut result: Self;
    unsafe {
      result = mem::uninitialized();
      jgmtime_r(&secs as *const time_t, &mut result);
    }
    result
  }

  /// This function initializes a JTM from a string with a special format.
  ///
  /// # Arguments
  ///
  /// * `s` - A string representing time.
  /// * `format` - A string representing format of *s* (see http://www.nongnu.org/jcal/jstrptime.html).
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let tmp1 = "26/07/1371 19:59:59".to_string();
  /// let tmp2 = "%d/%m/%Y %H:%M:%S".to_string();
  /// jalali::JTM::from_formatted_string(&tmp1, &tmp2).unwrap();
  ///
  /// ```
  pub fn from_formatted_string(s: &String, format: &String) -> Result<Self,()> {
    let mut tmp_s = s.clone();
    let mut tmp_f = format.clone();
    tmp_f.push('\0');
    tmp_s.push('\0');
    let mut result: Self;
    let tmp;
    unsafe {
      result = mem::uninitialized();
      tmp = jstrptime(tmp_s.as_ptr() as *const i8,
                      tmp_f.as_ptr() as *const i8,
                      &mut result);
    }
    result.update();
    if tmp.is_null() {
      Err(())
    } else {
      Ok(result)
    }
  }

  /// This method initializes a JTM from a string with a special format.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let tmp = jalali::JTM::from_days(8326);
  /// assert_eq!(tmp.get_timezone(), "IRST".to_string());
  ///
  /// ```
  pub fn get_timezone(&self) -> String {
    let mut tst = String::with_capacity(5);
    self.make_formated_string(&"%Z".to_string(), &mut tst);
    tst
  }

  /// This method alters tm_mon and tm_mday fields of the broken-down
  /// jalali time strucutre based on it's tm_yday field.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let mut j = jalali::JTMBuilder::new().set_yday(211).build();
  /// j.create_date_from_days().unwrap();
  /// assert_eq!(j.tm_mon, 6);
  /// assert_eq!(j.tm_mday, 26);
  ///
  /// ```
  pub fn create_date_from_days(&mut self) -> Result<(),()> {
    let tmp = unsafe {
      jalali_create_date_from_days(self)
    };
    match tmp {
      -1 => Err(()),
      _ => Ok(()),
    }
  }

  /// This method alters tm_yday field of the broken-down jalali
  /// time structure based on it's tm_mon and tm_mday fields.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let mut j = jalali::JTMBuilder::new().set_mday(26).set_mon(6).build();
  /// j.create_days_from_date().unwrap();
  /// assert_eq!(j.tm_yday, 211);
  ///
  /// ```
  pub fn create_days_from_date(&mut self) -> Result<(),()> {
    let tmp = unsafe {
      jalali_create_days_from_date(self)
    };
    match tmp {
      -1 => Err(()),
      _ => Ok(()),
    }
  }

  /// This method calculates the number of days passed since UTC
  /// Epoch based on a broken-down jalali time structure supplied to it.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let tmp = jalali::JTM::from_secs_to_utc(719425800);
  /// assert_eq!(tmp.get_days_since_epoch(), 8326);
  ///
  /// ```
  pub fn get_days_since_epoch(&self) -> i32 {
    unsafe {
      jalali_get_diff(self)
    }
  }

  /// This method updates tm_wday and tm_yday fields of the broken-down
  /// jalali time structure based on it's tm_year, tm_mon, tm_mday,
  /// tm_hour, tm_min and tm_sec fields. If structure members are outside
  /// their valid interval, they will be normalized (so that, for example,
  /// 40 Bahman is changed into 10 Esfand). tm_isdst, tm_gmtoff and tm_zone
  /// fields are set accordingly.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let mut j = jalali::JTMBuilder::new().set_sec(59).
  ///                                        set_min(59).
  ///                                        set_hour(19).
  ///                                        set_mday(26).
  ///                                        set_mon(6).
  ///                                        set_year(1371).
  ///                                        set_isdst(0).
  ///                                        set_gmtoff(12600).
  ///                                        set_zone("IRST".to_string()).
  ///                                        build();
  /// j.update();
  /// assert_eq!(j.tm_wday, 1);
  /// assert_eq!(j.tm_yday, 211);
  ///
  /// ```
  pub fn update(&mut self) {
    unsafe {
      jalali_update(self);
    }
  }

  /// This method writes a string representation of the struct
  /// to standard output.
  ///
  pub fn show_time(&self) {
    unsafe {
      jalali_show_time(self);
    }
  }

  /// This method returns a string representation of the struct.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let mut j = jalali::JTM::from_secs_to_utc(719425800);
  /// assert_eq!(j.to_string(), "Yek Meh 26 16:30:00 1371\n".to_string());
  ///
  /// ```
  pub fn to_string(&self) -> String {
    let mut buff = Vec::<u8>::with_capacity(100);
    let result: String;
    unsafe {
      jasctime_r(self, buff.as_mut_ptr() as *mut i8);
      result = CString::from_raw(buff.as_mut_ptr() as *mut i8).into_string().unwrap();
    }
    mem::forget(buff);
    return result;
  }

  /// This method converts a broken-down jalali time structure,
  /// expressed as local time, to calendar time representation.
  /// It ignores the values supplied by the caller in the tm_wday field.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let mut j = jalali::JTM::from_secs(719425800);
  /// assert_eq!(j.to_secs(), 719425800);
  ///
  /// ```
  pub fn to_secs(&mut self) -> i64 {
    unsafe {
      jmktime(self)
    }
  }

  /// This method returns a string representation of the struct
  /// based on a supplied *format*.
  /// It returns the number of characters placed in *buff*.
  /// If *buff* is not long enough, it returns max.
  ///
  /// # Arguments
  ///
  /// * `format` - A string representing format of *buff* (see http://www.nongnu.org/jcal/jstrftime.html).
  /// * `buff` - A string serving as a buffer. It must be long enough to hold the resulting string.
  ///
  /// # Examples
  ///
  /// ```
  /// extern crate jalali;
  ///
  /// let f = "%H:%M".to_string();
  /// let mut buffer = String::with_capacity(5);
  /// let mut j = jalali::JTM::from_secs(719425800);
  /// j.make_formated_string(&f, &mut buffer);
  /// assert_eq!(buffer, "20:0".to_string());
  ///
  /// ```
  pub fn make_formated_string(&self, format: &String, buff: &mut String) -> usize {
    let mut b = String::with_capacity(buff.capacity()).into_bytes();
    let retval: usize;
    unsafe {
      let f = format.as_ptr() as *const i8;
      let max = b.capacity() as size_t;
      retval = jstrftime(b.as_mut_ptr() as *mut i8, max, f, self);
      let tmp = CString::from_raw(b.as_mut_ptr() as *mut i8).into_string().unwrap();
      ptr::copy_nonoverlapping(&tmp, buff, 1);
      mem::forget(tmp);
    }
    mem::forget(b);
    return retval;
  }
}
