fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelvth",
    ];

    let presents = [
        "partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..13 {
        println!("On the {} day of Christmas my true love sent to me", days[day - 1]);
        for present in (1..day + 1).rev() {
            let prefix = if present == 1 && day == 1 { "A " } else if present == 1 { "And a " } else { "" };
            let suffix = if present == 1 { "." } else { "," };
            println!("{}{}{}", prefix, presents[present - 1], suffix);
        }
        println!("");
    }
}
