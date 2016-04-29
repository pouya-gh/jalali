use libc::time_t;
use data_structs::AB_JTM;
use jalali_bindings::*;
use std::mem;

impl AB_JTM {
  pub fn new() -> Self {
    AB_JTM {
      ab_sec: 0,
      ab_min: 0,
      ab_hour: 0,
      ab_days: 0,
    }
  }

  pub fn from_secs(secs: i64) -> Self {
    let mut result;
    unsafe {
      result = mem::uninitialized();
      jalali_create_time_from_secs(secs as time_t, &mut result);
    }
    result
  }

  pub fn to_secs(&self) -> i64 {
    unsafe {
      jalali_create_secs_from_time(self)
    }
  }
}
