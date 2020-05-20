use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let ages: Vec<u32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let eldest = find_eldest(&ages);

    println!("{}", eldest);
}

fn find_eldest(ages: &Vec<u32>) -> u32 {
    let mut eldest = ages[0];

    for &age in ages[1..].iter() {
        if age > eldest {
            eldest = age;
        }
    }

    return eldest;
}
