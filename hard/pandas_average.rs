use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = &mut stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read t");
    let t: u8 = input.parse().expect("cannot parse t");

    for _ in 0..t {
        let _ = lines.next().unwrap().expect("cannot read array size");
        let input = lines.next().unwrap().expect("cannot read array elements");
        let mut a: Vec<i32> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();

        match a.len() {
			0..=1 => println!("NO"),
			_ => {
				let mut best_diff = f32::INFINITY;

				for i in 1..a.len() {
					let right_half = a.split_off(i);
					let right_sum = right_half.iter().sum::<i32>();
					let left_sum = a.iter().sum::<i32>();

                    // Restore original array
					a.extend(right_half);

					let diff = (left_sum as f32 / i as f32 - right_sum as f32 / (a.len() - i) as f32).abs();

					if diff < best_diff {
						best_diff = diff;
					}
				}

				println!("{:.2}", best_diff);
			}
	    }
    }
}
