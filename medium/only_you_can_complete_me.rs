use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let n: f32 = input.trim().parse().expect("cannot parse input");
    let parts: Vec<&str> = input.trim().splitn(2, '.').collect();
    let (int_part, dec_part): (u32, u32) = (parts[0].parse().unwrap(), parts[1].parse().unwrap());

    if n - int_part as f32 == 0. {
        println!("1");
        return;
    }

    let dec_denominator = 10_u32.pow(parts[1].len() as u32);
    let res = dec_denominator as f32 / gcd(dec_denominator, dec_part) as f32;

    println!("{}", res);
}

fn gcd(x: u32, y: u32) -> u32 {
    if x == 0 {
        return y;
    }

    gcd(y % x, x)
}
