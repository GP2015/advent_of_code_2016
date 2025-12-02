use anyhow::{Result, anyhow};

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

fn has_an_abba(parts: &Vec<String>) -> bool {
    for part in parts {
        for start_index in 0..part.len().saturating_sub(3) {
            let part_chars: Vec<char> = part[start_index..start_index + 4].chars().collect();
            if part_chars[0] == part_chars[3]
                && part_chars[1] == part_chars[2]
                && part_chars[0] != part_chars[1]
            {
                return true;
            }
        }
    }

    false
}

fn get_aba_list(parts: &Vec<String>) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for part in parts {
        for start_index in 0..part.len().saturating_sub(2) {
            let part_chars: Vec<char> = part[start_index..start_index + 3].chars().collect();
            if part_chars[0] == part_chars[2] && part_chars[0] != part_chars[1] {
                list.push(part_chars.iter().collect());
            }
        }
    }

    list
}

fn bab_from_aba(aba: &String) -> String {
    let aba_chars: Vec<char> = aba.chars().collect();

    let mut bab = String::new();
    bab.push(aba_chars[1]);
    bab.push(aba_chars[0]);
    bab.push(aba_chars[1]);

    bab
}

fn has_bab(bab: &String, parts: &Vec<String>) -> bool {
    for part in parts {
        for start_index in 0..part.len().saturating_sub(2) {
            let part_chars: String = part[start_index..start_index + 3].chars().collect();
            if part_chars == *bab {
                return true;
            }
        }
    }

    false
}

pub fn part_a(input: &String) -> Result<()> {
    let mut count = 0;

    for line in input.trim().lines() {
        let (supernet, hypernet) = format_ipv7(line)?;

        if has_an_abba(&supernet) && !has_an_abba(&hypernet) {
            count += 1;
        }
    }

    println!("Day 7 Part A: {}", count);
    Ok(())
}

pub fn part_b(input: &String) -> Result<()> {
    let mut count = 0;

    for line in input.trim().lines() {
        let (supernet, hypernet) = format_ipv7(line)?;

        for aba in get_aba_list(&supernet) {
            let bab = bab_from_aba(&aba);

            if has_bab(&bab, &hypernet) {
                count += 1;
                break;
            }
        }
    }

    println!("Day 7 Part B: {}", count);
    Ok(())
}
