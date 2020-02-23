use std::io;

enum Temperature { Fahrenheit, Celsius }

fn main() {
    println!("Please choose input type (F for Fahrenheit, C for Celsius):");

    let mut input_type = String::new();

    io::stdin()
        .read_line(&mut input_type)
        .expect("Failed to read line");

    let temperature_type = match input_type.trim().to_lowercase().as_ref() {
        "f" => Temperature::Fahrenheit,
        "c" => Temperature::Celsius,
        _ => panic!("Invalid input type"),
    };

    let converter = get_converter(&temperature_type);

    let temperature_name = temperature_to_string(&temperature_type);

    println!("Please input temperature value in {}:", temperature_name);

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let converted_value: f64 = converter(
        value
            .trim()
            .parse()
            .expect("Unable to parse value")
     );

    println!("Converted value: {}", converted_value);
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn get_converter(t: &Temperature) -> Box<Fn(f64) -> f64> {
    Box::new(match t {
        Temperature::Fahrenheit => fahrenheit_to_celsius,
        Temperature::Celsius => celsius_to_fahrenheit,
    })
}

fn temperature_to_string(t: &Temperature) -> String {
    String::from(match t {
        Temperature::Fahrenheit => "Fahrenheit",
        Temperature::Celsius => "Celsius",
    })
}