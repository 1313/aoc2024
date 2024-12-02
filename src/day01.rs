use std::{error::Error, ops::Mul};

use crate::utils;

pub fn star1(example: bool) -> Result<(), Box<dyn Error>> {
    let input = get_input(example);

    let (first, second) = get_two_sorted_lists(input);

    let combined = first.iter().zip(second.iter());

    let sum_of_diff = combined.fold(0, |res, (a, b)| res + a.abs_diff(*b));
    let num_of_first_in_second = first.iter().fold(0, |res, a| {
        res + a.mul(second.iter().filter(|b| *b == a).count() as u32)
    });

    println!("First: {sum_of_diff}");
    println!("Second: {num_of_first_in_second}");

    Ok(())
}

fn get_two_sorted_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let parsed_numbers: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().expect("work"))
                .collect()
        })
        .collect();

    let mut transposed = utils::transpose(parsed_numbers);
    let mut first = transposed.remove(0);
    first.sort();
    let mut second = transposed.remove(0);
    second.sort();
    (first, second)
}

fn get_input(example: bool) -> &'static str {
    if example {
        include_str!("01.example.txt")
    } else {
        include_str!("01.txt")
    }
}

#[test]
fn test_star1() {
    star1(false).ok();
}
