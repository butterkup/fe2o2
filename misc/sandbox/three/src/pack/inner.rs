pub fn three() {
  println!("Hello from three::pack::inner::three()");
  super::pack(); // relative
  super::hidden(); // relative path
  crate::pack::outer::hidden(); // absolute path
}
pub mod my {
  pub type Result<T> = std::result::Result<T, u32>;

  pub fn get_value() -> Result<u8> { Ok(90) }
  pub fn take() -> Option<u8> { Some(30) }
}

pub use my::Result as ResultT;

