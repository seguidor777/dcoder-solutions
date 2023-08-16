use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let input = lines.next().unwrap().expect("cannot read k");
    let mut values = input.split_whitespace();

    values.next(); // Skip n

    let k: u32 = values.next().unwrap().parse().expect("cannot parse k");
    let input = lines.next().unwrap().expect("cannot read buildings");
    let mut buildings: Vec<u32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    buildings.sort();

    let (bombs, _, _) = buildings.iter().fold(
        (1, buildings[0], buildings[0]),
        |(bombs, range_start, attack_point), &building| match building > range_start + k {
            true => match building > attack_point + k {
                true => (bombs + 1, building, building),
                false => (bombs, range_start, attack_point),
            },
            false => (bombs, range_start, building),
        },
    );

    println!("{}", bombs);
}
