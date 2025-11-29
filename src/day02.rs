use crate::Part;
use anyhow::{Result, anyhow};

pub fn run_day_2(part: Part, input: &String) -> Result<()> {
    match part {
        Part::A => part_a(input)?,
        Part::B => part_b(input)?,
        Part::Both => {
            part_a(input)?;
            part_b(input)?;
        }
    }

    Ok(())
}

fn part_a(input: &String) -> Result<()> {
    print!("Day 2 Part A: ");
    general(Keypad::type_a(), input)?;
    Ok(())
}

fn part_b(input: &String) -> Result<()> {
    print!("Day 2 Part B: ");
    general(Keypad::type_b(), input)?;
    Ok(())
}

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn get_shifted(&self, dx: isize, dy: isize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const NOL: &str = "";

macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        vec![ $( String::from($x) ),* ]
    };
}

struct Keypad {
    labels: Vec<Vec<String>>,
    position: Position,
}

impl Keypad {
    fn type_a() -> Self {
        Self {
            labels: vec![
                string_vec!("1", "2", "3"),
                string_vec!("4", "5", "6"),
                string_vec!("7", "8", "9"),
            ],
            position: Position::new(1, 1),
        }
    }

    fn type_b() -> Self {
        Self {
            labels: vec![
                string_vec!(NOL, NOL, "1", NOL, NOL),
                string_vec!(NOL, "2", "3", "4", NOL),
                string_vec!("5", "6", "7", "8", "9"),
                string_vec!(NOL, "A", "B", "C", NOL),
                string_vec!(NOL, NOL, "D", NOL, NOL),
            ],
            position: Position::new(2, 2),
        }
    }

    fn shift(&mut self, direction: Direction) {
        let new_pos = match direction {
            Direction::Left => self.position.get_shifted(-1, 0),
            Direction::Right => self.position.get_shifted(1, 0),
            Direction::Up => self.position.get_shifted(0, -1),
            Direction::Down => self.position.get_shifted(0, 1),
        };

        if new_pos.y < 0 || new_pos.y >= self.labels.len() as isize {
            return;
        }

        let row = &self.labels[new_pos.y as usize];

        if new_pos.x < 0 || new_pos.x >= row.len() as isize {
            return;
        }

        let char = &row[new_pos.x as usize];

        if char.as_str() == NOL {
            return;
        }

        self.position = new_pos;
    }

    fn get_label(&self) -> String {
        self.labels[self.position.y as usize][self.position.x as usize].clone()
    }
}

fn general(mut keypad: Keypad, input: &String) -> Result<()> {
    let mut code = String::new();

    for instruction in input.lines() {
        for c in instruction.chars() {
            match c {
                'L' => keypad.shift(Direction::Left),
                'R' => keypad.shift(Direction::Right),
                'U' => keypad.shift(Direction::Up),
                'D' => keypad.shift(Direction::Down),
                _ => return Err(anyhow!("Invalid input: {}.", c)),
            }
        }

        code += keypad.get_label().as_str();
    }

    println!("{}", code);
    Ok(())
}
