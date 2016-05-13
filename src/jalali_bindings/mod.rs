use libc::{c_int, c_char, time_t, size_t};
use super::data_structs::*;

#[link(name = "jalali", kind = "static")]
extern {
  // jalali.h
  pub fn jalali_is_jleap(year: c_int) -> c_int;
  pub fn jalali_create_time_from_secs(time: time_t, ab_jtm: *mut AB_JTM);
  pub fn jalali_create_secs_from_time(ab_jtm: *const AB_JTM) -> time_t;
  pub fn jalali_create_date_from_days(j: *mut JTM) -> c_int;
  pub fn jalali_create_days_from_date(j: *mut JTM) -> c_int;
  pub fn jalali_get_jyear_info(year_info: *mut JYInfo);
  pub fn jalali_get_date(p: c_int, jtm: *mut JTM);
  pub fn jalali_get_diff(jtm: *const JTM) -> c_int;
  pub fn jalali_update(jtm: *mut JTM);
  pub fn jalali_show_time(jtm: *const JTM);
  pub fn jalali_year_month_days(year: c_int, month: c_int) -> c_int;

  // jtime.h
  pub fn jasctime(jtm: *const JTM) -> *mut c_char;
  pub fn jctime(timep: *const time_t) -> *mut c_char;
  pub fn jgmtime(timep: *const time_t) -> *mut JTM;
  pub fn jlocaltime(timep: *const time_t) -> &mut JTM;
  pub fn jmktime(jtm: *mut JTM) -> time_t;
  pub fn jstrftime(s: *mut c_char, max: size_t,format: *const c_char, jtm: *const JTM) -> size_t;
  pub fn jstrptime(s: *const c_char, format: *const c_char, jtm: *mut JTM) -> *mut c_char;
  pub fn jasctime_r(jtm: *const JTM, buf: *mut c_char) -> *mut c_char;
  pub fn jctime_r(timep: *const time_t, buf: *mut c_char) -> *mut c_char;
  pub fn jgmtime_r(timep: *const time_t, result: *mut JTM) -> *mut JTM;
  pub fn jlocaltime_r(timep: *const time_t, result: *mut JTM) -> *mut JTM;
  pub fn jalali_to_farsi(buf: *mut c_char, n: size_t, padding: c_int, pad: *mut c_char, d: c_int) -> c_int;
}
