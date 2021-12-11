fn main() {
    let input = include_str!("../input.txt");

    calculate(input, false);
    calculate(input, true);
}

fn calculate(input: &str, calculate_diagonals: bool) {
    let lines = parse_input(&mut input.lines());

    let grid_max = lines
        .iter()
        .flat_map(|l| -> [u32; 4] { [l.origin.x, l.origin.y, l.end.x, l.end.y] })
        .max()
        .unwrap();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; grid_max as usize + 1]; grid_max as usize + 1];

    for line in lines {
        let is_horizontal_or_vertical = line.origin.x == line.end.x || line.origin.y == line.end.y;

        if !calculate_diagonals && !is_horizontal_or_vertical {
            continue;
        }

        let min_y: u32;
        let max_y: u32;

        let min_x: u32;
        let max_x: u32;

        if line.origin.x > line.end.x {
            max_x = line.origin.x;
            min_x = line.end.x;
        } else {
            max_x = line.end.x;
            min_x = line.origin.x;
        }

        if line.origin.y > line.end.y {
            max_y = line.origin.y;
            min_y = line.end.y;
        } else {
            max_y = line.end.y;
            min_y = line.origin.y;
        }

        if is_horizontal_or_vertical {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid[x as usize][y as usize] += 1;
                }
            }
        } else {
            let x_values: Vec<_>;
            let y_values: Vec<_>;

            if line.origin.x != min_x {
                x_values = (min_x..=max_x).rev().collect();
            } else {
                x_values = (min_x..=max_x).collect();
            }

            if line.origin.y != min_y {
                y_values = (min_y..=max_y).rev().collect();
            } else {
                y_values = (min_y..=max_y).collect();
            }

            for (x, y) in (x_values).iter().zip(y_values) {
                grid[*x as usize][y as usize] += 1;
            }
        }
    }

    use std::collections::HashMap;

    let mut scores: HashMap<u32, u32> = HashMap::new();

    for line in grid {
        for point in line {
            if let Some(value) = scores.get_mut(&point) {
                *value += 1;
            } else {
                scores.insert(point, 1);
            }
        }
    }

    println!("{:?}", scores);

    let sum: u32 = scores
        .iter()
        .filter(|(&k, _)| k >= 2)
        .map(|(_, &v)| v)
        .sum();

    println!("{}", sum);
}

fn part_2(input: &str) {}

fn parse_input(input: &mut std::str::Lines) -> Vec<Line> {
    input.map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Line {
    origin: Point,
    end: Point,
}

impl std::str::FromStr for Line {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: [Point; 2] = s
            .split(" -> ")
            .map(|s| s.parse::<Point>().unwrap())
            .collect::<Vec<Point>>()
            .try_into()
            .unwrap();

        Ok(Line {
            origin: values[0],
            end: values[1],
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl std::str::FromStr for Point {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: [u32; 2] = s
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        Ok(Point {
            x: values[0],
            y: values[1],
        })
    }
}
