fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no longer valid

    {
        let s1 = String::from("hello");
        let s2 = s1;

        // println!("{}, world!", s1);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

}
