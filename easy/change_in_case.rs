use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let _n: u32 = input.trim().parse().expect("cannot parse input");

    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("cannot read input");
    input = String::new(); // Find if this is needed
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let v: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let (x, y) = (v[0], v[1]);

    // Constraints
    let s_len = s.len() as i32;

    if s_len >= 1 && s_len <= 40 && x >= 0 && x <= s_len && y >= 0 && y <= s_len {
        // Change case on indexes
        let mut result = String::with_capacity(s_len as usize);

        for (i, c) in s.chars().enumerate() {
            if [x, y].contains(&(i as i32)) {
                if c.is_uppercase() {
                    result.push(c.to_ascii_lowercase());
                } else {
                    result.push(c.to_ascii_uppercase());
                }
            } else {
                result.push(c);
            }
        }

        println!("{}", result);
    }
}
