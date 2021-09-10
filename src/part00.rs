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

use self::NumberOrNothing::{Nothing, Number};
fn print_number_or_nothing(n: NumberOrNothing) {
  match n {
    Nothing => println!("n = <nothing>"),
    Number(n) => println!("n = {:?}", n),
  };
}

pub fn main() {
  print_number_or_nothing(vec_min(vec![]));

  // println!("NumberOrNothing::Nothing = {}", vec_min(vec![6,4,5]));
}
