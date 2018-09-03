fn main() {
    // Mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing with the same type
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    // Shadowing with different type
    let spaces = "     ";
    let spaces = spaces.len();

    println!("spaces is {}", spaces);

    // Mutable variableでやろうとすると型が違うのでコンパイルエラーになる

    // let mut spaces = "     ";
    // spaces = spaces.len();
}
