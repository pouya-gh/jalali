use ::{is_year_leap, number_of_month_days, time_to_string};

#[test]
fn it_has_a_valid_leap_year_indicator() {
  assert!(is_year_leap(1370));
  assert!(! is_year_leap(1371));
}

#[test]
fn it_has_a_valid_number_of_month_days_indicator() {
  assert_eq!(number_of_month_days(1370, 11), 30);
  assert_eq!(number_of_month_days(1371, 11), 29);
}

#[test]
fn it_has_a_valid_time_to_string_converter() {
  assert_eq!(time_to_string(719425800), "Yek Meh 26 20:00:00 1371\n".to_string());
}
