pub struct Guess {
  value: i32
}

impl Guess {
  pub fn new(value: i32) -> Self {
    if value > 100 {
      panic!("Guess value must be between 1 and 100, got {value}.");
    }
    Guess { value }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_creating_guess() {
        Guess::new(-1);
    }
}
