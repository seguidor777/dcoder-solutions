use std::io;

struct Lucas {
    curr: u32,
    next: u32,
}

impl Iterator for Lucas {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_next = self.curr + self.next;

        if self.next == 2 {
            new_next = 1;
        }

        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

// Returns a Lucas sequence generator
fn lucas_sequence() -> Lucas {
    Lucas { curr: 0, next: 2 }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");
    let n: u32 = input.trim().parse().expect("cannot parse input");
    let mut sequence = lucas_sequence().skip_while(|&v| v <= n);

    println!("{}", sequence.next().unwrap())
}
