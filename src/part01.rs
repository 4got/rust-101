enum NumberOrNothing {
  Number(i32),
  Nothing,
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
  let mut min = NumberOrNothing::Nothing;
  for el in vec {
    match min {
      NumberOrNothing::Nothing => {
        min = NumberOrNothing::Number(el);
      }
      NumberOrNothing::Number(n) => {
        let new_min = min_i32(n, el);
        min = NumberOrNothing::Number(new_min);
      }
    }
  }
  min
}

fn min_i32(a: i32, b: i32) -> i32 {
  if a < b {
    a
  } else {
    b
  }
}

fn vec_sum(vec: Vec<i32>) -> i32 {
  let mut sum: i32 = 0;
  for n in vec {
    sum += n;
  }
  sum
}

fn vec_print(vec: Vec<i32>) {
  for n in vec {
    print!("n = {:?}", n);
  }
}

use self::NumberOrNothing::{Nothing, Number};

impl NumberOrNothing {
  fn print(self) {
    match self {
      Nothing => println!("n = <nothing>"),
      Number(n) => println!("n = {:?}", n),
    };
  }
  fn number_or_default(self, default: i32) -> i32 {
    match self {
      Nothing => default,
      Number(n) => n,
    }
  }
}

pub fn main() {
  vec_min(vec![]).print();
  vec_min(vec![3, 4, 5]).print();
  vec_min(vec![]).number_or_default(42);
  println!("n = {}", vec_min(vec![]).number_or_default(42));
  println!("sum = {}", vec_sum(vec![3, 4, 5]));
  vec_print(vec![3, 4, 5]);
}
