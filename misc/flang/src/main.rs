#![allow(unused, non_camel_case_types)]
struct __closure_1 {
  __capture_1: Vec<i32>,
}

impl __closure_1 {
  fn call(&self) {
    println!("vec = {:?}", self.__capture_1);
  }
}

struct __closure_2<'a> {
  __capture_1: &'a mut Vec<i32>,
  __capture_2: Vec<u8>,
}

impl<'a> __closure_2<'a> {
  fn call(&mut self, value: i32) {
    self.__capture_1.push(value);
    self.__capture_2.push((value >> 0) as u8);
    self.__capture_2.push((value >> 8) as u8);
    self.__capture_2.push((value >> 16) as u8);
    self.__capture_2.push((value >> 24) as u8);
    println!("We have: {:?} {:?}", self.__capture_1, self.__capture_2);
  }
}

fn true_capture() {
  let mut vec1 = vec![1, 4, 16, 64, 256, 1024, 4096, 16384];
  let vec2 = vec![1, 2, 4, 8, 16, 32, 64, 128];
  println!("We have: {:?} {:?}", vec1, vec2);
  let mut func = |value: i32| {
    vec1.push(value);
    let mut vec2 = vec2.clone();
    vec2.push((value >> 0) as u8);
    vec2.push((value >> 8) as u8);
    vec2.push((value >> 16) as u8);
    vec2.push((value >> 24) as u8);
    println!("We have: {:?} {:?}", vec1, vec2);
  };

  func(65536);
  println!("We have: {:?} {:?}", vec1, vec2);
}

fn capture() {
  let mut vec1 = vec![1, 4, 16, 64, 256, 1024, 4096, 16384];
  let mut vec2 = vec![1, 2, 4, 8, 16, 32, 64, 128];
  println!("We have: {:?} {:?}", vec1, vec2);
  let mut func = __closure_2 {
    __capture_1: &mut vec1,
    __capture_2: vec2.clone(),
  };
  func.call(65536);
  println!("We have: {:?} {:?}", vec1, vec2);
}

fn capture_local() -> impl FnOnce() {
  let vec = vec![1, 2, 3];
  let show_vec = move || println!("vec = {:?}", vec);
  show_vec();
  show_vec
}

fn capture_local_i() -> __closure_1 {
  let vec = vec![1, 2, 3];
  let show_vec = __closure_1 { __capture_1: vec };
  show_vec.call();
  show_vec
}

fn mover(thing: String) -> impl FnOnce() -> String { move || thing }

fn copier(thing: String) -> impl Fn() -> String { move || thing.clone() }

fn mutator(mut thing: String) -> impl FnMut() -> String {
  let mut counter = 1;
  move || {
    let mut temp = thing.clone();
    temp.push_str(&counter.to_string());
    counter += 1;
    temp
  }
}

fn main() {
  let mut get_string = mutator("Simon Nganga".into());
  println!("String is {}", get_string());
  println!("String is {}", get_string());
  println!("String is {}", get_string());
  println!("String is {}", get_string());
  println!("String is {}", get_string());
  capture();
  true_capture();
  capture_local()();
  capture_local_i().call();
  let factory = flang::inventory![
    flang::Red,
    flang::Blue,
    flang::Red,
    flang::Blue,
    flang::Red,
    flang::Blue
  ];
  println!("GiveAway: {:?}", factory.giveaway(Some(flang::Red)));
}
