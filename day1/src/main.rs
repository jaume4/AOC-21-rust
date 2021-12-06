use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;

    let values: Vec<u32> = input.split('\n').filter_map(|s| s.parse().ok()).collect();
    let len = values.len();

    println!("Part 1: {}", part_1(&values, len));
    println!("Part 2: {}", part_2(&values, len));
    Ok(())
}

fn part_1(values: &[u32], len: usize) -> usize {
    values
        .iter()
        .zip(&values[1..len])
        .map(|(x, y)| x < y)
        .filter(|&x| x)
        .count()
}

fn part_2(values: &[u32], len: usize) -> usize {
    let values: Vec<u32> = values
        .iter()
        .zip(&values[1..len])
        .zip(&values[2..len])
        .map(|((x, y), z)| x + y + z)
        .collect();

    values
        .iter()
        .zip(&values[1..values.len()])
        .map(|(x, y)| x < y)
        .filter(|&x| x)
        .count()
}

fn read_input() -> Result<String, io::Error> {
    std::fs::read_to_string("input.txt")
}
