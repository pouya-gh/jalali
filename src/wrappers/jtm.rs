use libc::{c_int, time_t, size_t};
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

impl JTM {
  pub fn from_secs(secs: i64) -> Self {
    let mut result: Self;
    unsafe {
      result = mem::uninitialized();
      jlocaltime_r(&secs as *const time_t, &mut result);
    }
    result
  }

  pub fn from_days(days: i32) -> Self {
    let mut result: Self;
    unsafe {
      result = mem::uninitialized();
      jalali_get_date(days as c_int, &mut result);
    }
    result
  }

  pub fn from_secs_to_utc(secs: i64) -> Self {
    let mut result: Self;
    unsafe {
      result = mem::uninitialized();
      jgmtime_r(&secs as *const time_t, &mut result);
    }
    result
  }

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

  pub fn get_timezone(&self) -> String {
    let mut tst = String::with_capacity(5);
    self.make_formated_string(&"%Z".to_string(), &mut tst);
    tst
  }

  pub fn create_date_from_days(&mut self) -> Result<(),()> {
    let tmp = unsafe {
      jalali_create_date_from_days(self)
    };
    match tmp {
      -1 => Err(()),
      _ => Ok(()),
    }
  }

  pub fn create_days_from_date(&mut self) -> Result<(),()> {
    let tmp = unsafe {
      jalali_create_days_from_date(self)
    };
    match tmp {
      -1 => Err(()),
      _ => Ok(()),
    }
  }

  pub fn get_days_since_epoch(&self) -> i32 {
    unsafe {
      jalali_get_diff(self)
    }
  }

  pub fn update(&mut self) {
    unsafe {
      jalali_update(self);
    }
  }

  pub fn show_time(&self) {
    unsafe {
      jalali_show_time(self);
    }
  }

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

  pub fn to_secs(&mut self) -> i64 {
    unsafe {
      jmktime(self)
    }
  }

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
