use anyhow::{Result, anyhow};

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

const NOL: char = ' ';

struct Keypad {
    labels: Vec<Vec<char>>,
    position: Position,
}

impl Keypad {
    fn type_a() -> Self {
        Self {
            labels: vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ],
            position: Position::new(1, 1),
        }
    }

    fn type_b() -> Self {
        Self {
            labels: vec![
                vec![NOL, NOL, '1', NOL, NOL],
                vec![NOL, '2', '3', '4', NOL],
                vec!['5', '6', '7', '8', '9'],
                vec![NOL, 'A', 'B', 'C', NOL],
                vec![NOL, NOL, 'D', NOL, NOL],
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

        let char = row[new_pos.x as usize];

        if char == NOL {
            return;
        }

        self.position = new_pos;
    }

    fn get_label(&self) -> char {
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
                _ => return Err(anyhow!("invalid character in input: {}", c)),
            }
        }

        code.push(keypad.get_label());
    }

    println!("{}", code);
    Ok(())
}

pub fn part_a(input: &String) -> Result<()> {
    print!("Day 2 Part A: ");
    general(Keypad::type_a(), input)?;
    Ok(())
}

pub fn part_b(input: &String) -> Result<()> {
    print!("Day 2 Part B: ");
    general(Keypad::type_b(), input)?;
    Ok(())
}
