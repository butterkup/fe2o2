#![feature(f16)]

#[cfg(not(target_os = "windows"))]
fn os() -> &'static str { "linux" }
#[cfg(target_os = "windows")]
fn os() -> &'static str { "windows" }

fn get(value: u16) -> f16 { unsafe { std::mem::transmute(value) } }

fn get64(value: u64) -> f64 { unsafe { std::mem::transmute(value) } }

fn print(value: u64) {
  println!("0x{:08x}\t{:e}", value, get64(value));
}

fn main() {
  print(0x7fef_ffff_ffff_ffff);
  for value in 0..=0xffffu16 {
    eprintln!("0x{:04x}\t{:016b}\t{}", value, value, get(value) as f32);
  }
  println!("f16::from_layout(0x4540) = {}", get(0x4540) as f32);
  if rand::random_bool(0.5) {
    println!("[{}] The fire is too hot!", os());
  } else {
    println!("[{}] Frozen flame", os());
  }
}
