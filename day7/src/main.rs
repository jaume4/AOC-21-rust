fn main() {
    let input = include_str!("../input.txt");

    let initial_state = input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    part1(&initial_state);
    part2(&initial_state);
}

fn part1(input: &[u32]) {
    let max = *input.iter().max().unwrap() as i32;
    let mut costs: Vec<i32> = Vec::new();

    let (mut min_cost, mut min_index): (i32, usize) = (0, 0);

    for p in 0..max {
        let cost: i32 = input.iter().map(|&crab| (crab as i32 - p).abs()).sum();
        if p == 0 || cost < min_cost {
            min_cost = cost;
            min_index = p as usize;
        }
        costs.push(cost);
    }
    println!(" {} {}", min_cost, min_index);
}

fn part2(input: &[u32]) {
    let max = *input.iter().max().unwrap() as i32;
    let mut costs: Vec<i32> = Vec::new();

    let (mut min_cost, mut min_index): (i32, usize) = (0, 0);

    for p in 0..max {
        let cost: i32 = input
            .iter()
            .map(|&crab| (calculate_fuel_cost(crab as i32, p)).abs())
            .sum();
        if p == 0 || cost < min_cost {
            min_cost = cost;
            min_index = p as usize;
        }
        costs.push(cost);
    }
    println!(" {} {}", min_cost, min_index);
}

fn calculate_fuel_cost(from: i32, to: i32) -> i32 {
    let (min, max): (i32, i32);

    if from > to {
        min = to;
        max = from;
    } else {
        min = from;
        max = to;
    }

    (min..=max).map(|d| (d - min).abs()).sum()
}
