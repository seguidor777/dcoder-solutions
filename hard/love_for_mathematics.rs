use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read N");
    let t: u8 = input.parse().expect("cannot parse N");
    let mut ranges = Vec::new();

    for _ in 0..t {
        let input = lines.next().unwrap().expect("cannot read range");
        let v: Vec<usize> = input
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        ranges.push((v[0], v[1]));
    }

    let (books, happy) = ranges.iter().fold((0, 0), |(books, happy), (x, _)| {
        let h = ranges.iter().filter(|(rx, ry)| rx <= x && ry >= x).count();

        if h > happy || (h == happy && *x < books) {
            return (*x, h)
        }

        (books, happy)
    });

    print!("{} {}", books, happy);
}
