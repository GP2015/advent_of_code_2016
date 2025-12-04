use crate::common::{alphabet_index_from_char, char_from_alphabet_index};

enum BoundType {
    Min,
    Max,
}

fn get_frequency_table(input: &String) -> Vec<Vec<usize>> {
    let line_length = input.lines().next().unwrap().len();

    let mut table = vec![vec![0; 26]; line_length];

    for line in input.trim().lines() {
        for line_index in 0..line.len() {
            let c = line.chars().nth(line_index).unwrap();

            let alphabet_index = alphabet_index_from_char(c);
            table[line_index][alphabet_index] += 1;
        }
    }

    table
}

fn general(bound_type: BoundType, input: &String) {
    let table = get_frequency_table(input);

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
}

pub fn part_a(input: &String) {
    print!("Day 6 Part A: ");
    general(BoundType::Max, input);
}

pub fn part_b(input: &String) {
    print!("Day 6 Part B: ");
    general(BoundType::Min, input);
}
