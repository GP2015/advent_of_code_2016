use std::fs;

fn main() {
    let raw_data =
        fs::read_to_string("input.txt").expect("Should have been able to read the file.");

    let instructions = raw_data.trim().split(", ");

    let mut pos = [0, 0];
    let mut dir = [1, 0];

    for instruction in instructions {
        let (turn, steps_str) = instruction.split_at(1);

        match turn {
            "R" => dir = [dir[1], -dir[0]],
            "L" => dir = [-dir[1], dir[0]],
            _ => (),
        }

        let steps: i32 = steps_str.parse().unwrap();

        for i in 0..2 {
            pos[i] += dir[i] * steps;
        }
    }

    println!("Answer: {}", pos[0].abs() + pos[1].abs());
}
