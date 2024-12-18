#[derive(Debug, Copy, Clone)]
enum Shape {
  Triangle(f32, f32, f32),
  Rectangle { width: f32, height: f32 },
  Circle(f32),
}

trait M {
  #![allow(dead_code)]
  type Output;
  fn sqr(&self) -> Self::Output;
  fn cube(&self) -> Self::Output;
}

impl M for f32 {
  type Output = Self;
  fn cube(&self) -> Self { self * self * self }
  fn sqr(&self) -> Self { self * self }
}

impl Shape {
  fn area(&self) -> f32 {
    match self {
      Self::Rectangle {
        width: w,
        height: h
      } => w * h,
      Self::Circle(radius) => std::f32::consts::PI.sqr() * radius,
      Self::Triangle(a, b, c) => {
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
      }
    }
  }
}

#[allow(unused)]
enum O<T> {
  N,
  S(T),
}
#[allow(unused)]
use O::{N, S};

impl FromStr for Shape {
  type Err = std::num::ParseFloatError;
  fn from_str(string: &str) -> Result<Shape, Self::Err> {
    match string.split_once('x') {
      None => Ok(Self::Circle(string.parse::<f32>()?)),
      Some((a, b)) => match b.split_once('x') {
        Some((b, c)) => Ok(Self::Triangle(a.parse()?, b.parse()?, c.parse()?)),
        None => Ok(Self::Rectangle {
          width: a.parse()?,
          height: b.parse()?,
        }),
      },
    }
  }
}

use std::io::{BufRead, Write};
use std::str::FromStr;
use Shape::{Circle, Rectangle, Triangle};
macro_rules! impl_shape_from {
  ($($type:ty),*) => {
  $(
      impl From<$type> for Shape {
        fn from(string: $type) -> Shape {
          Shape::from_str(&string).expect(
            &format!(
              "Decode error, {} {:?} does not encode a Shape",
              stringify!($type), string
            )
          )
        }
      }
    )*
  }
}

impl_shape_from!(String, &str);

fn main() {
  let shapes = [
    "23".into(),
    "12x34".into(),
    "12x23x34".into(),
    Shape::from("90"),
    Shape::from("78x90"),
    Shape::from("78x89x90"),
    Rectangle {
      width: 12.0,
      height: 34.0,
    },
    Circle(34.0),
    Triangle(12.0, 34.0, 23.0),
  ];
  for shape in shapes {
    println!("{shape:?}.area() = {}", shape.area());
  }

  println!(
    "\nCircle(F) Rectangle(FxF) Triangle\
     (FxFxF) {{quit|q|exit}}=exit F=f32"
  );
  loop {
    let mut buffer = String::new();
    print!("Enter a shape: ");
    let _ = std::io::stdout().flush();
    match std::io::stdin().lock().read_line(&mut buffer) {
      Ok(0) => break,
      Ok(_) => match buffer.trim() {
        "exit" | "quit" | "q" => break,
        "" => continue,
        pat => {
          let shape: Shape = pat.into();
          println!("{:?}.area() = {}", shape, shape.area());
        }
      },
      Err(e) => eprintln!("{e}"),
    }
  }
  println!("Bye!");
}
