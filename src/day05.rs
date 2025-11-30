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
    let trimmed_input = input.trim();

    let mut index = 0;
    let mut password = String::new();

    loop {
        let hash_input = format!("{}{}", trimmed_input, index);
        let digest = format!("{:x}", md5::compute(hash_input));

        if &digest[0..5] == "00000" {
            let c = digest.chars().nth(5).ok_or(anyhow!("invalid input"))?;
            password.push(c);

            if password.len() == 8 {
                break;
            }
        }

        index += 1;
    }

    println!("Day 5 Part A: {}", password);
    Ok(())
}

const EMPTY: char = ' ';

fn part_b(input: &String) -> Result<()> {
    let trimmed_input = input.trim();

    let mut index = 0;
    let mut password = [EMPTY; 8];

    loop {
        let hash_input = format!("{}{}", trimmed_input, index);
        let digest = format!("{:x}", md5::compute(hash_input));

        if &digest[0..5] == "00000" {
            let pos_char = digest.chars().nth(5).ok_or(anyhow!("invalid input"))?;

            let pos = (pos_char as usize).wrapping_sub('0' as usize);
            if pos < 8 {
                if password[pos] == EMPTY {
                    password[pos] = digest.chars().nth(6).ok_or(anyhow!("invalid input"))?;
                }

                let mut password_finished = true;
                for c in password {
                    if c == EMPTY {
                        password_finished = false;
                        break;
                    }
                }

                if password_finished {
                    break;
                }
            }
        }

        index += 1;
    }

    println!("Day 5 Part B: {}", password.iter().collect::<String>());
    Ok(())
}
