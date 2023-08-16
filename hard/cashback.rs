use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = &mut stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read N");
    let n: usize = input.parse().expect("cannot parse N");
    let mut items: Vec<(u32, u32)> = lines
        .take(n)
        .map(|line| {
            let line = line.unwrap();
            let mut values = line.split_whitespace();

            (
                values
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .expect("cannot parse cost"),
                values
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .expect("cannot parse cashback"),
            )
        })
        .collect();

    items.sort_by(|a, b| b.1.cmp(&a.1));

    let (mut min_dollars, highest_c, _) =
        items
            .iter()
            .fold((0, 0, 0), |(min_dollars, mut highest_c, prev_x), &item| {
                let (c, x) = item;

                if c > highest_c {
                    highest_c = c
                }

                (min_dollars + c - prev_x, highest_c, x)
            });

    if min_dollars < highest_c {
        min_dollars = highest_c;
    }

    println!("{}", min_dollars);
}
