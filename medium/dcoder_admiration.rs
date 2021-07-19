use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let t: u8 = input.trim().parse().expect("cannot parse input");

    for _ in 0..t {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("cannot read input");

        let n: f64 = input.trim().parse().expect("cannot parse input");
        let admirers = n - n.sqrt().floor();

        println!("{}", admirers as u32);
    }
}
