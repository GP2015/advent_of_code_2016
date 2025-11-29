use crate::Part;
use anyhow::Result;

pub fn run_day_2(part: Part, input: &String) -> Result<()> {
    match part {
        Part::A => general(get_keypad(KeypadType::A), input)?,
        Part::B => general(get_keypad(KeypadType::B), input)?,
        Part::Both => {
            general(get_keypad(KeypadType::A), input)?;
            general(get_keypad(KeypadType::B), input)?;
        }
    }

    Ok(())
}

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    fn get_shifted(&self, dx: isize, dy: isize) -> Self {
        Position {
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

struct Keypad {
    labels: Vec<Vec<String>>,
    selected: Position,
}

impl Keypad {
    fn new(labels: Vec<Vec<String>>, start_position: Position) -> Self {
        Keypad {
            labels,
            selected: start_position,
        }
    }

    fn shift(&mut self, direction: Direction) {
        let new_pos = match direction {
            Direction::Left => self.selected.get_shifted(-1, 0),
            Direction::Right => self.selected.get_shifted(1, 0),
            Direction::Up => self.selected.get_shifted(0, -1),
            Direction::Down => self.selected.get_shifted(0, 1),
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

        self.selected = new_pos;
    }
}

enum KeypadType {
    A,
    B,
}

macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        vec![ $( String::from($x) ),* ]
    };
}

fn get_keypad(keypad_type: KeypadType) -> Keypad {
    let labels = match keypad_type {
        KeypadType::A => vec![
            string_vec!("1", "2", "3"),
            string_vec!("4", "5", "6"),
            string_vec!("7", "8", "9"),
        ],
        KeypadType::B => vec![
            string_vec!(NOL, NOL, "1", NOL, NOL),
            string_vec!(NOL, "2", "3", "4", NOL),
            string_vec!("5", "6", "7", "8", "9"),
            string_vec!(NOL, "A", "B", "C", NOL),
            string_vec!(NOL, NOL, "D", NOL, NOL),
        ],
    };

    let position = match keypad_type {
        KeypadType::A => Position::new(1, 1),
        KeypadType::B => Position::new(2, 2),
    };

    Keypad::new(labels, position)
}

fn general(keypad: Keypad, input: &String) -> Result<()> {
    let instructions = input.trim().split("\n");

    for instruction in instructions {
        for char_index in 0..instruction.len() {}
    }

    // println!("Answer: {}", pos[0].abs() + pos[1].abs());
    Ok(())
}
