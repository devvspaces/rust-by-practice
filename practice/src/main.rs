
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
  let v = value_in_cents(Some(Coin::Quarter(UsState::Alabama)));
  println!("Value in cents is {}", v.expect("Not provided"));

  let config_max = Some(3u8);
  if let Some(max) = config_max {
      println!("The maximum is configured to be {max}");
  }
}

fn value_in_cents(coin: Option<Coin>) -> Option<u8> {
  match coin {
    Some(_coin) => {
      match _coin {
        Coin::Penny => Some(1),
        Coin::Nickel => Some(5),
        Coin::Dime => Some(10),
        Coin::Quarter(state) => {
          println!("The state is {state:?}");
          Some(25)
        },
      }
    },
    None => None,
  }
}

