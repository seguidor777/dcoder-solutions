use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let input_str = input.trim();
    let mut queue = input_str.chars().collect::<VecDeque<_>>();
    let n: u16 = input_str.parse().expect("cannot parse input");
    let mut count = 0;

    permute(&mut Vec::new(), &mut queue, n, &mut count);
    println!("{}", count);
}

fn permute(used: &mut Vec<char>, unused: &mut VecDeque<char>, n: u16, count: &mut u16) {
    if unused.is_empty() {
        let used_val: u16 = used.iter().collect::<String>().parse().unwrap();

        if used_val > n {
            *count += 1;
        }
    } else {
        for _ in 0..unused.len() {
            used.push(unused.pop_front().unwrap());
            permute(used, unused, n, count);
            unused.push_back(used.pop().unwrap());
        }
    }
}
