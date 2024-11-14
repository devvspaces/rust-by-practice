use std::io;

fn main() {
    let mut fib = String::new();

    io::stdin().read_line(&mut fib).expect("Failed to read line");

    let fib: u64 = match fib.trim().parse() {
      Ok(num) => num,
      Err(_) => 0,
    };

    let mut a = 0;
    let mut b = 1;

    for _ in 1..fib+1 {
      let temp = a + b;
      a = b;
      b = temp;
      println!("{}", a);
    };
}
