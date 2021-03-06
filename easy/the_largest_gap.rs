use std::io;

// Find the largest gap
fn max_gap(v: &Vec<i32>, v_len: usize) -> i32 {
    let (mut gap, mut max): (i32, i32);

    max = 0;

    for i in 0..(v_len - 1) {
        for j in 1..v_len {
            gap = i32::abs(v[i] - v[j]);

            if gap > max {
                max = gap
            }
        }
    }

    max
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    // Constraints
    if n >= 2 && n <= 10000 {
        for item in &v {
            if *item < 1 || *item > 100000 {
                return;
            }
        }

        let max = max_gap(&v, v.len());

        println!("{}", max)
    }
}
