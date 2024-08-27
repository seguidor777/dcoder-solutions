fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("cannot read N");
    let n:usize = input.trim().parse().expect("cannot parse N");
    let primes = sieve(n);
    let primes_len = primes.len();

    if primes_len == 0 {
        return
    } else if primes_len < 4 {
        println!("2");

        return
    }

    let matrix_len = (primes_len as f32).sqrt() as usize;
    let mut matrix = vec![vec![0; matrix_len]; matrix_len];
    let mut row_start = 0;
    let mut row_end = matrix_len - 1;
    let mut col_start = 0;
    let mut col_end = matrix_len - 1;
    let mut num = 0;
    let spiral_len = matrix_len * matrix_len;

    while num < spiral_len {
        for i in col_start..=col_end {
            matrix[row_start][i] = primes[num];
            num += 1;
        }
        row_start += 1;

        for i in row_start..=row_end {
            matrix[i][col_end] = primes[num];
            num += 1;
        }
        col_end -= 1;

        if row_start <= row_end {
            for i in (col_start..=col_end).rev() {
                matrix[row_end][i] = primes[num];
                num += 1;
            }
            row_end -= 1;
        }

        if col_start <= col_end {
            for i in (row_start..=row_end).rev() {
                matrix[i][col_start] = primes[num];
                num += 1;
            }
            col_start += 1;
        }
    }

    for row in matrix {
        let numbers = row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

        println!("{}", numbers);
    }
}

fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false
    }

    for n in 2..=((limit as f64).sqrt() as usize + 1) {
        if is_prime[n] {
            let mut multiple = n * n;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += n;
            }
        }
    }

    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(i, item)| if item { Some(i) } else { None })
        .collect()
}
