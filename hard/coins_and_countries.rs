use std::io::{BufRead, self};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().expect("cannot read input");
    let t: usize = input.parse().expect("cannot parse t");

    for _ in 0..t {
        input = lines.next().unwrap().expect("cannot read input");
        let mut params = input.split_whitespace();
        let n: u32 = params.next().unwrap().parse().expect("cannot parse n");
        let r: u32 = params.next().unwrap().parse().expect("cannot parse r");

        // Calculate the number of ways using combinations (n-1 choose r-1)
        let num_ways = comb(n - 1, r - 1);
        
        println!("{}", num_ways);
    }
}

// Function to calculate combinations (n choose k)
// Credits of this awesome function to ChartGPT 3.5
fn comb(n: u32, k: u32) -> u32 {
    let mut result = 1;
    let mut denom = 1;
    
    for i in 0..k {
        result *= n - i;
        denom *= i + 1;
    }
    
    result / denom
}
