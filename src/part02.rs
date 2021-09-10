pub enum SomethingOrNothing<T> {
  Something(T),
  Nothing,
}

pub use self::SomethingOrNothing::*;
type NumberOrNothing = SomethingOrNothing<i32>;
type FloatOrNothing = SomethingOrNothing<f32>;

// impl<T> SomethingOrNothing<T> {
//   fn new(o: Option<T>) -> Self {
//     match o {
//       None => Nothing,
//       Some(t) => Something(t),
//     }
//   }
// }

pub trait Minimum: Copy {
  fn min(self, b: Self) -> Self;
}
impl NumberOrNothing {
  pub fn print(self) {
    match self {
      Nothing => println!("The number is: <nothing>"),
      Something(n) => println!("The number is: {}", n),
    };
  }
}
impl FloatOrNothing {
  pub fn print(self) {
    match self {
      Nothing => println!("The number is: <nothing>"),
      Something(n) => println!("The number is: {}", n),
    };
  }
}
impl Minimum for i32 {
  fn min(self, b: Self) -> Self {
    if self < b {
      self
    } else {
      b
    }
  }
}
impl Minimum for f32 {
  fn min(self, b: Self) -> Self {
    if self < b {
      self
    } else {
      b
    }
  }
}
pub fn vec_min<T: Minimum>(vec: Vec<T>) -> SomethingOrNothing<T> {
  let mut min = Nothing;
  for el in vec {
    min = Something(match min {
      Nothing => el,
      Something(n) => el.min(n),
    })
  }
  min
}

pub fn main() {
  vec_min(vec![4, 5, 6]).print();
  vec_min(vec![4.1, 5.2, 6.8]).print();
}
