use std::cmp::Ordering;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let mut happies = 0;
    let mut saddies = 0;

    for window in input.as_bytes().windows(3) {
        match window {
            b"^_^" => happies += 1,
            b"-_-" => saddies += 1,
            _ => (),
        }
    }

    match happies.cmp(&saddies) {
        Ordering::Less => println!("Cody wants to leave job"),
        Ordering::Greater => println!("Cody is happy with Dcoder"),
        Ordering::Equal => println!("Cody doesn't know what to do"),
    }
}
