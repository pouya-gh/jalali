use ::{JTM, JTMBuilder};
use std::ffi::CString;

fn assert_jtm(t1: JTM, t2: JTM) {
  assert_eq!(t1.tm_sec, t2.tm_sec);
  assert_eq!(t1.tm_min, t2.tm_min);
  assert_eq!(t1.tm_hour, t2.tm_hour);
  assert_eq!(t1.tm_mday, t2.tm_mday);
  assert_eq!(t1.tm_mon, t2.tm_mon);
  assert_eq!(t1.tm_year, t2.tm_year);
  assert_eq!(t1.tm_wday, t2.tm_wday);
  assert_eq!(t1.tm_yday, t2.tm_yday);
  assert_eq!(t1.tm_isdst, t2.tm_isdst);
  assert_eq!(t1.tm_gmtoff, t2.tm_gmtoff);
  assert_eq!(t1.get_timezone().shrink_to_fit(),
             t2.get_timezone().shrink_to_fit());
}

#[test]
fn it_has_a_valid_timezone_getter() {
  let tm = JTM {
    tm_sec: 11,
    tm_min: 22,
    tm_hour: 33,
    tm_mday: 44,
    tm_mon: 55,
    tm_year: 66,
    tm_wday: 77,
    tm_yday: 88,
    tm_isdst: 1,
    tm_gmtoff: 99,
    tm_zone: CString::new("IRST").unwrap().into_raw(),
  };
  assert_eq!("IRST".to_string(), tm.get_timezone());
}

#[test]
fn it_has_a_valid_builder() {
  let t1 = JTMBuilder::new().set_sec(11).
                             set_min(22).
                             set_hour(33).
                             set_mday(44).
                             set_mon(55).
                             set_year(66).
                             set_wday(77).
                             set_yday(88).
                             set_isdst(1).
                             set_gmtoff(99).
                             set_zone("IRST".to_string()).
                             build();
  let t2 = JTM {
    tm_sec: 11,
    tm_min: 22,
    tm_hour: 33,
    tm_mday: 44,
    tm_mon: 55,
    tm_year: 66,
    tm_wday: 77,
    tm_yday: 88,
    tm_isdst: 1,
    tm_gmtoff: 99,
    tm_zone: CString::new("IRST").unwrap().into_raw(),
  };

  assert_jtm(t1, t2);
}

#[test]
fn it_has_a_valid_from_secs_initializer() {
  let t1 = JTMBuilder::new().set_sec(0).
                             set_min(0).
                             set_hour(20).
                             set_mday(26).
                             set_mon(6).
                             set_year(1371).
                             set_wday(1).
                             set_yday(211).
                             set_isdst(0).
                             set_gmtoff(12600).
                             set_zone("IRST".to_string()).
                             build();
  let t2 = JTM::from_secs(719425800);
  assert_jtm(t1, t2);
}

#[test]
fn it_has_a_valid_from_days_initializer() {
  let t1 = JTMBuilder::new().set_sec(0).
                             set_min(0).
                             set_hour(20).
                             set_mday(26).
                             set_mon(6).
                             set_year(1371).
                             set_wday(1).
                             set_yday(211).
                             set_isdst(0).
                             set_gmtoff(12600).
                             set_zone("IRST".to_string()).
                             build();
  let t2 = JTM::from_days(8326);
  assert_eq!(t1.tm_year, t2.tm_year);
  assert_eq!(t1.tm_mon, t2.tm_mon);
  assert_eq!(t1.tm_mday, t2.tm_mday);
}

#[test]
fn it_has_a_valid_from_secs_to_utc_initializer() {
  let t1 = JTMBuilder::new().set_sec(0).
                             set_min(30).
                             set_hour(16).
                             set_mday(26).
                             set_mon(6).
                             set_year(1371).
                             set_wday(1).
                             set_yday(211).
                             set_isdst(0).
                             set_gmtoff(0).
                             set_zone("UTC".to_string()).
                             build();
  let t2 = JTM::from_secs_to_utc(719425800);
  assert_jtm(t1, t2);
}

