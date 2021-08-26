use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let q: u8 = input.trim().parse().expect("cannot parse input");
    let mut stack = Stack::new();

    for _ in 0..q {
        input = lines.next().unwrap().expect("cannot read input");

        match &*input {
            i if i.starts_with("PUSH") => {
               let tokens: Vec<&str> = input.split(' ').collect();

               if let Some(n) = tokens.get(1) {
                   let n: u16 = n.parse().expect("cannot parse number");

                   stack.push(n);
                }
            }
            "PEEK" => {
                if let Some(&n) = stack.peek() {
                    println!("{}", n);
                }
            }
            "POP" => {
                stack.pop()
            }
            _ => ()
        }
    }

    println!("{}", stack.items.iter().sum::<u16>());
}

#[derive(Default)]
struct Stack {
    items: Vec<u16>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            ..Default::default()
        }
    }

    fn push(&mut self, n: u16) {
        self.items.push(n)
    }

    fn peek(&self) -> Option<&u16> {
        self.items.last()
    }

    fn pop(&mut self) {
        self.items.pop();
    }
}
