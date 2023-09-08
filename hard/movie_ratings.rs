use std::io::{self, BufRead};

#[derive(PartialEq, PartialOrd)]
struct Movie {
    name: String,
    rating: f32,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let input = lines.next().unwrap().expect("cannot read input");
    let mut params = input.split_whitespace();
    let n: usize = params.next().unwrap().parse().expect("cannot parse n");
    let order: i8 = params.next().unwrap().parse().expect("cannot parse order");

    let mut movies: Vec<Movie> = lines.take(n).map(|line| {
        let input = line.expect("cannot read input");
        let params: Vec<&str> = input.split_whitespace().collect();
        let name = params[0..params.len() - 1].join(" ");
        let rating: f32 = params[params.len() - 1]
            .parse()
            .expect("cannot parse rating");

        Movie { name, rating }
    }).collect();

    if order == 1 {
        movies.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
    } else {
        movies.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());
    }

    for movie in movies.iter() {
        println!("{} {:.1}", movie.name, movie.rating);
    }
}
