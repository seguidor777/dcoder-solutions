use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().expect("cannot read line");
    let n: usize = line.parse().expect("cannot parse n");
    line = lines.next().unwrap().expect("cannot read line");
    let (mut subarray, mut max_subarray) = (vec![], vec![]);

    line.split_whitespace()
        .take(n)
        .enumerate()
        .for_each(|(i, x)| {
            let number: i16 = x.parse().expect("cannot parse number");

            if number >= 0 {
                subarray.push(number);

                // Check if it's last element
                if i == n - 1 {
                    max_subarray = compute_max(&subarray, &max_subarray);
                }
            } else {
                max_subarray = compute_max(&subarray, &max_subarray);

                // Initialize a new subarray
                subarray = vec![];
            }
        });

    println!(
        "{}",
        max_subarray
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn compute_max(subarray: &Vec<i16>, max_subarray: &Vec<i16>) -> Vec<i16> {
    let subarray_sum: i16 = subarray.iter().sum();
    let max_subarray_sum: i16 = max_subarray.iter().sum();

    if subarray_sum > max_subarray_sum
        || (subarray_sum == max_subarray_sum && subarray.len() > max_subarray.len())
    {
        return subarray.to_vec();
    }

    max_subarray.to_vec()
}
