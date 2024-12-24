
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

pub fn mul(a: i32, b: i32) -> i32 {
  a * b
}

#[cfg(test)]
mod tests {
  #[test]
  fn add_test() {
    assert_eq!(super::add(3, 4), 7);
  }

  #[test]
  fn mul_test() {
    assert_eq!(super::mul(9, 3), 27);
  }
}

