fn main() {
    let input = include_str!("../input.txt");

    let initial_state = input
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    run(initial_state, 80, 256);
}

fn run(initial: Vec<u8>, first: u32, second: u32) {
    let mut school = Lanternfish::new(initial);

    for i in 1..=second {
        school.run();
        if i == first {
            school.calculate_population();
        }
    }

    school.calculate_population();
}

struct Lanternfish {
    child: [u64; 9],
}

impl Lanternfish {
    fn new(from: Vec<u8>) -> Lanternfish {
        let mut child: [u64; 9] = [0; 9];

        for &v in from.iter() {
            child[v as usize] += 1
        }

        Lanternfish { child }
    }

    fn calculate_population(&self) {
        let population: u64 = self.child.iter().sum();

        println!("{}", population);
    }

    fn run(&mut self) {
        let mut new_values = [0; 9];

        new_values[8] = self.child[0];
        new_values[..8].clone_from_slice(&self.child[1..9]);

        new_values[6] += new_values[8];
        self.child = new_values;
    }
}
