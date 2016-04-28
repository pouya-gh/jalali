use libc::{c_int, c_char, c_long, time_t, size_t};
use super::data_structs::*;

#[link(name = "jalali", kind = "static")]
extern {
  // jalali.h
  fn jalali_is_jleap(year: c_int) -> c_int;
  fn jalali_create_time_from_secs(time: time_t, ab_jtm: *mut AB_JTM);
  fn jalali_create_secs_from_time(ab_jtm: *const AB_JTM) -> time_t;
  fn jalali_create_date_from_days(j: *mut JTM) -> c_int;
  fn jalali_create_days_from_date(j: *mut JTM) -> c_int;
  fn jalali_get_jyear_info(year_info: *mut JYInfo);
  fn jalali_get_date(p: c_int, jtm: *mut JTM);
  fn jalali_get_diff(jtm: *const JTM) -> c_int;
  fn jalali_update(jtm: *mut JTM);
  fn jalali_show_time(jtm: *const JTM);
  fn jalali_year_month_days(year: c_int, month: c_int) -> c_int;

  // jtime.h
  fn jasctime(jtm: *const JTM) -> *mut c_char;
  fn jctime(timep: *const time_t) -> *mut c_char;
  fn jgmtime(timep: *const time_t) -> *mut JTM;
  fn jlocaltime(timep: *const time_t) -> &mut JTM;
  fn jmktime(jtm: *mut JTM) -> time_t;
  fn jstrftime(s: *mut c_char, max: size_t,format: *const c_char, jtm: *const JTM) -> size_t;
  fn jstrptime(s: *const c_char, format: *const c_char, jtm: *mut JTM) -> *mut c_char;
  fn jasctime_r(jtm: *const JTM, buf: *mut c_char) -> *mut c_char;
  fn jctime_r(timep: *const time_t, buf: *mut c_char) -> *mut c_char;
  fn jgmtime_r(timep: *const time_t, result: *mut JTM) -> *mut JTM;
  fn jlocaltime_r(timep: *const time_t, result: *mut JTM) -> *mut JTM;
  fn jalali_to_farsi(buf: *mut c_char, n: size_t, padding: c_int, pad: *mut c_char, d: c_int) -> c_int;
}
