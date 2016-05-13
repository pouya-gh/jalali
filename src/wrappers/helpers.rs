use std::ffi::CString;
use std::mem;
use jalali_bindings::*;

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

pub fn number_of_month_days(year: i32, month: i32) -> i32 {
  unsafe {
    jalali_year_month_days(year, month)
  }
}

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
