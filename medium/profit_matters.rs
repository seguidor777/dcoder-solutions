//rust 1.30.0 

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let prices: Vec<u16> = input
        .trim()
        .replace("[", "")
        .replace("]", "")
        .split(",")
        .filter_map(|p| p.parse().ok())
        .collect();
    let mut min = 0;
    let mut max = 0;
    let prices_len = prices.len();

    if prices_len == 1 {
        println!("[{}]", prices[0]);

        return;
    }

    if prices[0] > prices[1] {
        max = prices[0];
        min = prices[1];
    } else {
        max = prices[1];
        min = prices[0];
    }

    for &price in prices.iter().take(prices_len).skip(2) {
        if price > max {
            max = price;
        } else if price < min {
            min = price;
        }
    }

    println!("[{},{}]", min, max);
}
