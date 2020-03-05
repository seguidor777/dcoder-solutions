use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("cannot read t");

    let t: i32 = input.trim().parse().expect("cannot parse t");

    for _ in 0..t {
        input = String::new();
        io::stdin().read_line(&mut input).expect("cannot read a");

        let a: i32 = input.trim().parse().expect("cannot parse a");

        if a & 1 == 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
