use std::{
    fs::File,
    io::{BufWriter, Write},
};

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

fn formatted_line(line: &str) -> Result<(Vec<&str>, usize, &str)> {
    let mut split_line = line.split("[");
    let mut name: Vec<&str> = split_line
        .next()
        .ok_or(anyhow!("Invalid input."))?
        .split("-")
        .collect();
    let sector_id: usize = name.last().ok_or(anyhow!("Invalid input."))?.parse()?;
    name.pop();
    let checksum = &split_line.next().ok_or(anyhow!("Invalid input."))?[0..5];
    Ok((name, sector_id, checksum))
}

fn is_real_room(name: &Vec<&str>, checksum: &str) -> Result<bool> {
    let mut max_freq = 0;
    let mut freq = [0; 26];
    for word in name {
        for c in word.chars() {
            let alphabet_index = alphabet_index_from_char(c);
            freq[alphabet_index] += 1;

            if freq[alphabet_index] > max_freq {
                max_freq = freq[alphabet_index];
            }
        }
    }

    let mut checksum_index = 0;

    for current_freq in (1..=max_freq).rev() {
        for alphabet_index in 0..26 {
            if freq[alphabet_index] == current_freq {
                let checksum_char = checksum
                    .chars()
                    .nth(checksum_index)
                    .ok_or(anyhow!("Invalid input"))?;

                if char_from_alphabet_index(alphabet_index) != checksum_char {
                    return Ok(false);
                }

                checksum_index += 1;

                if checksum_index >= 5 {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

fn alphabet_index_from_char(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

fn char_from_alphabet_index(index: usize) -> char {
    (index as u8 + 'a' as u8) as char
}

fn shifted_char(c: char, amount: usize) -> char {
    let alphabet_index = alphabet_index_from_char(c);
    let shifted_index = (alphabet_index + amount) % 26;
    char_from_alphabet_index(shifted_index)
}

fn part_a(input: &String) -> Result<()> {
    let mut sector_id_sum = 0;

    for line in input.trim().lines() {
        let (name, sector_id, checksum) = formatted_line(line)?;

        if is_real_room(&name, checksum)? {
            sector_id_sum += sector_id;
        }
    }

    println!("Day 4 Part A: {}", sector_id_sum);
    Ok(())
}

fn part_b(input: &String) -> Result<()> {
    let mut output_stream = BufWriter::new(File::create("output.txt")?);

    for line in input.trim().lines() {
        let (name, sector_id, checksum) = formatted_line(line)?;

        if !is_real_room(&name, checksum)? {
            continue;
        }

        output_stream.write((sector_id.to_string() + " - ").as_bytes())?;

        for word in name {
            let shifted_bytes: Vec<u8> = word
                .chars()
                .map(|c| shifted_char(c, sector_id) as u8)
                .collect();

            output_stream.write(&shifted_bytes)?;
            output_stream.write(b" ")?;
        }

        output_stream.write(b"\n")?;
    }

    println!("Day 4 Part B: See list of room names in output.txt");
    output_stream.flush()?;

    Ok(())
}
