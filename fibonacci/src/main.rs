fn main() {
    println!("{}", fibonacci_number(2000));
}

fn fibonacci_number(number: i32) -> i32 {
    if number == 1 || number == 0 { return 1 ;}
    return fibonacci_number(number - 1) + fibonacci_number(number - 2);
}
