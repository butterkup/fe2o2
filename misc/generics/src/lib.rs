pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct BinOp<A, B, C> {
  pub arg1: A,
  pub arg2: B,
  pub op: for<'a> fn(&'a A, &'a B) -> C,
}

impl<A, B, C> BinOp<A, B, C> {
  pub fn call(&self) -> C {
    (self.op)(&self.arg1, &self.arg2)
  }
}

pub struct _Name(String);
pub struct _Email(String);

pub enum Summarizable {
  Name(_Name),
  Email(_Email),
}

pub use Summarizable::Email;
pub use Summarizable::Name;

macro_rules! _from_T {
  ($T:ty, $U:ty) => {
    impl From<$U> for $T {
      fn from(value: $U) -> Self { Self(value.into()) }
    }
  };
}

_from_T!(_Name, &str);
_from_T!(_Name, String);
_from_T!(_Email, &str);
_from_T!(_Email, String);

pub fn get_summary<S: Summary>(s: &S) -> String { s.summarize() }

pub fn create_summary() -> impl Summary {
  Summarizable::Name("Jesus Christ".into())
}

pub fn summary(s: &impl Summary) -> String { s.summarize() }

impl Summary for Summarizable {
  fn summarize(&self) -> String {
    match self {
      Self::Name(_Name(name)) => {
        format!("Refer to me as {name} as it's my name.")
      }
      Self::Email(_Email(email)) => {
        format!("Contact me via email at {email}, am always online.")
      }
    }
  }
}
