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

enum SequenceType {
    Supernet,
    Hypernet,
}

fn format_ipv7(line: &str) -> Result<(Vec<String>, Vec<String>)> {
    let mut supernet = vec![String::new()];
    let mut hypernet = vec![String::new()];

    let mut sequence_type = SequenceType::Supernet;

    for line_char in line.chars() {
        if line_char == '[' || line_char == ']' {
            match sequence_type {
                SequenceType::Supernet => {
                    supernet.push(String::new());
                    sequence_type = SequenceType::Hypernet;
                }
                SequenceType::Hypernet => {
                    hypernet.push(String::new());
                    sequence_type = SequenceType::Supernet;
                }
            }
        } else {
            match sequence_type {
                SequenceType::Supernet => &mut supernet,
                SequenceType::Hypernet => &mut hypernet,
            }
            .last_mut()
            .ok_or(anyhow!("invalid input"))?
            .push(line_char);
        }
    }

    Ok((supernet, hypernet))
}

fn has_abba(parts: Vec<String>) -> Result<bool> {
    for part in parts {
        for start_index in 0..part.len().saturating_sub(3) {
            let part_chars: Vec<char> = part[start_index..start_index + 4].chars().collect();
            if part_chars[0] == part_chars[3]
                && part_chars[1] == part_chars[2]
                && part_chars[0] != part_chars[1]
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn part_a(input: &String) -> Result<()> {
    let mut count = 0;

    for line in input.trim().lines() {
        let (supernet, hypernet) = format_ipv7(line)?;

        if has_abba(supernet)? && !has_abba(hypernet)? {
            count += 1;
        }
    }

    println!("Day 7 Part A: {}", count);
    Ok(())
}

fn part_b(input: &String) -> Result<()> {
    // println!("Day 7 Part B: {}", count);
    Ok(())
}
