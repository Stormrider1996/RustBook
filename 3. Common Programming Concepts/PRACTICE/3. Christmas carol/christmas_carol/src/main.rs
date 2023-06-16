fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[day]
        );
        for gift in (0..=day).rev() {
            if day == 0 && gift == 0 {
                println!("{}.", gifts[gift]);
            } else if gift == 0 {
                println!("and {}.", gifts[gift]);
            } else {
                println!("{},", gifts[gift]);
            }
        }
        println!();
    }
}
