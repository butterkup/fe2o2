#![allow(unused)]
#![feature(random)]

use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Read;

struct Int(i32);

fn get_value() -> Result<i32, Int> {
  if std::random::random() {
    Ok(12345)
  } else {
    Err(54321.into())
  }
}

impl From<i32> for Int {
  fn from(val: i32) -> Self { Int(val) }
}

impl From<Int> for std::io::Error {
  fn from(value: Int) -> Self { ErrorKind::PermissionDenied.into() }
}

fn extract(vec: &Vec<i32>) -> Option<i32> { Some(vec.first()? + 1000) }

fn get_user_name() -> Result<String, std::io::Error> {
  println!("value: {}", get_value()?);
  std::fs::read_to_string("username.txt")
  //let mut file = std::fs::File::open("username.txt")?;
  //let mut username = String::new();
  //file.read_to_string(&mut username)?;
  //Ok(username.trim().into())
}

fn main() -> std::io::Result<()> {
  let path = std::path::Path::new("~/one/one");
  println!(
    "username: {:?}",
    get_user_name().unwrap_or("Simon Nganga".into())
  );
  println!("exists: {:?}", path.try_exists());
  println!("chdir: {:?}", std::env::set_current_dir(path));
  println!("curdir: {:?}", std::env::current_dir());
  BufReader::new(std::fs::File::open(file!())?)
    .lines()
    .zip(1usize..)
    .for_each(|(line, line_number)| {
      println!("{line_number:3}: {}", line.expect("expected a line"))
    });
  Ok(())
  //match std::fs::File::open(file!()) {
  //  Ok(mut file) => {
  //    let mut buf_file = BufReader::new(file);
  //    for (line, line_number) in buf_file.lines().zip(1usize..) {
  //      match line {
  //        Ok(content) => println!("{line_number}: {content}"),
  //        Err(error) => println!("Error: {error}"),
  //      }
  //    }
  //  }
  //  Err(error) => println!("Error: {error}"),
  //}
}

fn one(a: i32) { two(a + 1); }
fn two(a: i32) { three(a + 1); }
fn three(a: i32) { four(a + 1); }
fn four(a: i32) { five(a + 1); }
fn five(a: i32) {
  panic!("{a}");
}