#[test]
fn it_has_a_valid_from_formatted_string_initializer() {
  let tmp1 = "26/07/1371 19:59:59".to_string();
  let tmp2 = "%d/%m/%Y %H:%M:%S".to_string();
  let t1 = JTMBuilder::new().set_sec(59).
                             set_min(59).
                             set_hour(19).
                             set_mday(26).
                             set_mon(6).
                             set_year(1371).
                             set_wday(1).
                             set_yday(211).
                             set_isdst(0).
                             set_gmtoff(12600).
                             set_zone("IRST".to_string()).
                             build();
  let t2 = JTM::from_formatted_string(&tmp1, &tmp2).unwrap();
  assert_jtm(t1, t2);
}

#[test]
fn it_can_create_month_and_day_from_yearday() {
  let t1 = JTMBuilder::new().set_mday(26).set_mon(6).set_yday(211).build();
  let mut t2 = JTMBuilder::new().set_yday(211).build();
  t2.create_date_from_days().unwrap();
  assert_jtm(t1, t2);
}

#[test]
fn it_can_create_yearday_from_month_and_day() {
  let t1 = JTMBuilder::new().set_mday(26).set_mon(6).set_yday(211).build();
  let mut t2 = JTMBuilder::new().set_mday(26).set_mon(6).build();
  t2.create_days_from_date().unwrap();
  assert_jtm(t1, t2);
}

#[test]
fn it_has_a_valid_days_since_epoch_calculator() {
  let tmp = JTM::from_secs_to_utc(719425800);
  assert_eq!(tmp.get_days_since_epoch(), 8326);
}

#[test]
fn it_has_a_valid_update_method() {
  let t1 = JTMBuilder::new().set_sec(59).
                             set_min(59).
                             set_hour(19).
                             set_mday(26).
                             set_mon(6).
                             set_year(1371).
                             set_wday(1).
                             set_yday(211).
                             set_isdst(0).
                             set_gmtoff(12600).
                             set_zone("IRST".to_string()).
                             build();
  let mut t2 = JTMBuilder::new().set_sec(59).
                             set_min(59).
                             set_hour(19).
                             set_mday(26).
                             set_mon(6).
                             set_year(1371).
                             set_isdst(0).
                             set_gmtoff(12600).
                             set_zone("IRST".to_string()).
                             build();
  t2.update();
  assert_jtm(t1, t2);
}

#[test]
fn it_has_a_valid_to_string_method() {
  let t = JTMBuilder::new().set_sec(59).
                            set_min(59).
                            set_hour(19).
                            set_mday(26).
                            set_mon(6).
                            set_year(1371).
                            set_wday(1).
                            set_yday(211).
                            set_isdst(0).
                            set_gmtoff(12600).
                            set_zone("IRST".to_string()).
                            build();

  assert_eq!(t.to_string(), "Yek Meh 26 19:59:59 1371\n".to_string());
}

#[test]
fn it_has_a_valid_to_secs_method() {
  let mut tmp = JTMBuilder::new().set_sec(0).
                                  set_min(0).
                                  set_hour(20).
                                  set_mday(26).
                                  set_mon(6).
                                  set_year(1371).
                                  set_wday(1).
                                  set_yday(211).
                                  set_isdst(0).
                                  set_gmtoff(12600).
                                  set_zone("IRST".to_string()).
                                  build();
  assert_eq!(tmp.to_secs(), 719425800);
}

#[test]
fn it_has_a_valid_make_formatted_string_method() {
  let tmp = JTMBuilder::new().set_sec(0).
                              set_min(0).
                              set_hour(20).
                              set_mday(26).
                              set_mon(6).
                              set_year(1371).
                              set_wday(1).
                              set_yday(211).
                              set_isdst(0).
                              set_gmtoff(12600).
                              set_zone("IRST".to_string()).
                              build();
  let f1 = "%H:%M".to_string();
  let f2 = "%Z".to_string();
  let mut buffer1 = String::with_capacity(5);
  let mut buffer2 = String::with_capacity(5);

  tmp.make_formated_string(&f1, &mut buffer1);
  tmp.make_formated_string(&f2, &mut buffer2);

  assert_eq!(buffer1, "20:0".to_string());
  assert_eq!(buffer2, "IRST".to_string());
}
