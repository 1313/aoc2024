use std::error::Error;

use crate::utils;

pub fn star1(example: bool) -> Result<(), Box<dyn Error>> {
    let input = String::from(if example {
        include_str!("01.example.txt")
    } else {
        include_str!("01.txt")
    });

    let parsed_numbers: Vec<Vec<_>> = input
        .lines()
        .into_iter()
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

    let res = first
        .iter()
        .zip(second.iter())
        .fold(0, |res, (a, b)| res + a.abs_diff(*b));

    println!("{}", res);

    Ok(())
}

#[test]
fn test_star1() {
    star1(false).ok();
}
