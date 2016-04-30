use ::{JYInfo};

fn assert_jy_info(t1: JYInfo, t2: JYInfo) {
  assert_eq!(t1.lf, t2.lf);  
  assert_eq!(t1.y, t2.y);  
  assert_eq!(t1.r, t2.r);  
  assert_eq!(t1.p, t2.p);  
  assert_eq!(t1.rl, t2.rl);  
  assert_eq!(t1.pl, t2.pl);  
  assert_eq!(t1.apl, t2.apl);  
}

#[test]
fn it_has_a_valid_initializer() {
  let t1 = JYInfo { lf: 0, y: 0, r: 0, p: 0, rl: 0, pl: 0, apl: 0 };
  let t2 = JYInfo::new();
  assert_jy_info(t1, t2);
}

#[test]
fn it_outputs_year_info_correctly() {
  let t1 = JYInfo { lf: 0, y: 1371, r: 1923, p: 896, rl: 466, pl: 217, apl: 217 };
  let t2 = JYInfo::get_jalali_year_info(1371);
  assert_jy_info(t1, t2);
}
