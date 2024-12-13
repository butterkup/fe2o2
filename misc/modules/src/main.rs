#![feature(min_specialization)]
#![allow(unused)]
#[derive(Debug)]
struct Person {
  name: String,
  email: String,
  age: i32,
}

fn take(s: String) {}

fn greet(person: &Person) {
  println!("Hello {}?", person.name);
}

impl Person {
  fn greet(&self) { greet(&self); }
}

struct Empty();

impl Empty {
  fn r#static() {
    println!("This is a static method");
  }
  fn method(&self) {
    println!("This is a normal method");
  }
}

struct Choose<T> {
  __: std::marker::PhantomData<T>,
}

macro_rules! create_chooser {
  ($($typ:ident),+) => {
    $(
      impl Choose<$typ> {
        fn choose(&self) {
          Self::chosen();
        }
        fn chosen() {
          println!("Specialized Choose<{}>", stringify!($typ));
        }
      }
    )*
  }
}

create_chooser!(
  u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, char, bool
);

enum C {
  A(i8, i8),
  B(i8, i8),
  E(),
}

fn get(a: bool) -> C { if a { C::A(1, 2) } else { C::B(3, 4) } }

struct Bound<'a, T, R, M: Fn(&'a T) -> R> {
  instance: &'a T,
  method: M,
}

impl<'a, T, R, M: Fn(&'a T) -> R> Bound<'a, T, R, M> {
  fn call(self: &Self) -> R { (self.method)(self.instance) }
}

impl <'a, T, R, M: Fn(&'a T) -> R> Bound<'a, T, R, M> {
  fn second_impl(&self) {
    let planet = "Earth";
    dbg!(planet);
  }
}

macro_rules! mula {
  () => {
    dbg!(file!());
    dbg!(column!());
  }
}

fn main() {
  let function = Empty::r#static;
  function();
  let method = Empty::method;
  method(&Empty());
  let someone = Person {
    name: String::from("John Doe"),
    email: String::from("johndoe@phillipines.com"),
    age: 34,
  };
  let package = Bound {
    instance: &someone,
    method: Person::greet,
  };
  package.call();
  let c = Choose::<f32> {
    __: std::marker::PhantomData,
  };
  c.choose();
  Choose::<f64>::chosen();
  Choose::<char>::chosen();
  Choose::<bool>::chosen();
  Choose::<i8>::chosen();
  Choose::<i16>::chosen();
  Choose::<i32>::chosen();
  Choose::<i64>::chosen();
  Choose::<isize>::chosen();
  Choose::<u8>::chosen();
  Choose::<u16>::chosen();
  Choose::<u32>::chosen();
  Choose::<u64>::chosen();
  Choose::<usize>::chosen();
  let e = Empty();
  Empty::r#static();
  e.method();
  let me1 = Person {
    name: String::from("Simon Nganga"),
    email: String::from("simongash@gmail.com"),
    age: 22,
  };
  dbg!(&me1);
  let mut me2 = Person { age: 25, ..me1 };
  dbg!(&me2);
  take(me2.email);
  println!("Hello {}?", me2.name);
  //greet(&me);
  //println!("Hello {}?", me1.name);
  let point = (1, 2, 3);
  println!("point: {point:?}");
  //println!("point.000: {}", point.000);
  println!("point.0: {}", point.0);
  for _ in 0..5 {
    match get(rand::random_bool(0.5)) {
      C::A(a, b) => println!("C::A({}, {})", a, b),
      C::B(a, b) => println!("C::B({}, {})", a, b),
      C::E() => panic!("This is not possible"),
    }
  }
  dbg!(file!());
  dbg!(line!());
  dbg!(column!());
  mula!();
  package.second_impl();
}

