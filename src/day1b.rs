use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let raw_data = fs::read_to_string("input.txt")?;

    let instructions = raw_data.trim().split(", ");

    let mut pos: [i32; 2] = [0, 0];
    let mut dir = [1, 0];

    let mut past_locations = vec![[0, 0]];

    'main_loop: for instruction in instructions {
        let (turn, steps_str) = instruction.split_at(1);
        let steps: i32 = steps_str.parse().unwrap();

        dir = match turn {
            "R" => [dir[1], -dir[0]],
            "L" => [-dir[1], dir[0]],
            _ => panic!("Invalid input."),
        };

        for _ in 0..steps {
            for i in 0..2 {
                pos[i] += dir[i];
            }

            if past_locations.contains(&pos) {
                break 'main_loop;
            }

            past_locations.push(pos);
        }
    }

    println!("Answer: {}", pos[0].abs() + pos[1].abs());
    Ok(())
}
