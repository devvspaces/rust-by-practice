use std::io;

fn main() {
    println!("Temperature Converter -- C <-> F");
    // Ask for temp conversion
    println!("Please enter the temperature conversion you want to do: ");
    println!("1 -> Celsius to Fahrenheit");
    println!("2 -> Fahrenheit to Celsius");
    let mut temp_type = String::new();

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Enter: ");

    // Validate value
    let temp_type: u8 = match temp_type.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if temp_type > 2 || temp_type < 1 {
      println!("Enter either 1 or 2");
      return;
    }

    // Ask for temp value
    println!("Please enter the temperature value: ");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Enter a temperature");

    // Validate value
    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.1,
    };

    if temp_type == 1 {
      let temp = ((9/5) as f32 * temp) + 32 as f32;
      println!("Conversion from C => F: {}", temp);
    } else {
      let temp = (temp - 32 as f32) * (5/9) as f32;
      println!("Conversion from F => C: {}", temp);
    }
}
