fn fahrenheit_to_celsius(value: f32) -> f32 {
    return (value - 32.0) / 1.8
}

fn main() {
    let fahrenheit: f32 = 96.0;
    println!("{} degrees fahrenheit is {} degrees celsius!", fahrenheit, fahrenheit_to_celsius(fahrenheit));
}