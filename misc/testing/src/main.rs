#![feature(cmp_minmax)]
#![allow(unused)]

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Rect {
  width: i32,
  height: i32,
}

impl Rect {
  fn new(width: i32, height: i32) -> Self {
    if width < 0 {
      panic!("Expected non negative width, but got width={width}");
    }
    if height < 0 {
      panic!("Expected non negative height, but got height={height}");
    }
    Self { width, height }
  }
  fn can_hold(&self, rect: Rect) -> bool { *self >= rect }
  fn scale(&self, scalar: f32) -> Rect {
    Rect {
      width: (self.width as f32 * scalar) as i32,
      height: (self.height as f32 * scalar) as i32,
    }
  }
  fn area(&self) -> i32 { self.width * self.height }
  fn rotate(&self) -> Rect {
    Rect {
      width: self.height,
      height: self.width,
    }
  }
  fn small(&self, rect: Self) -> Self {
    Self {
      width: std::cmp::min(self.width, rect.width),
      height: std::cmp::min(self.height, rect.height),
    }
  }
  fn big(&self, rect: Self) -> Self {
    Self {
      width: std::cmp::max(self.width, rect.width),
      height: std::cmp::max(self.height, rect.height),
    }
  }
  fn smallbig(&self, rect: Self) -> (Self, Self) {
    let [smallw, bigw] = std::cmp::minmax(self.width, rect.width);
    let [smallh, bigh] = std::cmp::minmax(self.height, rect.height);
    (
      Self {
        width: smallw,
        height: smallw,
      },
      Self {
        width: bigw,
        height: bigh,
      },
    )
  }
}

//impl std::cmp::PartialEq for Rect {
//  fn eq(&self, other: &Self) -> bool {
//    self.width == other.width && self.height == other.height
//  }
//}

impl std::cmp::PartialOrd for Rect {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    if self.width == other.width && self.height == other.height {
      Some(std::cmp::Ordering::Equal)
    } else if self.width <= other.width && self.height <= other.height {
      Some(std::cmp::Ordering::Less)
    } else if self.width >= other.width && self.height >= other.height {
      Some(std::cmp::Ordering::Greater)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Rect;

  #[test]
  fn rect_can_hold() {
    let large = super::Rect {
      width: 60,
      height: 80,
    };
    let small = super::Rect {
      width: 20,
      height: 50,
    };
    assert!(large.can_hold(small));
    assert!(large.can_hold(large));
    assert!(small.can_hold(small));
    assert!(!small.can_hold(large));
  }
  #[test]
  fn rect_area() {
    let rect = super::Rect {
      width: 60,
      height: 80,
    };
    assert_eq!(rect.area(), rect.width * rect.height);
  }
  #[test]
  fn rect_rotate() {
    let rect = super::Rect {
      width: 60,
      height: 80,
    };
    assert_eq!(rect, rect.rotate().rotate());
  }
  #[test]
  fn rect_scale() {
    let rect = super::Rect {
      width: 60,
      height: 80,
    };
    assert_eq!(rect.scale(1.5).scale(2.0 / 3.0), rect);
  }
  #[test]
  #[should_panic(expected = "non negative width")]
  fn rect_new_bad_width() { Rect::new(-100, 30); }
  #[test]
  #[should_panic(expected = "non negative height")]
  fn rect_new_bad_height() { Rect::new(100, -20); }
  #[test]
  fn rect_nonholdable() {
    let rect1 = Rect::new(100, 50);
    let rect2 = Rect::new(50, 100);
    assert!(!rect1.can_hold(rect2));
    assert!(!rect2.can_hold(rect1));
  }
  #[test]
  fn rect_smallbig() {
    let rect1 = Rect::new(100, 50);
    let rect2 = Rect::new(50, 100);
    let small = rect1.small(rect2);
    let big = rect1.big(rect2);
    let smallbig = rect1.smallbig(rect2);
    assert_eq!((small, big), smallbig);
    assert!(!rect1.can_hold(big));
    assert!(!rect2.can_hold(big));
    assert!(!small.can_hold(rect1));
    assert!(!small.can_hold(rect2));
    assert!(rect1.can_hold(small));
    assert!(rect2.can_hold(small));
    assert!(big.can_hold(rect1));
    assert!(big.can_hold(rect2));
    assert!(big.can_hold(small));
    assert!(!small.can_hold(big));
  }
}

fn plus2(i: i32) -> i32 { i + 2 }

/// Subtract 3 from input integer
/// ```
/// #[test]
/// fn equals_3() {
///   assert_eq!(minus3(5), 2));
/// }
/// ```
fn minus3(i: i32) -> i32 { i - 3 }

#[test]
#[ignore]
fn minus3_test() {
  assert_eq!(minus3(5), 2);
  assert!(minus3(10) == 7);
}

#[test]
#[ignore]
fn plus2_test() {
  assert_eq!(5, plus2(3));
  assert!(10 == plus2(8));
  assert!(60 == plus2(58));
}

fn main() {
  println!("7 + 2 = {}", plus2(7));
  println!("7 - 3 = {}", minus3(7));
}
