use ::AB_JTM;

fn assert_ab_jtm(t1: AB_JTM, t2: AB_JTM) {
  assert_eq!(t1.ab_sec, t2.ab_sec);
  assert_eq!(t1.ab_min, t2.ab_min);
  assert_eq!(t1.ab_hour, t2.ab_hour);
  assert_eq!(t1.ab_days, t2.ab_days);
}

#[test]
fn it_has_a_valid_initializer() {
  let t1 = AB_JTM::new();
  let t2 = AB_JTM {
                    ab_sec: 0,
                    ab_min: 0,
                    ab_hour: 0,
                    ab_days: 0,
                  };
  assert_ab_jtm(t1, t2);
}

#[test]
fn it_has_a_valid_from_secs_initializer() {
  let t1 = AB_JTM {
                    ab_sec: 0,
                    ab_min: 30,
                    ab_hour: 16,
                    ab_days: 8326,
                  };
  let t2 = AB_JTM::from_secs(719425800);
  assert_ab_jtm(t1, t2);
}

#[test]
fn it_has_a_valid_to_secs_converter() {
  let t1 = 719425800;
  assert_eq!(t1, AB_JTM::from_secs(t1).to_secs());
}
