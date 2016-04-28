use libc::{c_int, c_char, c_long, time_t, size_t};
use data_structs::{JTM};
use jalali_bindings::*;
use std::ffi::CString;
use std::{mem, ptr};

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

  pub fn set_sec(&mut self, sec: i32) -> &mut Self {
    self.tm_sec = sec;
    self
  }

  pub fn set_min(&mut self, min: i32) -> &mut Self {
    self.tm_min = min;
    self
  }

  pub fn set_hour(&mut self, hour: i32) -> &mut Self {
    self.tm_hour = hour;
    self
  }

  pub fn set_mday(&mut self, mday: i32) -> &mut Self {
    self.tm_mday = mday;
    self
  }

  pub fn set_mon(&mut self, mon: i32) -> &mut Self {
    self.tm_mon = mon;
    self
  }

  pub fn set_year(&mut self, year: i32) -> &mut Self {
    self.tm_year = year;
    self
  }

  pub fn set_wday(&mut self, wday: i32) -> &mut Self {
    self.tm_wday = wday;
    self
  }

  pub fn set_yday(&mut self, yday: i32) -> &mut Self {
    self.tm_yday = yday;
    self
  }

  pub fn set_isdst(&mut self, isdst: i32) -> &mut Self {
    self.tm_isdst = isdst;
    self
  }

  pub fn set_gmtoff(&mut self, gmtoff: i64) -> &mut Self {
    self.tm_gmtoff = gmtoff;
    self
  }

  pub fn set_zone(&mut self, zone: String) -> &mut Self {
    self.tm_zone = zone;
    self
  }

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
