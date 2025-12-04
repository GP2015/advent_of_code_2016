fn format_instruction(instruction: &str) -> (&str, usize) {
    let (turn, steps_str) = instruction.split_at_checked(1).unwrap();
    let steps: usize = steps_str.parse().unwrap();
    (turn, steps)
}

fn update_dir(dir: &mut [isize; 2], turn: &str) {
    *dir = match turn {
        "R" => [dir[1], -dir[0]],
        "L" => [-dir[1], dir[0]],
        _ => panic!(),
    };
}

pub fn part_a(input: &String) {
    let mut pos: [isize; 2] = [0, 0];
    let mut dir: [isize; 2] = [1, 0];

    for instruction in input.trim().split(", ") {
        let (turn, steps) = format_instruction(instruction);
        update_dir(&mut dir, turn);

        for i in 0..2 {
            pos[i] += dir[i] * steps as isize;
        }
    }

    println!("Day 1 Part A: {}", pos[0].abs() + pos[1].abs());
}

pub fn part_b(input: &String) {
    let mut pos: [isize; 2] = [0, 0];
    let mut dir: [isize; 2] = [1, 0];

    let mut past_locations = vec![[0, 0]];

    'main_loop: for instruction in input.trim().split(", ") {
        let (turn, steps) = format_instruction(instruction);
        update_dir(&mut dir, turn);

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
}
