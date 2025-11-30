use crate::Part;
use anyhow::{Result, anyhow};

pub fn run(part: Part, input: &String) -> Result<()> {
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
    let instructions = input.trim().split(", ");

    let mut pos = [0, 0];
    let mut dir = [1, 0];

    for instruction in instructions {
        let (turn, steps_str) = instruction.split_at(1);
        let steps: i32 = steps_str.parse()?;

        dir = match turn {
            "R" => [dir[1], -dir[0]],
            "L" => [-dir[1], dir[0]],
            _ => return Err(anyhow!("invalid input")),
        };

        for i in 0..2 {
            pos[i] += dir[i] * steps;
        }
    }

    println!("Day 1 Part A: {}", pos[0].abs() + pos[1].abs());
    Ok(())
}

fn part_b(input: &String) -> Result<()> {
    let instructions = input.trim().split(", ");

    let mut pos: [i32; 2] = [0, 0];
    let mut dir = [1, 0];

    let mut past_locations = vec![[0, 0]];

    'main_loop: for instruction in instructions {
        let (turn, steps_str) = instruction.split_at(1);
        let steps: i32 = steps_str.parse()?;

        dir = match turn {
            "R" => [dir[1], -dir[0]],
            "L" => [-dir[1], dir[0]],
            _ => return Err(anyhow!("invalid input")),
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

    println!("Day 1 Part B: {}", pos[0].abs() + pos[1].abs());
    Ok(())
}
