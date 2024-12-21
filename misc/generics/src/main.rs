#![feature(random)]
#![allow(unused)]

use generics::{Summarizable, Summary};

fn rand_vec(mut count: usize) -> Vec<i32> {
  let mut cont = Vec::with_capacity(count);
  while count > 0 {
    cont.push(std::random::random::<u8>() as i32);
    count -= 1;
  }
  cont
}

fn refs() {
  let mut a = 90;
  let mut b = 100;
  println!("a={a} b={b}");
  let mut y = &mut a;
  *y = 54;
  y = &mut b;
  *y = 23;
  println!("a={a} b={b}");
}

fn get_largest<T: std::cmp::PartialOrd>(seq: &[T]) -> Option<&T> {
  let mut largest = seq.first()?;
  for value in seq {
    largest = if value > largest { value } else { largest };
  }
  Some(largest)
}
//fn get_largest_i32(seq: &[i32]) -> Option<&i32> {
//  let mut largest = seq.first()?;
//  for value in seq {
//    largest = if value > largest { value } else { largest };
//  }
//  Some(largest)
//}
//
//fn get_largest_char(seq: &[char]) -> Option<&char> {
//  let mut largest = seq.first()?;
//  for value in seq {
//    largest = if value > largest { value } else { largest };
//  }
//  Some(largest)
//}

#[derive(Default, Clone, Copy)]
struct Point<T> {
  x: T,
  y: T,
}

impl<T: Clone + Copy> Point<T> {
  fn dot<OtherT: Clone>(&self, other: &Point<OtherT>) -> T
  where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T>,
    OtherT: Into<T>,
  {
    (self.x - other.x.clone().into()) * (self.y - other.y.clone().into())
  }
}

struct Own<'a> {
  val: &'a i32,
}

impl<'a> Own<'a> {
  fn new(val: &'a i32) -> Own<'a> { Own { val } }
  fn meth<'b: 'a>(self: &mut Self, val: &'b i32) { self.val = val; }
  fn read(&self) -> i32 { *self.val }
}

fn main() {
  let function = generics::get_summary::<Summarizable>;
  let function1: for<'a> fn(&'a Summarizable) -> String = generics::summary;
  let creator: fn() -> _ = generics::create_summary;

  dbg!(creator().summarize());
  dbg!(generics::get_summary(&creator()));
  dbg!(generics::summary(&creator()));

  let add3to4 = generics::BinOp {
    arg1: 3,
    arg2: 4,
    op: |a, b| a + b,
  };
  dbg!(add3to4.call());

  let stories = vec![
    generics::Name("Matthew Davidson".into()),
    generics::Email("davidson@matinc.com".into()),
    generics::Name("John Doe".into()),
    generics::Email("doe.john@igotmoney.com".into()),
  ];
  stories
    .iter()
    .map(generics::summary)
    .for_each(|s| println!("Another: {s}"));
  stories
    .iter()
    .for_each(|s| println!("Headline: {}", s.summarize()));
  stories
    .iter()
    .map(generics::get_summary)
    .for_each(|s| println!("Another: {s}"));
  return;
  let one = Point { x: 20, y: 30 };
  let two = Point { x: 10.54, y: 40.78 };
  //one.dot(&two);
  two.dot(&one);
  refs();
  let numbers = rand_vec(10);
  println!("Numbers: {:?}", numbers);
  let largest = get_largest(numbers.as_slice()).expect("Sequence is empty");
  //let largest = get_largest_i32(numbers.as_slice()).expect("Sequence is empty");
  //let mut largest = &numbers[0];
  //for number in &numbers {
  //  largest = if number > largest { number } else { largest };
  //}
  println!("Largest number is {largest:?}.");

  let characters: Vec<char> = "My name is Simon Nganga!".chars().collect();
  println!("Characters: {characters:?}");
  let largest = get_largest(&characters).expect("Sequence is empty");
  //let largest = get_largest_char(&characters).expect("Sequence is empty");
  println!("Largest char is {largest:?}.");
}
