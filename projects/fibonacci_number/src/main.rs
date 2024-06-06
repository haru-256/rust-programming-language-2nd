fn main() {
    let n = 10;
    for i in 0..(n + 1) {
        let result = fibonacci(i);
        println!("The {i}th fibonacci number is {result}");
    }
}

fn fibonacci(n: i32) -> i32 {
    let mut sum = 0;
    let mut pre_sum: i32 = sum;
    for i in 1..(n + 1) {
        if i == 1 {
            sum = 1;
            pre_sum = 0;
            continue;
        }
        let temp = sum;
        sum += pre_sum;
        pre_sum = temp;
    }
    return sum;
}
