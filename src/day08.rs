use std::{
    fs::File,
    io::{BufWriter, Write},
    slice::Iter,
};

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
                "column" => Instruction::RotateCol(a, b),
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
        self.pixels
            .iter_mut()
            .zip(col_vals.iter())
            .for_each(|(row, &val)| row[col] = val);
    }

    fn count_lit(&self) -> usize {
        self.pixels.iter().flatten().filter(|&&val| val).count()
    }

    fn rows(&self) -> Iter<'_, Vec<bool>> {
        self.pixels.iter()
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

    println!("Day 8 Part A: {}", display.count_lit());
}

pub fn part_b(input: &String) {
    let mut display = Display::new(50, 6);

    for line in input.trim().lines() {
        match format_line(line) {
            Instruction::Rect(width, height) => display.rect(width, height),
            Instruction::RotateRow(row, steps) => display.rotate_row(row, steps),
            Instruction::RotateCol(col, steps) => display.rotate_col(col, steps),
        }
    }

    let mut output_stream = BufWriter::new(File::create("output.txt").unwrap());

    for row in display.rows() {
        let mut vals: Vec<u8> = row
            .iter()
            .map(|&val| match val {
                true => '#',
                false => '.',
            } as u8)
            .collect();

        vals.push('\n' as u8);

        output_stream.write(&vals).unwrap();
    }

    output_stream.flush().unwrap();

    println!("Day 7 Part B: See the screen output in output.txt");
}
