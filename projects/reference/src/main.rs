fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // let s2 = String::from("hello");
    // let len2 = calculate_length2(s2);
    // println!("The length of '{}' is {}.", s2, len2);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{s2}");
    {
        let hoge = &mut s2;
        println!("{hoge}");
        let fuga = &mut s2;
    }
    let fuga = &mut s2;
    println!("{fuga}");

    let mut s3 = String::from("hello");
    let r1 = &s3;
    let r2 = &s3;
    println!("{}, {}", r1, r2);
    let r3 = &mut s3;
    // println!("{r1}")
    println!("{r3}")
}

fn calculate_length(s: &String) -> usize {
    // Q: reference でもlen()メソッドを使えるのはなぜ？
    let length = s.len();
    return length;
}

fn calculate_length2(s: String) -> usize {
    // Q: reference でもlen()メソッドを使えるのはなぜ？
    let length = s.len();
    return length;
}

fn change(s: &mut String) {
    s.push_str(", world");
}
