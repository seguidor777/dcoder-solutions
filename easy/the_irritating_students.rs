use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: u32 = input.trim().parse().expect("cannot parse input");

    for _ in 0..n {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");

        let sum: u32 = input.trim().parse().expect("cannot parse input");
        let section_a = sum / 2;
        let section_b = if sum % 2 == 0 {
            section_a
        } else {
            section_a + 1
        };

        println!("{} {}", section_a, section_b);
    }
}
