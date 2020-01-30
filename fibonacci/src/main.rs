fn main() {
    println!("{}", new_fibonacci_number(10));
    // println!("{}", fibonacci_number(2000));
}

fn fibonacci_number(number: u128) -> u128 {
    if number == 1 || number == 0 { return 1 ;}
    println!("{}", number);
    return fibonacci_number(number - 1) + fibonacci_number(number - 2);
}

fn new_fibonacci_number(number: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;
    let mut not_number = number - 1;

    while not_number != 0 {
        let temp = b;
        b += a;
        a = temp;
        not_number -= 1;
    }

    return b ;
}