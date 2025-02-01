use std::cmp::PartialOrd;

fn longest<'b>(a: &'b str, b: &str) -> &'b str {
    a
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for num in list {
        if num > max {
            max = num;
        }
    }
    return max;
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
    let a = String::from("Hello");
    let c;
    {
        let b = String::from("xyz");
        c = longest(a.as_str(), b.as_str());
    }
    print!("This is the longest string: {c}\n");

    let numbers = vec![1, 2, 32, 5, 7, 2, 7];
    let big = largest(&numbers);
    print!("The largest number is {big}\n");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
