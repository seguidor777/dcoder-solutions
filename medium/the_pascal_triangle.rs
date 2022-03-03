use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read n");
    let n: usize = input.trim().parse().expect("cannot parse n");

    for line in 1..=n {
        let mut c = 1;
        let mut digits = format!("{:1$}", "", n - line);

        for i in 1..=line {
            digits.push_str(&format!("{} ", c));
            c = c * (line - i) / i;
        }

        digits.pop();
        println!("{}", digits);
    }
}
