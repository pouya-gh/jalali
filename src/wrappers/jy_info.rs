use data_structs::JYInfo;
use jalali_bindings::*;

impl JYInfo {
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

  pub fn get_jalali_year_info(year: i32) -> Self {
    let mut result = Self::new();
    result.y = year;
    unsafe {
      jalali_get_jyear_info(&mut result);
    }
    result
  }
}
