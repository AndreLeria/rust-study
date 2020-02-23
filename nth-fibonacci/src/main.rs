fn main() {
    let mut input = String::new();

    println!("Please input nth desired fibonacci number:");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u64 = input.trim().parse().expect("Failed to parse value");

    println!("{}{} fibonnaci number: {}", n, nth_string(&n), fib(n));
}

fn nth_string(n: &u64) -> String {
    let n_string = n.to_string();
    String::from(
        match n_string[n_string.len()-1..].as_ref() {
            "1" => "st",
            "2" => "nd",
            "3" => "rd",
            _ => "th",
        }
    )
}

fn fib(n: u64) -> u64 {
    if n > 1 { fib(n - 1) + fib(n - 2) } else { n }
}