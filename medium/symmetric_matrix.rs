use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: usize = input.trim().parse().expect("cannot parse input");
    let mut matrix: Vec<Vec<String>> = Vec::with_capacity(n);

    // Create matrix
    for _ in 0..n {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");
        matrix.push(input.split_whitespace().map(|x| x.to_string()).collect());
    }

    // Check if it is symmetric
    match (0..n).all(|i| (0..n).all(|j| i == j || matrix[i][j] == matrix[j][i])) {
        true => println!("YES"),
        false => println!("NO"),
    }
}
