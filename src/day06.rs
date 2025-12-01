use crate::Part;
use crate::common::{alphabet_index_from_char, char_from_alphabet_index};
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
    print!("Day 6 Part A: ");
    general(BoundType::Max, input)?;
    Ok(())
}

fn part_b(input: &String) -> Result<()> {
    print!("Day 6 Part B: ");
    general(BoundType::Min, input)?;
    Ok(())
}

enum BoundType {
    Min,
    Max,
}

fn get_frequency_table(input: &String) -> Result<Vec<Vec<usize>>> {
    let line_length = input.lines().next().ok_or(anyhow!("invalid input"))?.len();

    let mut table = vec![vec![0; 26]; line_length];

    for line in input.trim().lines() {
        for line_index in 0..line.len() {
            let c = line
                .chars()
                .nth(line_index)
                .ok_or(anyhow!("invalid input"))?;

            let alphabet_index = alphabet_index_from_char(c);
            table[line_index][alphabet_index] += 1;
        }
    }

    Ok(table)
}

fn general(bound_type: BoundType, input: &String) -> Result<()> {
    let table = get_frequency_table(input)?;

    let bound: Vec<usize> = table
        .iter()
        .filter_map(|row| {
            match bound_type {
                BoundType::Min => row.iter().min(),
                BoundType::Max => row.iter().max(),
            }
            .copied()
        })
        .collect();

    let mut message = String::new();

    for letter_index in 0..bound.len() {
        for alphabet_index in 0..26 {
            if table[letter_index][alphabet_index] == bound[letter_index] {
                let letter = char_from_alphabet_index(alphabet_index);
                message.push(letter);
                break;
            }
        }
    }

    println!("{}", message);
    Ok(())
}
