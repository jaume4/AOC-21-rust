fn main() {
    let input = include_str!("../input.txt");

    let values: Vec<_> = input
        .split('\n')
        .map(|s| -> u32 { u32::from_str_radix(s, 2).unwrap() })
        .collect();

    part_1(&values);
    part_2(&values);
}

fn part_1(values: &[u32]) {
    let lenght = values.len() as u32;

    let mut counts: [u32; 12] = [0; 12];

    let bool_iter = values.iter().map(|value| -> Vec<_> {
        (0..12)
            .map(|i| -> bool { ((value & (1 << i)) >> i) == 1 })
            .rev()
            .collect()
    });

    for line in bool_iter {
        for (i, value) in line.iter().enumerate() {
            if *value {
                counts[i] += 1;
            }
        }
    }

    let gamma_array: Vec<_> = counts.iter().map(|x| x >= &(lenght / 2)).collect();
    let gamma: u32 = gamma_array
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| if *v { 1 << (i as u32) } else { 0 })
        .sum();

    let epsilon: u32 = gamma_array
        .iter()
        .map(|x| !x)
        .rev()
        .enumerate()
        .map(|(i, v)| if v { 1 << (i as u32) } else { 0 })
        .sum();

    println!("{} {} {}", gamma, epsilon, gamma * epsilon);
}

fn part_2(values: &[u32]) {
    let mut _values: Vec<_> = values.to_vec();

    let mut oxygen_filter = 0;

    while _values.len() > 1 {
        let sum: u32 = _values
            .iter()
            .map(|x| (x & (1 << (11 - oxygen_filter))) >> (11 - oxygen_filter))
            .sum();

        if sum >= (_values.len() as u32 - sum) {
            _values = _values
                .into_iter()
                .filter(|x| (*x & (1 << (11 - oxygen_filter))) >> (11 - oxygen_filter) == 1)
                .collect();
        } else {
            _values = _values
                .into_iter()
                .filter(|x| (*x & (1 << (11 - oxygen_filter))) >> (11 - oxygen_filter) == 0)
                .collect();
        }

        oxygen_filter += 1;
    }

    let oxygen = _values[0];

    let mut _values: Vec<_> = values.to_vec();

    let mut co2_filter = 0;

    while _values.len() > 1 {
        let sum: u32 = _values
            .iter()
            .map(|x| (x & (1 << (11 - co2_filter))) >> (11 - co2_filter))
            .sum();

        if sum < (_values.len() as u32 - sum) {
            _values = _values
                .into_iter()
                .filter(|x| (*x & (1 << (11 - co2_filter))) >> (11 - co2_filter) == 1)
                .collect();
        } else {
            _values = _values
                .into_iter()
                .filter(|x| (*x & (1 << (11 - co2_filter))) >> (11 - co2_filter) == 0)
                .collect();
        }

        co2_filter += 1;
    }

    let co2 = _values[0];

    println!("{} {} {}", oxygen, co2, oxygen * co2);
}
