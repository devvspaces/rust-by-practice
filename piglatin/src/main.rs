fn main() {
    let pig = string_to_pig("apple");
    if let Some(pig) = pig {
      println!("The pig latin is {pig}")
    }
}

fn string_to_pig(string: &str) -> Option<String> {
    let vowels = "aeiou";
    let char = string.chars().nth(0);
    match char {
        None => {
            println!("The string is empty");
        }
        Some(char) => {
            if vowels.contains(char) {
                return Some(format!("{string}-hay"));
            } else {
                return Some(format!("{}-{char}ay", &string[1..string.len()]));
            }
        }
    }
    return None;
}
