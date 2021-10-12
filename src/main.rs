use std::{ascii::AsciiExt, iter};

use itertools::Itertools;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

fn permutations() {
    let number_of_digits = 6;

    let set = vec![0, 12, 132, 7, 8, 10, 3, 1, 28, 100];

    let mut numerator = 1;

    for num in 1..set.len() + 1 {
        numerator *= num;
    }

    let mut denominator = 1;

    for num in 1..(set.len() - number_of_digits) + 1 {
        denominator *= num;
    }

    let result = numerator / denominator;

    println!("Number of possiblities: {}", result);

    println!("20 possibilities:");

    for (num_of_permuations, permutation) in set
        .iter()
        .permutations(number_of_digits)
        .unique()
        .enumerate()
    {
        if num_of_permuations >= 20 {
            break;
        }

        println!("{:?}", permutation);
    }
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn gen_license_plate() -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.gen_range(0..25))
        .map(|i| ASCII_LOWER[i])
        .take(3)
        .collect();

    let upper = chars.to_uppercase();

    let numbers = rng.gen_range(1000..9999);

    format!("{}-{}", upper, numbers)
}

fn main() {
    for _num in 0..30 {
        let plate = gen_license_plate();

        println!("{}", plate);
    }
}
