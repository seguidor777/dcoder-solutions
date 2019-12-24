use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let t: u32 = input.trim().parse().expect("cannot parse input");

    if t >= 1 && t <= 10 {
        let mut vs = vec![];

        for _ in 0..t {
            input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("cannot read input");

            let v: Vec<String> = input.split_whitespace().map(String::from).collect();

            vs.push(v);
        }

        for v in vs {
            // Constraints
            if v.len() == 2 {
                let (n, f) = (&v[0], &v[1]);

                if n.len() >= 1
                    && n.len() <= 100
                    && n.chars().all(char::is_lowercase)
                    && f.len() >= 1
                    && f.len() <= 100
                    && f.chars().all(char::is_lowercase)
                {
                    // Check if the sequence contains the favourite sequence
                    if n.contains(f) {
                        println!("Yes")
                    } else {
                        println!("No")
                    }
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }
}
