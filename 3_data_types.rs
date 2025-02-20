fn main() {
  // array
  let arr: [i32; 4] = [0, 1, 2, 3];
  println!("arr = {:?}", arr);
  println!("len_arr = {}", arr.len());

  // tuple
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  println!("tup = {:?}", tup);

  // type casting
  let x: i32 = 5;
  let mut y: f64 = x as f64;
  y = y + 0.1;
  println!("x (i32) = {}", x);
  println!("y (f64) = {}", y);
}