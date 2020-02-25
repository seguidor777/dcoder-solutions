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

        let mut total_cost: f32 = input.trim().parse().expect("cannot parse input");

        if total_cost > 1000.0 {
            total_cost *= 0.9
        }

        println!("{:.2}", total_cost);
    }
}
