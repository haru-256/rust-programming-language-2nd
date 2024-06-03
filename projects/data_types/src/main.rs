fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);
    // turbofish
    // https://keens.github.io/blog/2019/12/03/rustnoturbofishworikaisuru/
    let guess = "42".parse::<u32>().expect("Not a number!");
    println!("Guess: {}", guess);

    let x = 3_000;
    println!("x: {}", x);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10;
    println!("sum: {}", sum);
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);
    let product = 4 * 30;
    println!("product: {}", product);
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    let truncated_quotient = 43 / 5;
    println!("truncated_quotient: {}", truncated_quotient);
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    let c = 'z';
    println!("c: {}", c);
    let z: char = 'ℤ';
    println!("z: {}", z);
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat);
    let heart_eyed_cat = "😻";
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);

    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);
    let empty = ();
    println!("empty: {:?} ", empty);

    let a = [1, 2, 3, 4, 5];
    let b: [f64; 5] = [3.14, 2.71, 1.61, 1.41, 1.0];
    let c = [3; 5];
    println!("a: {:?}, b: {:?}, c: {:?}, c[0]: {}", a, b, c, c[0]);

    another_function();
}

fn another_function() {
    println!("Another function.");
}
