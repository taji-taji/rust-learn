fn main() {
    // {} でフォーマット
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            verb="jumps over",
            subject="the quick brown fox");

    // フォーマットの型指定
    // {:b} はBinary
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // 左側を空文字で埋める
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>width$}", number=11, width=6);

    // 左側を0で埋める
    println!("{number:>0width$}", number=1, width=6);
    println!("{number:>0width$}", number=11, width=6);

    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));

    // Activity
    let pi = 3.141592;
    println!("Pi is roughly {:.*}", 3, pi);
}