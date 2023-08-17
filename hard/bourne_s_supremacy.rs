use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read t");
    let t: u8 = input.parse().expect("cannot parse t");

    for _ in 0..t {
        lines.next(); // Skip n
        input = lines.next().unwrap().expect("cannot read points");

        let points: Vec<usize> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        input = lines.next().unwrap().expect("cannot read values");

        let mut values = input.split_whitespace();
        let mut source: usize = values.next().unwrap().parse().expect("cannot parse source");
        let dest: usize = values.next().unwrap().parse().expect("cannot parse dest");
        let mut answer = "No";

        for _ in 0..points.len() {
            let i = source - 1; // Index starts at 0

            match points[i] == dest {
                true => {
                    answer = "Yes";
                    break;
                }
                false => source = points[i],
            }
        }

        println!("{}", answer);
    }
}
