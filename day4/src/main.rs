fn main() {
    let input = include_str!("../input.txt");

    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {
    let mut lines = input.lines();

    let numbers = parse_input_numbers(&mut lines);
    lines.next();

    let mut boards: Vec<_> = Vec::new();

    while let Some(board) = Board::new(&mut lines) {
        boards.push(board);
    }

    for number in numbers {
        for board in boards.iter_mut() {
            if board.check(number) == BoardNumberCheck::Found {
                let sum = board.calculate_sum_unmarked();
                println!("{} {} {}", sum, number, sum * number);
                return;
            }
        }
    }
}

fn part_2(input: &str) {
    let mut lines = input.lines();

    let numbers = parse_input_numbers(&mut lines);
    lines.next();

    let mut boards: Vec<_> = Vec::new();

    while let Some(board) = Board::new(&mut lines) {
        boards.push(board);
    }

    let mut winner: Option<(Board, u32)> = None;

    for number in numbers {
        for board in &mut boards.iter_mut() {
            if board.check(number) == BoardNumberCheck::Found {
                winner = Some((*board, number));
            }
        }
    }

    let winner = winner.unwrap();
    let sum = winner.0.calculate_sum_unmarked();
    let number = winner.1;
    println!("{} {} {}", sum, number, sum * number);
}

fn parse_input_numbers(input: &mut std::str::Lines) -> Vec<u32> {
    input
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, Copy, Clone)]
struct Board {
    won: bool,
    grid: [[BoardNumber; 5]; 5],
}

impl Board {
    fn check(&mut self, value: u32) -> BoardNumberCheck {
        if self.won {
            return BoardNumberCheck::Previous;
        }

        let mut values: [[bool; 5]; 5] = [[false; 5]; 5];

        for i in 0..5 {
            for j in 0..5 {
                match self.grid[i][j].check(value) {
                    BoardNumberCheck::Found | BoardNumberCheck::Previous => values[i][j] = true,
                    _ => continue,
                }
            }
        }

        let horizontal_lines = values
            .iter()
            .filter(|l| l.iter().filter(|&&v| v).count() == 5)
            .count();

        if horizontal_lines > 0 {
            self.won = true;
            return BoardNumberCheck::Found;
        }

        let vertical_lines = [0, 1, 2, 3, 4]
            .map(|i| [0, 1, 2, 3, 4].map(|j| values[i][j]))
            .iter()
            .filter(|l| l.iter().filter(|&&v| v).count() == 5)
            .count();

        if vertical_lines > 0 {
            self.won = true;
            return BoardNumberCheck::Found;
        }

        BoardNumberCheck::NotFound
    }

    fn new(input: &mut std::str::Lines) -> Option<Board> {
        let lines: Vec<[BoardNumber; 5]> = input
            .take(5)
            .map(|n| {
                let n: Vec<u32> = n
                    .split(' ')
                    .filter(|s| s.chars().count() != 0)
                    .take(5)
                    .map(|s| s.parse().unwrap())
                    .collect();
                let n: [u32; 5] = n.try_into().unwrap();
                let n: [BoardNumber; 5] = n.map(|n| BoardNumber {
                    number: n,
                    marked: false,
                });
                n
            })
            .collect();

        if lines.len() != 5 {
            return None;
        }

        let lines: [[BoardNumber; 5]; 5] = lines.try_into().unwrap();

        input.next(); //advance empty line

        Some(Board {
            won: false,
            grid: lines,
        })
    }

    fn calculate_sum_unmarked(&self) -> u32 {
        self.grid
            .into_iter()
            .map(|l| -> u32 { l.into_iter().filter(|v| !v.marked).map(|v| v.number).sum() })
            .sum()
    }
}

#[derive(Debug, Copy, Clone)]
struct BoardNumber {
    number: u32,
    marked: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum BoardNumberCheck {
    Previous,
    Found,
    NotFound,
}

impl BoardNumber {
    fn check(&mut self, value: u32) -> BoardNumberCheck {
        if self.number == value {
            self.marked = true;
            BoardNumberCheck::Found
        } else if self.marked {
            BoardNumberCheck::Previous
        } else {
            BoardNumberCheck::NotFound
        }
    }
}
