use anyhow::Result;
use std::fs;

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

const KEYPAD_NULL_LABEL: &str = "";

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

        if char.as_str() == KEYPAD_NULL_LABEL {
            return;
        }

        self.selected = new_pos;
    }
}

fn main() -> Result<()> {
    let raw_data = fs::read_to_string("input.txt")?;

    let instructions = raw_data.trim().split("\n");

    let labels = vec![
        vec![String::from("1"), String::from("1"), String::from("1")],
        vec![String::from("2"), String::from("3"), String::from("4")],
        vec![String::from("5"), String::from("6"), String::from("7")],
    ];

    let keypad = Keypad::new(labels, Position::new(1, 1));

    for instruction in instructions {
        for char_index in 0..instruction.len() {}
    }

    // println!("Answer: {}", pos[0].abs() + pos[1].abs());
    Ok(())
}
