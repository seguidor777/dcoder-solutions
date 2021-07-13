fn main() {
    let numbers = (1..=10)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", numbers)
}
