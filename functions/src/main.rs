fn main() {
    let x = celsius_converter(100.0);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn celsius_converter(temperature: f32) -> f32 {
    return (temperature - 32.0)/1.8;
}