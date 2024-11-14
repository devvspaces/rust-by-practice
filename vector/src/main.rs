mod front_house;

fn main() {
    use front_house::show;
    let mut todo: Vec<i32> = Vec::new();
    todo.push(3);
    todo.push(5);
    todo.push(6);
    for i in &todo {
        println!("The values are {}", i);
    }
    show::make_good();

    let hello = "Здравствуйте";
    for c in hello.bytes() {
      println!("{c}");
    }
}
