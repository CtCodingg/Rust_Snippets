fn sum(a: i32, b: i32) -> i32 {
  return a + b;
}

fn greater_than(a: i32, b: i32) -> bool {
  return a > b;
}

fn main() {
  // sum
  let sum_of_two_int = sum(2, 3);
  println!("sum_of_two_int = {}", sum_of_two_int);

  // greater_than
  let is_greater = greater_than(2, 3);
  println!("is_greater = {}", is_greater);
}