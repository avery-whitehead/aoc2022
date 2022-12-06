struct Instruction {
    amount: usize,
    source: usize,
    dest: usize,
}

fn main() {
    let input = include_str!("./input.txt");
    move_crates(input).expect("Error while moving crates");
}

fn move_crates(input: &str) -> Result<(), String> {
    let split_input = input.split("\n\n");
    let matrix = split_input.clone().nth(0).ok_or("No matrix")?;
    let inst_lines = split_input.clone().nth(1).ok_or("No instructions")?;
    let num_columns = count_columns(matrix).ok_or("No column info")?;
    let stacks = create_stacks(matrix, num_columns);
    let instructions = parse_instructions(inst_lines);
    let complete = run_instructions(stacks, instructions);
    println!("{}", get_top_crates(complete));
    Ok(())
}

fn count_columns(matrix: &str) -> Option<usize> {
    Some(
        matrix
            .split("\n")
            .last()?
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .len(),
    )
}

fn create_stacks(matrix: &str, num_stacks: usize) -> Vec<Vec<String>> {
    let mut output = Vec::from_iter((0..num_stacks).into_iter().map(|_| Vec::new()));
    let lines = matrix.split("\n").collect::<Vec<_>>();
    let lines = lines[0..lines.len() - 1].into_iter();
    lines.for_each(|row| {
        row.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(stack, crt)| {
                if crt.is_alphabetic() {
                    output[stack].push(crt.to_string());
                }
            })
    });
    for stack in output.iter_mut() {
        stack.reverse();
    }
    output
}

fn parse_instructions(lst: &str) -> Vec<Instruction> {
    let lines = lst.split("\n");
    lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let words = l.split(" ");
            let digits = words
                .filter(|w| w.chars().all(char::is_numeric))
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Instruction {
                amount: digits[0],
                source: digits[1],
                dest: digits[2],
            }
        })
        .collect()
}

fn run_instructions(stacks: Vec<Vec<String>>, instructions: Vec<Instruction>) -> Vec<Vec<String>> {
    instructions.iter().fold(stacks, |mut acc, instruction| {
        let src_idx = instruction.source - 1;
        let dest_idx = instruction.dest - 1;
        let mut moving_from = acc[src_idx].clone();
        let moved = moving_from.drain(moving_from.len() - instruction.amount..moving_from.len());
        // To solve part 2, remove the .rev()
        acc[dest_idx].extend(moved);
        acc[src_idx] = moving_from;
        acc
    })
}

fn get_top_crates(stacks: Vec<Vec<String>>) -> String {
    stacks.iter().fold("".to_string(), |acc, stack| {
        let next = stack.last().unwrap();
        acc + next
    })
}
