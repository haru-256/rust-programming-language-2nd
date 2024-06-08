fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{s1}");

    let s2 = s1;
    println!("{s2}");
    // println!("{s1}")

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{s}");
    let x = 5;
    makes_copy(x);
    println!("{x}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
