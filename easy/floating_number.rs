use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let t: i32 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");

        let n: f32 = input.trim().parse().expect("cannot parse input");

        println!("{:.2}", n);
    }
}
