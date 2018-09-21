fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of `{}` is {}.", s1, len);

    change(&mut s1);
    println!("{}", s1);

    {
        {
            // スコープ内で複数回mutableな参照はできない
            let r1 = &mut s1;
            // let r2 = &mut s1;
        }

        {
            let r1 = &mut s1;
        }
        // r1がスコープをぬければr1はdropされ、s1は参照されなくなるので、mutableな参照が可能
        let r2 = &mut s1;

        {
            let r1 = &s1;
            // immutableな参照は複数回可能
            let r2 = &s1;
            // immutableな参照をしているとmutableな参照はできない
            // let r3 = &mut s1;
        }
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // スコープを抜けても `s` は参照だけで所有権は持っていないので何起きない

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}