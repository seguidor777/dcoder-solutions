use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n = input.trim().parse().expect("cannot parse input");

    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("cannot read input");

    // Constraints
    if n >= 1 && n <= 20 {
        // Generate string
        let s = s.as_bytes();

        let result = (0..n)
            .map(|i| String::from_utf8(vec![s[i]; 3]).unwrap())
            .collect::<String>();

        println!("{}", result);
    }
}
