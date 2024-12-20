#![feature(random)]
#![allow(unused)]

macro_rules! mvec {
  ($type:ty) => {
    Vec::<$type>::new()
  };
  [$($value:expr),*] => {
    {
      let mut tmp = Vec::new();
      $(
        tmp.push($value);
      )*
      tmp
    }
  };
}

#[derive(Debug)]
enum Thing {
  Utensil,
  Bed,
  Rust,
  Cpp,
  Python,
  Java,
}

impl std::fmt::Display for Thing {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Self::Utensil => "Thing::Utensil",
      Self::Bed => "Thing::Bed",
      Self::Python => "Thing::Python",
      Self::Rust => "Thing::Rust",
      Self::Java => "Thing::Java",
      Self::Cpp => "Thing::Cpp",
    }
    .fmt(f)
  }
}

use std::{collections::HashMap, fmt::Write, iter::FlatMap};

macro_rules! myformat {
  ($($args:expr),+) => {
    {
      let mut buffer = String::new();
      write!(&mut buffer, $( $args ),*);
      buffer
    }
  }
}

macro_rules! map {
  [$(($key:expr,$value:expr)),+] => {
    {
      let mut map = HashMap::new();
      $(
        map.insert($key, $value);
      )*
      map
    }
  }
}

struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
  fn new(data: T) -> Node<T> { Node { data, next: None } }
  fn with_next(data: T, next: Node<T>) -> Node<T> {
    Node {
      data,
      next: Some(Box::new(next)),
    }
  }
}

struct ForwardList<T> {
  len: usize,
  head: Option<Box<Node<T>>>,
}

impl<T> ForwardList<T> {
  fn push_data(&mut self, data: T) {
    let mut node = Box::new(Node::new(data));
    std::mem::swap(&mut node.next, &mut self.head);
    self.head = Some(node);
    self.len += 1;
  }

  fn pop_data(&mut self) -> Option<T> {
    let mut node = std::mem::take(&mut self.head);
    match node {
      Some(mut bx) => {
        self.head = std::mem::replace(&mut bx.next, None);
        self.len -= 1;
        Some(bx.data)
      }
      None => None,
    }
  }

  fn new() -> ForwardList<T> { ForwardList { head: None, len: 0 } }
  fn iter_mut<'a>(&'a mut self) -> ForwardListMutIterator<'a, T> {
    ForwardListMutIterator {
      head: if let Some(bx) = &mut self.head {
        Some(bx)
      } else {
        None
      },
    }
  }
  fn iter<'a>(&'a self) -> ForwardListIterator<'a, T> {
    ForwardListIterator { head: &self.head }
  }
}

impl<A> FromIterator<A> for ForwardList<A> {
  fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
    let mut list = ForwardList::new();
    for val in iter {
      list.push_data(val);
    }
    list
  }
}

struct ForwardListMutIterator<'a, T> {
  head: Option<&'a mut Box<Node<T>>>,
}

impl<'a, T> Iterator for ForwardListMutIterator<'a, T> {
  type Item = &'a mut T;
  fn next(&mut self) -> Option<Self::Item> {
    match std::mem::replace(&mut self.head, None) {
      Some(bx) => {
        self.head = if let Some(bx) = &mut bx.next {
          Some(bx)
        } else {
          None
        };
        Some(&mut bx.data)
      }
      None => None,
    }
  }
}

struct ForwardListIterator<'a, T> {
  head: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for ForwardListIterator<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    match std::mem::replace(&mut self.head, &None) {
      Some(bx) => {
        self.head = &bx.next;
        Some(&bx.data)
      }
      None => None,
    }
  }
}

macro_rules! list {
  [$($item:expr),+] => {
    {
      let mut list = ForwardList::new();
      $(
        list.push_data($item);
      )*
      list
    }
  }
}

struct Obj {
  val: i32,
}

impl Default for Obj {
  fn default() -> Obj { Obj { val: 0 } }
}

impl Obj {
  fn new(val: i32) -> Obj { Obj { val } }
  fn swap(&mut self, n: &mut i32) {
    let tmp = self.val;
    self.val = *n;
    *n = tmp;
  }
}

trait IMyError {
  fn why(&self) -> &'static str { "An Error Occured" }
}

struct MyError {
  reason: &'static str,
}

impl MyError {
  fn new(reason: &'static str) -> MyError { MyError { reason } }
}

impl IMyError for MyError {
  fn why(&self) -> &'static str { self.reason }
}

fn try_get_value() -> Result<i32, impl IMyError> {
  if std::random::random::<bool>() {
    Ok(5052)
  } else {
    Err(MyError::new("We got a random false"))
  }
}

fn main() {
  match try_get_value() {
    Ok(value) => println!("I got the number {value}"),
    Err(error) => println!("ERROR: {}", error.why()),
  }
}

fn main1() {
  let _b = mvec!(i32);
  let _vec = mvec![1, 2, 3, 4];
  let s1: String = "Simon".into();
  let s2 = "Simon".to_string();
  let s3 = String::from("Simon");
  let s4: String = From::from("Simon");
  let s5: String = Into::into("Simon");
  let s6 = myformat!("{:?} {:?}", Thing::Cpp, Thing::Python);
  println!("Thing: {} {}", s6, Thing::Python);
  let s7 = s1 + " " + &s2;
  println!("Value: {}", s7);
  let m1 = map![
    ("Simon", 22),
    ("Foo", 11),
    ("Bar", 38),
    ("John", 50),
    ("Doe", 19),
    ("Jane", 21)
  ];
  HashMap::new().entry("Hey").or_insert("Nganga");
  let simons_age = m1.get("Simon").copied().unwrap_or(121);
  println!("Simon's age is {simons_age}");
  //let mut m1 = HashMap::new();
  //m1.insert("simon", 22);
  //m1.insert("foo", 11);
  //m1.insert("bar", 38);
  println!("{:?}", m1);

  for (name, age) in &m1 {
    println!("Hello {name}, you are {age} years old!");
  }

  //let mut names = list![
  //  "Simon".to_string(),
  //  "John".into(),
  //  "Doe".into(),
  //  "Jane".into()
  //];
  let mut names = ForwardList::from_iter(m1.keys());
  //names.push_data("Darius".into());
  //for name in names.iter_mut() {
  //  *name = name.to_uppercase();
  //}
  for name in names.iter() {
    println!("Hello {name}?");
  }
  let g = &mut 34;
  println!();
  let mut obj = Obj::new(5052);
  let mut n = 1234;
  println!("B4: val={} n={n}", obj.val);
  obj.swap(&mut n);
  println!("AF: val={} n={n}", obj.val);
  obj.swap(&mut 6789);
  println!("TMP(6789): val={}", obj.val);
  obj.swap(g);
  println!("val={} g={g}", obj.val);
}
