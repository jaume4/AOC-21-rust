use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;

    let values: Vec<_> = input
        .split('\n')
        .map(|s| -> Move {
            let values: Vec<_> = s.split(' ').collect();
            let number: u32 = values[1].parse().unwrap();
            match values[0] {
                "forward" => Move::Forward(number),
                "up" => Move::Up(number),
                "down" => Move::Down(number),
                _ => panic!(),
            }
        })
        .collect();

    part_1(&values);
    part_2(&values);

    Ok(())
}

fn part_1(values: &[Move]) {
    let mut position = Position { x: 0, y: 0 };

    for value in values {
        match value {
            Move::Forward(x) => position.x += x,
            Move::Down(y) => position.y += y,
            Move::Up(y) => position.y -= y,
        }
    }

    println!("{:?}", position);
    println!("{}", position.x * position.y);
}

fn part_2(values: &[Move]) {
    let mut full_position = FullPosition { x: 0, y: 0, aim: 0 };

    for value in values {
        match value {
            Move::Forward(x) => {
                full_position.x += x;
                full_position.y += x * full_position.aim;
            }
            Move::Down(aim) => full_position.aim += aim,
            Move::Up(aim) => full_position.aim -= aim,
        }
    }

    println!("{:?}", full_position);
    println!("{}", full_position.x * full_position.y);
}

fn read_input() -> Result<String, io::Error> {
    std::fs::read_to_string("input.txt")
}

enum Move {
    Forward(u32),
    Up(u32),
    Down(u32),
}

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct FullPosition {
    x: u32,
    y: u32,
    aim: u32,
}
