use std::io;

// Divide and conquer
fn partition(v: &mut Vec<char>, low: i32, high: i32) -> i32 {
    let pivot = v[high as usize];

    let mut i = low - 1;
    let mut j = low;

    while j < high {
        if v[j as usize].to_ascii_uppercase() < pivot.to_ascii_uppercase() {
            i += 1;
            v.swap(i as usize, j as usize);
        }

        j += 1;
    }

    let pi = i + 1;

    v.swap(pi as usize, high as usize);

    pi
}

// Perform the ascending sort
fn quicksort(mut v: &mut Vec<char>, low: i32, high: i32) {
    if low < high {
        let pi = partition(&mut v, low, high);

        quicksort(&mut v, low, pi - 1);
        quicksort(&mut v, pi + 1, high);
    }
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

    let mut v: Vec<char> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    // Constraints
    if n >= 1 && n <= 26 {
        let v_len = v.len() as i32;

        quicksort(&mut v, 0, v_len - 1);
        println!(
            "{}",
            v.iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
