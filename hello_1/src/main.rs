use hello_1::convert;
use std::io;

fn main() {
    println!("Temperature converter program \n Choose the conversion type: \n 1. Celsius to Fahrenheit \n 2. Fahrenheit to Celsius");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    let converted_user_input: u8 = user_input.trim().parse::<u8>().expect("Please enter a valid number");

    println!("Please, enter the temperature value: ");

    let mut temperature_value = String::new();

    io::stdin()
        .read_line(&mut temperature_value)
        .unwrap();

    let temperature_value = temperature_value.trim().parse::<f32>().expect("Please enter a valid number");

    match convert(temperature_value, converted_user_input) {
        Some(result) => println!("The converted temperature is: {result}"),
        None => println!("Invalid conversion type"),
    }
}
