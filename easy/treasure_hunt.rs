use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let max_w: u32 = input.trim().parse().expect("cannot parse input");
    let (value1, weight1) = read_item();
    let (value2, weight2) = read_item();

    let total = if weight1 + weight2 <= max_w {
        value1 + value2
    } else if value1 > value2 && weight1 <= max_w {
        value1
    } else if weight2 <= max_w {
        value2
    } else {
        0
    };

    println!("{}", total);
}

// Read a line containing the value and weight of an item
fn read_item() -> (u32, u32) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<u32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    return (v[0], v[1]);
}
