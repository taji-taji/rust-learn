fn main() {
    for i in 1..13 {
        let mut lyrics = String::new();
        lyrics.push_str(&lyrics_base(i));
        lyrics.push_str("\n");
        for j in (1..i + 1).rev() {
            lyrics.push_str(&get_nth_present(j));
            if j == 1 {
                lyrics.push_str(".");
            } else {
                lyrics.push_str(",");
            }
            lyrics.push_str("\n");
        }
        lyrics.push_str("\n");
        println!("{}", lyrics);
    }
}

fn lyrics_base(n: u32) -> String {
    ["On the".to_string(), get_nth_phrase(n), "day of Christmas,".to_string(), "my true love sent to me.".to_string()].join(" ")
}

fn get_nth_phrase(n: u32) -> String {
    if n == 1 {
        return "first".to_string();
    } else if n == 2 {
        return "second".to_string();
    } else if n == 3 {
        return "third".to_string();
    } else if n == 4 {
        return "fourth".to_string();
    } else if n == 5 {
        return "fifth".to_string();
    } else if n == 6 {
        return "sixth".to_string();
    } else if n == 7 {
        return "seventh".to_string();
    } else if n == 8 {
        return "eighth".to_string();
    } else if n == 9 {
        return "ninth".to_string();
    } else if n == 10 {
        return "tenth".to_string();
    } else if n == 11 {
        return "eleventh".to_string();
    } else if n == 12 {
        return "twelve".to_string();
    }
    "".to_string()
}

fn get_nth_present(n: u32) -> String {
    if n == 1 {
        return "a partridge in a pear tree".to_string();
    } else if n == 2 {
        return "two turtle doves".to_string();
    } else if n == 3 {
        return "three French hens".to_string();
    } else if n == 4 {
        return "four calling birds".to_string();
    } else if n == 5 {
        return "five golden rings".to_string();
    } else if n == 6 {
        return "six geese a-laying".to_string();
    } else if n == 7 {
        return "seven swans a-swimming".to_string();
    } else if n == 8 {
        return "eight maids a-milking".to_string();
    } else if n == 9 {
        return "nine ladies dancing".to_string();
    } else if n == 10 {
        return "ten lords a-leaping".to_string();
    } else if n == 11 {
        return "eleven pipers piping".to_string();
    } else if n == 12 {
        return "twelve drummers drumming".to_string();
    }
    "".to_string()
}