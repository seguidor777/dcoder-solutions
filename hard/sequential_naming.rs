use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().expect("cannot read line");
    let n: usize = line.parse().expect("cannot parse n");
    line = lines.next().unwrap().expect("cannot read line");
    let filenames: Vec<String> = line.split_whitespace().take(n).map(String::from).collect();
    let mut set: HashSet<String> = HashSet::new();
    let mut new_filenames = Vec::new();

    for mut filename in filenames {
        if set.contains(&filename) {
            let mut number = 1;
            let mut tmp_filename = format!("{}({})", filename, number);

            // Find next number
            while set.contains(&tmp_filename) {
                number += 1;
                tmp_filename = format!("{}({})", filename, number);
            }

            filename = tmp_filename;
        }

        set.insert(filename.clone());
        new_filenames.push(filename);
    }

    println!("{}", new_filenames.join(" "));
}
