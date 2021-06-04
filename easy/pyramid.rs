fn main() {
    let mut stars = 1;

    for spaces in (0..=4).rev() {
        println!("{:1$}{2}", "", spaces, "*".repeat(stars));
        stars += 2;
    }
}
