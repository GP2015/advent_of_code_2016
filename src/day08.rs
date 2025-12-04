enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

fn format_line(line: &str) -> Instruction {
    let words: Vec<&str> = line.split_whitespace().collect();

    let instruction = match words[0] {
        "rect" => {
            let dims: Vec<usize> = words[1].split("x").filter_map(|n| n.parse().ok()).collect();
            Instruction::Rect(dims[0], dims[1])
        }
        "rotate" => {
            let a: usize = words[2][2..].parse().unwrap();
            let b: usize = words[4].parse().unwrap();

            match words[1] {
                "row" => Instruction::RotateRow(a, b),
                "col" => Instruction::RotateCol(a, b),
                _ => panic!(),
            }
        }
        _ => panic!(),
    };

    instruction
}

struct Display {
    pixels: Vec<Vec<bool>>,
}

impl Display {
    fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![false; width]; height],
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for col in 0..width {
                self.pixels[row][col] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, steps: usize) {
        self.pixels[row].rotate_right(steps);
    }

    fn rotate_col(&mut self, col: usize, steps: usize) {
        let mut col_vals: Vec<bool> = self.pixels.iter().map(|v| v[col]).collect();

        col_vals.rotate_right(steps);

        for row in 0..self.pixels.len() {}
    }
}

pub fn part_a(input: &String) {
    let mut display = Display::new(50, 6);

    for line in input.trim().lines() {
        match format_line(line) {
            Instruction::Rect(width, height) => display.rect(width, height),
            Instruction::RotateRow(row, steps) => display.rotate_row(row, steps),
            Instruction::RotateCol(col, steps) => display.rotate_col(col, steps),
        }
    }

    // println!("Day 7 Part A: {}", count);
}

pub fn part_b(input: &String) {
    // println!("Day 7 Part B: {}", count);
}
