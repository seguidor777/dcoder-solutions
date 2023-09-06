use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let mut queue = input.trim().chars().collect::<VecDeque<_>>();
    let mut perms = Vec::new();

    permute(&mut Vec::new(), &mut queue, &mut perms);
    perms.sort();
    perms.dedup();
    println!("{}", perms.join(" "));
}

// Taken from https://rosettacode.org/wiki/Permutations#Rust
fn permute(used: &mut Vec<char>, unused: &mut VecDeque<char>, perms: &mut Vec<String>) {
    if unused.is_empty() {
        perms.push(used.iter().collect());
    } else {
        for _ in 0..unused.len() {
            used.push(unused.pop_front().unwrap());
            permute(used, unused, perms);
            unused.push_back(used.pop().unwrap());
        }
    }
}
