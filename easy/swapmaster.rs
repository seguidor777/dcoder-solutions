use std::io;

// Perform the symmetric swap
fn swap(v: &mut Vec<u32>) {
    let mut front = 0;
    let mut back = v.len() - 1;

    while front < back {
        let tmp = v[front];

        v[front] = v[back];
        v[back] = tmp;
        front += 1;
        back -= 1;
    }
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: usize = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let mut v: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Constraints
    if n >= 2 && n <= 100 {
        for x in &v {
            if !(*x >= 1 && *x <= 1000) {
                return;
            }
        }

        swap(&mut v);

        println!(
            "{}",
            v.iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
