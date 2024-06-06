fn main() {
    println!("Hello, world!");
    let x = another_function(3, 'h');
    println!("The value of x is: {}", x);

    let y = {
        let x = 6;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn another_function(value: i32, unit_label: char) -> i32 {
    println!("Another function. x is {value}{unit_label}");
    return value + 1;
    // value + 1
}
