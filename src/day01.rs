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

fn format_instruction(instruction: &str) -> Result<(&str, usize)> {
    let (turn, steps_str) = instruction
        .split_at_checked(1)
        .ok_or(anyhow!("failed to split up input instruction"))?;
    let steps: usize = steps_str.parse()?;
    Ok((turn, steps))
}

fn update_dir(dir: &mut [isize; 2], turn: &str) -> Result<()> {
    *dir = match turn {
        "R" => [dir[1], -dir[0]],
        "L" => [-dir[1], dir[0]],
        _ => return Err(anyhow!("invalid direction in input")),
    };
    Ok(())
}

fn part_a(input: &String) -> Result<()> {
    let mut pos: [isize; 2] = [0, 0];
    let mut dir: [isize; 2] = [1, 0];

    for instruction in input.trim().split(", ") {
        let (turn, steps) = format_instruction(instruction)?;
        update_dir(&mut dir, turn)?;

        for i in 0..2 {
            pos[i] += dir[i] * steps as isize;
        }
    }

    println!("Day 1 Part A: {}", pos[0].abs() + pos[1].abs());
    Ok(())
}

fn part_b(input: &String) -> Result<()> {
    let mut pos: [isize; 2] = [0, 0];
    let mut dir: [isize; 2] = [1, 0];

    let mut past_locations = vec![[0, 0]];

    'main_loop: for instruction in input.trim().split(", ") {
        let (turn, steps) = format_instruction(instruction)?;
        update_dir(&mut dir, turn)?;

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
