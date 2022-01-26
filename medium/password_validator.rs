use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read input");
    let n: u8 = input.parse().expect("cannot parse input");

    'outer: for _ in 0..n {
        let password = lines.next().unwrap().expect("cannot read input");
        let (mut upper_count, mut lower_count, mut num_count): (u8, u8, u8) = (0, 0, 0);

        if password.len() < 6 || password.len() > 20 {
            println!("False");
            continue;
        }
        
        for c in password.chars() {
            if c.is_ascii_uppercase() {
                upper_count += 1;
            } else if c.is_ascii_lowercase() {
                lower_count += 1;
            } else if c.is_ascii_digit() {
                num_count += 1;
            }
            
            if upper_count >= 1 && lower_count >= 1 && num_count >= 1 {
                println!("True");
                continue 'outer;
            }
        }
        
       println!("False");
    }
}
