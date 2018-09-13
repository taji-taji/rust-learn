use std::io;

fn main() {
    let mut tmp_type = String::new();
    let mut input_value = String::new();
    let mut input_value_f: f64 = 0.;
    loop {
        println!("Fahrenheit or Celsius (Type F or C):");

        io::stdin().read_line(&mut tmp_type)
            .expect("Failed to read line");

        match tmp_type.trim() {
            "F" | "C" => {
                tmp_type = tmp_type.trim().to_string();
                break;
            },
            _ => println!("Type F or C")
        };
        tmp_type = String::new();
    }

    loop {
        println!("Input temperature value:");

        io::stdin().read_line(&mut input_value)
            .expect("Failed to read line");

        match input_value.trim().parse::<f64>() {
            Ok(num) => {
                input_value_f = num;
                break;
            }
            Err(_) => {
                input_value = String::new();
            },
        };
    }
    if tmp_type == "C" {
        println!("{}", convert_c_to_f(input_value_f));
    } else {
        println!("{}", convert_f_to_c(input_value_f));
    }
}

fn convert_f_to_c(value: f64) -> f64 {
    (value - 32.) / 1.8
}

fn convert_c_to_f(value: f64) -> f64 {
    value * 1.8 + 32.
}
