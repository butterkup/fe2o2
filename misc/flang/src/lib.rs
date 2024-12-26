#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShirtColor {
  Red,
  Blue,
}

pub use ShirtColor::*;

pub struct Inventory {
  shirts: Vec<ShirtColor>,
}

#[macro_export]
macro_rules! inventory {
  [$($shirt:expr),*] => {
    {
      let mut inv = flang::Inventory::new();
      $(
        inv.push($shirt);
      )*
      inv
    }
  }
}

impl Inventory {
  pub fn new() -> Inventory { Inventory { shirts: Vec::new() } }
  pub fn push(&mut self, shirt: ShirtColor) { self.shirts.push(shirt); }
  pub fn giveaway(&self, usrpref: Option<ShirtColor>) -> ShirtColor {
    usrpref.unwrap_or_else(|| {
      let mut red_shirts = 0;
      let mut blue_shirts = 0;
      for shirt in &self.shirts {
        match shirt {
          ShirtColor::Red => red_shirts += 1,
          ShirtColor::Blue => blue_shirts += 1,
        }
      }
      if red_shirts > blue_shirts {
        ShirtColor::Red
      } else {
        ShirtColor::Blue
      }
    })
  }
}
