use std::io;
use std::io::prelude::*;

fn read_vec() -> Vec<i32> {
  let mut vec = Vec::<i32>::new();
  let stdin = io::stdin();

  for line in stdin.lock().lines() {
    let line = line.unwrap();
    match line.trim().parse::<i32>() {
      Ok(num) => vec.push(num),
      Err(_) => println!("Num is NaN"),
    }
  }

  vec
}
fn read_vec_f32() -> Vec<f32> {
  let mut vec = Vec::<f32>::new();
  let stdin = io::stdin();

  for line in stdin.lock().lines() {
    let line = line.unwrap();
    match line.trim().parse::<f32>() {
      Ok(num) => vec.push(num),
      Err(_) => println!("Num is NaN"),
    }
  }
  vec
}

use crate::part02::{vec_min, Nothing, Something, SomethingOrNothing};

// **Exercise 03.1**
pub trait Print {
  fn print2(self);
}
impl Print for i32 {
  fn print2(self) {
    println!("Printed i32: {}", self);
  }
}
impl Print for f32 {
  fn print2(self) {
    println!("Printed f32: {}", self);
  }
}
impl<T: Print> SomethingOrNothing<T> {
  fn print2(self) {
    match self {
      Nothing => println!("t = <nothing>"),
      Something(t) => t.print2(),
    }
  }
}

pub fn main() {
  // let vec = read_vec();
  // let min = vec_min(vec);
  // min.print2();

  let vec = read_vec_f32();
  let min = vec_min(vec);
  min.print2();
}
