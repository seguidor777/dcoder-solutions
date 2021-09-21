use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let year: u16 = input.trim().parse().expect("cannot parse input");
    let century = (year + 99) / 100;

    // Exemptions
    if century > 10 && century < 14 {
        println!("{}th", century);
        return
    }

    let suffix = match century % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    println!("{}{}", century, suffix)
}
