use std::io;

// Divide and conquer
fn partition(v: &mut Vec<u32>, low: i32, high: i32) -> i32 {
    let pivot = v[high as usize];

    let mut i = low - 1;
    let mut j = low;

    while j < high {
        if v[j as usize] < pivot {
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
fn quicksort(mut v: &mut Vec<u32>, low: i32, high: i32) {
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

    let _n: u32 = input.trim().parse().expect("cannot parse input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let mut pearls: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let pearls_len = pearls.len() as i32;

    quicksort(&mut pearls, 0, pearls_len - 1);
    println!(
        "{}",
        pearls
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}
