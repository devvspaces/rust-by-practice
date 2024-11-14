use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 4, 5];
    numbers.sort();

    // Find mode
    {
        let mut tracker = HashMap::new();
        for num in &numbers {
            let exist = tracker.entry(*num).or_insert(0);
            *exist += 1;
        }
        let mut mode: Option<i32> = None;
        let mut max = 0;
        for (k, v) in &tracker {
            if *v > max {
                max = *v;
                mode = Some(*k);
            } else if *v == max {
                mode = None;
            }
        }
        match mode {
            Some(mode) => {
                println!("The mode is {mode}");
            }
            None => {
                println!("There is no mode");
            }
        }
    }

    // Find median
    {
        if numbers.len() % 2 == 0 {
            let middle = numbers.len() / 2;
            let a = numbers[middle];
            let b = numbers[middle - 1];
            println!("The median is {}", ((a + b) as f32 / 2 as f32));
        } else {
            let middle = (numbers.len() as f32 / 2 as f32).floor() as usize;
            println!("The median is {}", numbers[middle]);
        }
    }
}
