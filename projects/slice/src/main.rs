fn main() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    let hoge = "hoge";
    println!("The first word is: {}", word);
    // s.clear(); // error!

    let a = [1, 2, 3, 4, 5];
    let b = &a[1..3];
    println!("{:#?}", b);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
