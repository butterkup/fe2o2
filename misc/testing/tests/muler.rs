#[test]
fn mul5t6() {
  assert_eq!(testing::mul(5, 6), 30);
}
#[test]
fn mul7t8() {
  assert_eq!(testing::mul(7, 8), 56);
}
#[test]
fn mul9t10() {
  assert_eq!(testing::mul(9, 10), 90);
}
#[test]
fn mul5t10() {
  assert_eq!(testing::mul(5, 10), 50);
}

mod adder;
mod lola;

#[test]
fn get_lola() {
  lola::lola_for_mul();
  adder::mylola();
}
