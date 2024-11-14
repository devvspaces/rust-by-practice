
#[derive(Debug)]
struct Rectangle (u32, u32);

impl Rectangle {
  fn area(&self) -> u32 {
      self.0 * self.1
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.0 > other.0 && self.1 > other.1
  }
  fn square(size: u32) -> Self {
    Self(size, size)
  }
}

fn main() {
  let r1 = Rectangle(30, 50);
  let r2 = Rectangle(40, 70);


  println!("Can r2 hold r1? {}", r2.can_hold(&r1));
  println!("Can r1 hold r2? {}", r1.can_hold(&r2));

  dbg!(Rectangle::square(12));
}
