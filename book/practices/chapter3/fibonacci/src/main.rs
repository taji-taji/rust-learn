use std::io;

fn main() {


    loop {
        println!("Input nth number:");

        let mut input_value = String::new();

        io::stdin().read_line(&mut input_value)
            .expect("Failed to read line");

        let input_value: u32 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        get_fibonacchi_of(input_value);
        // println!("{}番目のフィボナッチ数は: {}", input_value, );
    }
}

fn get_fibonacchi_of(n: u32) -> u32 {
    // n == 4
    let mut previous: u32 = 0;
    let mut current: u32 = 0;
    let mut sum: u32 = 0;
    for i in 0..n {
        if i == 0 {
            sum += 1;
            continue;
        } else {
            current = sum;
            sum += previous;
            previous = current;
        }
    }
    sum
}

// 0:0
// 1:1
// 2:1
// 3:2
// 4:3
// 5:5
// 6:8
