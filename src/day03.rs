use anyhow::Result;

fn is_triangle(sides: &[usize]) -> bool {
    !(sides[0] + sides[1] <= sides[2]
        || sides[0] + sides[2] <= sides[1]
        || sides[1] + sides[2] <= sides[0])
}

fn format_input(input: &String) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn part_a(input: &String) -> Result<()> {
    let input: Vec<Vec<usize>> = format_input(input);
    let mut count = 0;

    for tri in input {
        if is_triangle(tri.as_slice()) {
            count += 1;
        }
    }

    println!("Day 3 Part A: {}", count);
    Ok(())
}

pub fn part_b(input: &String) -> Result<()> {
    let input: Vec<Vec<usize>> = format_input(input);
    let mut count = 0;

    for tri_num in 0..input.len() {
        let tri = [
            input[tri_num - (tri_num % 3)][tri_num % 3],
            input[tri_num - (tri_num % 3) + 1][tri_num % 3],
            input[tri_num - (tri_num % 3) + 2][tri_num % 3],
        ];

        if is_triangle(&tri) {
            count += 1;
        }
    }

    println!("Day 3 Part B: {}", count);
    Ok(())
}
