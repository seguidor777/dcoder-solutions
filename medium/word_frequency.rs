use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("cannot read input");

    let words_input: Vec<&str> = input.split_whitespace().collect();
    let mut words_counter: HashMap<&str, u32> = HashMap::new();
    let mut sortable: Vec<&str> = Vec::new();

    for word in words_input.into_iter() {
        if let Some(count) = words_counter.get_mut(word) {
            *count += 1;
            continue;
        }

        words_counter.insert(word, 1);
        sortable.push(word);
    }

    sortable.sort_by(|a, b| words_counter[b].cmp(&words_counter[a]));
    println!("{}", sortable.join(" "));
}
